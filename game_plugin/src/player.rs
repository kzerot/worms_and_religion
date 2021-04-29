use crate::{game::AppleHP, loading::TextureAssets};
use crate::GameState;
use bevy::prelude::*;


pub struct PlayerPlugin;

pub struct Player;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            SystemSet::on_enter(GameState::Playing)
                .with_system(spawn_player.system())
                .with_system(spawn_camera.system())
        )
        .add_system_set(SystemSet::on_update(GameState::Playing)
        .with_system(apple_death.system())
        )
        .add_system_set(SystemSet::on_exit(GameState::Playing)
        .with_system(remove_player.system())    
        );
    }
}


fn spawn_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn spawn_player(
    mut commands: Commands,
    textures: Res<TextureAssets>,

    mut materials: ResMut<Assets<ColorMaterial>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture = textures.texture_apple.clone().into();
    let texture_atlas = TextureAtlas::from_grid(texture, Vec2::new(256.0, 256.0), 17, 1);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    let texture_back: Handle<Texture> = textures.texture_back.clone();
    commands
        .spawn_bundle(
            SpriteBundle {
                  material: materials.add(texture_back.into()),
                  ..Default::default()
              }
        ).with_children(|parent| {
            parent.spawn_bundle(SpriteSheetBundle {
                texture_atlas:texture_atlas_handle,
                transform: Transform::from_translation(Vec3::new(0., 0., 1.)),
                sprite: TextureAtlasSprite{
                    index: 0,
                    ..Default::default()
                },
                ..Default::default()
            }).insert(Player);
        })
        .insert(Player);
}

fn apple_death(
    apple_hp: Res<AppleHP>,
    mut query: Query<&mut TextureAtlasSprite, With<Player>>,
){
    for mut sprite in query.iter_mut() {
        if apple_hp.hp >= 0{
            sprite.index = (17 - apple_hp.hp) as u32;
        }            

    }
}
fn remove_player(mut commands: Commands, player_query: Query<Entity, With<Player>>) {
    for player in player_query.iter() {
        commands.entity(player).despawn();
    }
}
