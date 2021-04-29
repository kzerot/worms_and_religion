use std::cmp::max;

use crate::{GameState};

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

#[derive(Default)]
pub struct Scores{
  pub time: i64,
  pub worms: i64
}
#[derive(Default)]
pub struct Effects{
  pub signum:i32,
  pub last_signum:f64,
  pub last_signum_angle:f32
}

impl Plugin for GameProcessPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
        .insert_resource(Scores{..Default::default()})
        .add_startup_system(init_scores.system())
        .add_system(update_scores.system())
        .add_system_set(SystemSet::on_enter(GameState::Playing).with_system(setup_level.system()))
        .add_system_set(SystemSet::on_update(GameState::Playing).with_system(level_check.system()))
        .add_system_set(SystemSet::on_update(GameState::Playing).with_system(check_lose.system()))
        ;

    }
}

pub struct ScoreWorms;

fn init_scores(mut commands: Commands, asset_server: Res<AssetServer>){
    commands
    .spawn_bundle(TextBundle {
        style: Style {
            align_self: AlignSelf::FlexStart,
            position_type: PositionType::Absolute,
            position: Rect {
                top: Val::Px(5.0),
                right: Val::Px(10.0),
                ..Default::default()
            },
            // size: Size::new(Val::Percent(100.0), Val::Px(200.0)),
            ..Default::default()
        },
        // Use the `Text::with_section` constructor
        text: Text::with_section(
            "0",
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 30.0,
                color: Color::DARK_GRAY,
            },
            TextAlignment {
                ..Default::default()
            },
        ),
        ..Default::default()
    }).insert(ScoreWorms);



    commands
    .spawn_bundle(TextBundle {
        style: Style {
            align_self: AlignSelf::FlexStart,
            position_type: PositionType::Absolute,
            position: Rect {
                bottom: Val::Px(5.0),
                left: Val::Px(10.0),
                ..Default::default()
            },
            // size: Size::new(Val::Percent(100.0), Val::Px(200.0)),
            ..Default::default()
        },
        // Use the `Text::with_section` constructor
        text: Text::with_section(
            "Prayers: Ave Mater, Pater Noster, Signum Crucis",
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 30.0,
                color: Color::DARK_GRAY,
            },
            TextAlignment {
                ..Default::default()
            },
        ),
        ..Default::default()
    });
}

fn update_scores(
  sc: Res<Scores>,
  mut q: Query<&mut Text, With<ScoreWorms>>
){
  for mut t in q.iter_mut(){
      let newtext = format!("{}", sc.worms);
      t.sections[0].value = newtext.clone();
  }
}


fn setup_level(
  mut commands: Commands,
  mut sc : ResMut<Scores>
) {
    sc.worms = 0;
    commands.insert_resource(Level{
      level:0,
      time_over:0.0,
      need_worms:1
    });
    commands.insert_resource(Effects{..Default::default()});
    commands.insert_resource(AppleHP{hp:17});

}

fn level_check(
    time: Res<Time>,
    mut level: ResMut<Level>
) {
    level.time_over += time.delta_seconds_f64();
    if level.time_over > (level.level as f64) * 3.0 {
      level.level += 1;
      level.need_worms = max((0.3 + level.level as f32 * 0.2) as i32, 1);
    }
}

fn check_lose(
  mut state: ResMut<State<GameState>>,
  apple_hp: Res<AppleHP>
){
    if apple_hp.hp <= 0{
      println!("Lose");
      state.set(GameState::Menu).unwrap();
    }
}

