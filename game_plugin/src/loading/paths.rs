pub struct AssetPaths {
    pub fira_sans: &'static str,
    pub audio_flying: &'static str,
    pub texture_bevy: &'static str,
    pub texture_worm: &'static str,
    pub texture_avemater: &'static str,
    pub texture_apple: &'static str,
    pub texture_back: &'static str,
}

pub const PATHS: AssetPaths = AssetPaths {
    fira_sans: "fonts/FiraSans-Bold.ttf",
    audio_flying: "audio/flying.ogg",
    texture_bevy: "textures/bevy.png",
    texture_worm: "textures/worm.png",
    texture_avemater: "textures/light.png",
    texture_apple: "textures/TheApple.png",
    texture_back: "textures/back.png",
};
