use bevy::prelude::*;
use crate::actions::TextEntered;
use crate::GameState;
use crate::game::ScoreWorms;
pub struct PrayersPlugin;
pub struct PrayerText;
pub struct PrayerFullText;


impl Plugin for PrayersPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
        .add_system_set(SystemSet::on_enter(GameState::Playing)
            .with_system(setup_prayers.system())
        )
        .add_system_set(SystemSet::on_update(GameState::Playing)
            .with_system(show_prayers.system())
            .with_system(show_full_prayers.system())
        )
        .add_system_set(SystemSet::on_exit(GameState::Playing)
            .with_system(despawn_all.system())
        )
        ;

    }
}

fn setup_prayers(mut commands: Commands, asset_server: Res<AssetServer>){
    commands
    .spawn_bundle(TextBundle {
        style: Style {
            align_self: AlignSelf::FlexStart,
            position_type: PositionType::Absolute,
            position: Rect {
                top: Val::Px(5.0),
                left: Val::Px(100.0),
                ..Default::default()
            },
            // size: Size::new(Val::Percent(100.0), Val::Px(200.0)),
            ..Default::default()
        },
        // Use the `Text::with_section` constructor
        text: Text::with_section(
            "Start type prayer!",
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
    }).insert(PrayerFullText)
    .with_children(|parent| {
        parent.spawn_bundle(TextBundle {
            text: Text {
                sections: vec![TextSection {
                    value: "".to_string(),
                    style: TextStyle {
                        font: asset_server.get_handle("fonts/FiraSans-Bold.ttf"),
                        font_size: 30.0,
                        color: Color::WHITE,
                    },
                }],
                alignment: Default::default(),
            },
            ..Default::default()
        }).insert(PrayerText);
    });



}
fn show_prayers(
    text: Res<TextEntered>,
    mut query: Query<&mut Text, With<PrayerText>>,
){
    if text.is_changed(){
        let mut text_field = query.single_mut().expect("Should be only one!");
        text_field.sections[0].value = text.text_entered.clone();
    }
}

fn show_full_prayers(
    text: Res<TextEntered>,
    mut query: Query<&mut Text, With<PrayerFullText>>,
){
    if text.is_changed(){
        let mut text_field = query.single_mut().expect("Should be only one!");
        text_field.sections[0].value = text.text_match.clone();
    }
}

fn despawn_all(
    mut commands: Commands,
    q: Query<Entity, (With<Text>, Without<ScoreWorms>)>
){
    for e in q.iter(){
        commands.entity(e).despawn();
    }
}