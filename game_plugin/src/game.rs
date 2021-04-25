use crate::GameState;
use bevy::prelude::*;


pub struct GameProcessPlugin;

pub struct Level{
  level: i32,
  time_over:f64,
  pub need_worms: i32
}

pub struct AppleHP{
  pub hp:i32
}

impl Plugin for GameProcessPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
        .add_system_set(SystemSet::on_enter(GameState::Playing).with_system(setup_level.system()))
        .add_system_set(SystemSet::on_update(GameState::Playing).with_system(level_check.system()))
        .add_system_set(SystemSet::on_update(GameState::Playing).with_system(check_lose.system()))
        ;

    }
}

fn setup_level(
  mut commands: Commands,
) {
    commands.insert_resource(Level{
      level:0,
      time_over:0.0,
      need_worms:1
    });
    commands.insert_resource(AppleHP{hp:100});

}

fn level_check(
    time: Res<Time>,
    mut level: ResMut<Level>
) {
    level.time_over += time.delta_seconds_f64();
    if level.time_over > (level.level as f64) * 10.0 {
      level.level += 1;
      level.need_worms = level.level * 5;
    }
}

fn check_lose(
  apple_hp: Res<AppleHP>
){
    if apple_hp.hp <= 0{
      println!("Lose");
    }
}

