use bevy::prelude::*;
use crate::actions::TextEntered;
use crate::GameState;

pub struct PrayersPlugin;
pub struct PrayerText;

impl Plugin for PrayersPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
        .add_system_set(SystemSet::on_enter(GameState::Playing)
            .with_system(setup_prayers.system())
        )
        .add_system_set(SystemSet::on_update(GameState::Playing)
            .with_system(show_prayers.system())

        );

    }
}

fn setup_prayers(mut commands: Commands, asset_server: Res<AssetServer>){
    commands
    .spawn_bundle(TextBundle {
        style: Style {
            align_self: AlignSelf::Center,
            position_type: PositionType::Absolute,
            position: Rect {
                bottom: Val::Px(30.0),
                right: Val::Px(30.0),
                left: Val::Px(30.0),
                top: Val::Px(30.0),
                ..Default::default()
            },
            ..Default::default()
        },
        // Use the `Text::with_section` constructor
        text: Text::with_section(
            // Accepts a `String` or any type that converts into a `String`, such as `&str`
            "hello\nbevy!",
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 20.0,
                color: Color::WHITE,
            },
            // Note: You can use `Default::default()` in place of the `TextAlignment`
            TextAlignment {
                horizontal: HorizontalAlign::Center,
                ..Default::default()
            },
        ),
        ..Default::default()
    }).insert(PrayerText);
}

fn show_prayers(
    text: Res<TextEntered>,
    mut query: Query<&mut Text, With<PrayerText>>
){
    if text.is_changed(){
        let mut text_field = query.single_mut().expect("Should be only one!");
        text_field.sections[0].value = text.text_entered.clone();
    }
}