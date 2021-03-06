mod paths;

use crate::loading::paths::PATHS;
use crate::GameState;
use bevy::asset::LoadState;
use bevy::prelude::*;
use bevy_kira_audio::AudioSource;

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            SystemSet::on_enter(GameState::Loading).with_system(start_loading.system()),
        )
        .add_system_set(SystemSet::on_update(GameState::Loading).with_system(check_state.system()));
    }
}

pub struct LoadingState {
    textures: Vec<HandleUntyped>,
    fonts: Vec<HandleUntyped>,
    audio: Vec<HandleUntyped>,
}

pub struct FontAssets {
    pub fira_sans: Handle<Font>,
}

pub struct AudioAssets {
    pub flying: Handle<AudioSource>,
}

pub struct TextureAssets {
    pub texture_bevy: Handle<Texture>,
    pub texture_worm: Handle<Texture>,
    pub texture_avemater: Handle<Texture>,
    pub texture_apple: Handle<Texture>,
    pub texture_back: Handle<Texture>

}


fn start_loading(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut fonts: Vec<HandleUntyped> = vec![];
    fonts.push(asset_server.load_untyped(PATHS.fira_sans));

    let mut audio: Vec<HandleUntyped> = vec![];
    audio.push(asset_server.load_untyped(PATHS.audio_flying));

    let mut textures: Vec<HandleUntyped> = vec![];
    textures.push(asset_server.load_untyped(PATHS.texture_bevy));
    textures.push(asset_server.load_untyped(PATHS.texture_worm));
    textures.push(asset_server.load_untyped(PATHS.texture_avemater));
    textures.push(asset_server.load_untyped(PATHS.texture_apple));
    textures.push(asset_server.load_untyped(PATHS.texture_back));
    
    commands.insert_resource(LoadingState {
        textures,
        fonts,
        audio,
    });

}

fn check_state(
    mut commands: Commands,
    mut state: ResMut<State<GameState>>,
    asset_server: Res<AssetServer>,
    loading_state: Res<LoadingState>,
) {
    if LoadState::Loaded
        != asset_server.get_group_load_state(loading_state.fonts.iter().map(|handle| handle.id))
    {
        return;
    }
    if LoadState::Loaded
        != asset_server.get_group_load_state(loading_state.textures.iter().map(|handle| handle.id))
    {
        return;
    }
    if LoadState::Loaded
        != asset_server.get_group_load_state(loading_state.audio.iter().map(|handle| handle.id))
    {
        return;
    }

    commands.insert_resource(FontAssets {
        fira_sans: asset_server.get_handle(PATHS.fira_sans),
    });

    commands.insert_resource(AudioAssets {
        flying: asset_server.get_handle(PATHS.audio_flying),
    });

    commands.insert_resource(TextureAssets {
        texture_bevy: asset_server.get_handle(PATHS.texture_bevy),
        texture_worm: asset_server.get_handle(PATHS.texture_worm),
        texture_avemater: asset_server.get_handle(PATHS.texture_avemater),
        texture_apple: asset_server.get_handle(PATHS.texture_apple),
        texture_back: asset_server.get_handle(PATHS.texture_back)
    });


    state.set(GameState::Menu).unwrap();
}
