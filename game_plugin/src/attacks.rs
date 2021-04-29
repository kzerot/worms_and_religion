use std::f32::consts::PI;

use crate::{game::Effects, loading::TextureAssets, worms::Hp, worms::Worm};
use crate::GameState;
use crate::actions::*;
use bevy::prelude::*;

pub struct AttackPlugin;

pub struct AttackPower(f32);
pub struct Pater;
pub struct AttackDirection{pub dir: Vec2}

impl Plugin for AttackPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(SystemSet::on_update(GameState::Playing)
            .with_system(spawn_avemater.system())
            .with_system(move_avemater.system())
            .with_system(move_pater.system())
            .with_system(spawn_signum.system())
            .with_system(damage.system())
        )
        // .add_system_set(SystemSet::on_update(GameState::Playing).with_system(move_player.system()))
        .add_system_set(SystemSet::on_exit(GameState::Playing).with_system(remove_missiles.system()));
    }
}

fn spawn_avemater(
    textures: Res<TextureAssets>,
    mut effects: ResMut<Effects>,
    mut commands: Commands,
    mut ev_prayer: EventReader<PrayerComplete>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    query: Query<&Transform, With<Worm>>
) {
    for ev in ev_prayer.iter() {
      if ev.0 == "ave_maria"{
        let texture = textures.texture_avemater.clone();
        let mut nearest = 1000.0;
        let mut rot = Vec2::new(0.0, 0.0);
        for transform in query.iter(){
            let dist = transform.translation.distance(Vec3::new(0.,0.,0.));
            if dist < nearest{
              nearest = dist;
              rot.x = transform.translation.x;
              rot.y = transform.translation.y;
              rot = rot.normalize();
            }
        }
        let mut transform = Transform::from_translation(Vec3::new(0.0, 0.0, 5.0));
        transform.rotation = Quat::from_rotation_z(Vec2::new(1.,0.).angle_between(rot));
        commands
            .spawn_bundle(SpriteBundle {
                material: materials.add(texture.into()),
                transform,
                ..Default::default()
            })
            .insert(AttackDirection{dir:rot})
            .insert(AttackPower(5.0))
            ;
        }
      else if ev.0 == "pater" {
          for i in 0..4{

              let texture: Handle<Texture> = textures.texture_avemater.clone();
              let mut transform = Transform::from_translation(Vec3::new(0.0, 0.0, 5.0));
              transform.rotation = Quat::from_rotation_z((i as f32) * PI / 2.0);
              transform.scale = Vec3:: new(0.5,0.5,0.5);
              commands
              .spawn_bundle(SpriteBundle {
                  material: materials.add(texture.into()),
                  transform,
                  ..Default::default()
              })
              .insert(AttackPower(2.0))
              .insert(Pater)
              ;
            }
          }
        else if ev.0 == "signum"{
            effects.signum += 30;
        }

      }
}

fn spawn_signum(
  time: Res<Time>,
  textures: Res<TextureAssets>,
  mut commands: Commands,
  mut effects: ResMut<Effects>,
  mut materials: ResMut<Assets<ColorMaterial>>,
){
  if effects.signum > 0 && time.seconds_since_startup() - effects.last_signum > 0.5{
    effects.last_signum = time.seconds_since_startup();
    effects.last_signum_angle += PI/10.0;
    effects.signum -= 1;
    let texture = textures.texture_avemater.clone();

    let rot3 = Quat::from_rotation_z(effects.last_signum_angle) *  Vec3::new(1.0, 0.0, 0.0);
    let rot = Vec2::new(rot3.x, rot3.y);
    let mut transform = Transform::from_translation(Vec3::new(0.0, 0.0, 5.0));
    transform.rotation = Quat::from_rotation_z(Vec2::new(1.,0.).angle_between(rot));
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(texture.into()),
            transform,
            ..Default::default()
        })
        .insert(AttackDirection{dir:rot})
        .insert(AttackPower(5.0))
        ;
    }
}


fn move_pater(
  time: Res<Time>,
  mut query: Query<&mut Transform, With<Pater>>
){
  for mut t in query.iter_mut(){
    let dir = t.rotation * Vec3::new(1.,0.,0.);
    t.translation += dir * time.delta_seconds() * 150.0;
    t.rotate(Quat::from_rotation_z(time.delta_seconds()* 0.8)); 
  }
}

fn move_avemater(
  time: Res<Time>,
  mut query: Query<(&mut Transform, &AttackDirection), Without<Pater>>
){
  for (mut t, d) in query.iter_mut(){
    t.translation += Vec3::new(d.dir.x, d.dir.y, 0.0) * time.delta_seconds() * 100.0;

  }
}

fn damage(
  mut commands: Commands,
  query: Query<(&Transform, Entity, &AttackPower)>,
  mut query2: Query<(&Transform, &mut Hp), With<Worm>>
  
){
  for (t_praryer, e_prayer, attack) in query.iter(){
    for (t_worm, mut hp) in query2.iter_mut(){
      let dist = t_praryer.translation.distance(t_worm.translation);
      if dist <= 50.0{
        commands.entity(e_prayer).despawn();

        hp.0 -= attack.0;
      }

    }
}
}

fn remove_missiles(
  mut commands: Commands,
  query: Query<Entity, With<AttackPower>>
){
  for e in query.iter(){
    commands.entity(e).despawn();
  }
}

