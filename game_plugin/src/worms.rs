use crate::{loading::TextureAssets};
use crate::GameState;
use crate::game::{AppleHP, Level, Scores};
use bevy::prelude::*;
use rand::prelude::random;

pub struct WormsPlugin;

pub struct Worm;
pub struct WormDirection{
    dir: Vec2
}
pub struct Hp(pub f32);

pub struct WormSpeed(f32);

impl Plugin for WormsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            SystemSet::on_update(GameState::Playing)
                .with_system(spawn_worm.system())
                
        )
        .add_system_set(SystemSet::on_update(GameState::Playing)
            .with_system(animate_worm.system())
            .with_system(move_worm.system())
            .with_system(check_death.system())
        )
        .add_system_set(SystemSet::on_exit(GameState::Playing).with_system(remove_worm.system()));
    }
}

fn spawn_worm(
    mut commands: Commands,
    textures: Res<TextureAssets>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut level: ResMut<Level>
) {

    if level.need_worms > 0{
        let texture = textures.texture_worm.clone().into();
        let texture_atlas = TextureAtlas::from_grid(texture, Vec2::new(100.0, 30.0), 39, 1);
        let texture_atlas_handle = texture_atlases.add(texture_atlas);

        // Get random position and rotation
        let rot = Vec2::new(random::<f32>()-0.5, random::<f32>()-0.5).normalize();
        // let rot = Vec2::new(3.0, 4.0).normalize();
        let dist = 300.0 + random::<f32>() * 300.0;
        let pos = Vec3::new(-rot.x * dist, 
                                 -rot.y * dist, 2.0);
        let mut tr = Transform::from_translation(pos);

        tr.rotation = Quat::from_rotation_z(Vec2::new(1.,0.).angle_between(rot));

        commands
            .spawn_bundle(SpriteSheetBundle {
                texture_atlas: texture_atlas_handle,
                transform: tr,

                ..Default::default()
            })
            .insert(Worm)
            .insert(Timer::from_seconds(0.03, true))
            .insert(WormDirection{dir:rot})
            .insert(WormSpeed(30.0 + random::<f32>()*20.0))
            .insert(Hp(3.0))
            ;
        level.need_worms -= 1;
    } 
}

fn move_worm(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform, &WormDirection, &WormSpeed), With<Worm>>,
    mut apple_hp: ResMut<AppleHP>
){
    for (e, mut worm_transform, direction, speed) in query.iter_mut() {
        worm_transform.translation += Vec3::new(direction.dir.x, direction.dir.y, 0.0) * time.delta_seconds() * speed.0;
        if worm_transform.translation.distance(Vec3::new(0.,0., 0.)) <= 150.0 {
            println!("Eaten");
            commands.entity(e).despawn();
            apple_hp.hp -= 1;
        }
    }
}

fn animate_worm(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(&mut Timer, &mut TextureAtlasSprite, &Handle<TextureAtlas>), With<Worm>>,
) {
    for (mut timer, mut sprite, texture_atlas_handle) in query.iter_mut() {
        timer.tick(time.delta());
        if timer.finished() {
            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
            sprite.index = ((sprite.index as usize + 1) % texture_atlas.textures.len()) as u32;
            
        }
    }
}

fn check_death(
    mut commands: Commands,
    mut scores: ResMut<Scores>,
    query: Query<(Entity, &Hp)>
){
    for (e, hp) in query.iter(){
        if hp.0 <= 0.0 {
            println!("Killed");
            commands.entity(e).despawn();
            scores.worms += 1;
            print!("Worms now - {}", scores.worms);
        }
    }
}

fn remove_worm(mut commands: Commands, player_query: Query<Entity, With<Worm>>) {
    for player in player_query.iter() {
        commands.entity(player).despawn();
    }
}