use crate::GameState;
use bevy::{prelude::*, utils::HashMap};

pub struct Prayers {
    prayers: HashMap<String, String>
} 
pub struct ActionsPlugin;

impl Plugin for ActionsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        let mut prayers: HashMap<String, String> = HashMap::default();
        prayers.insert("ave_maria".to_string(), "Ave Maria".to_uppercase().to_string());
        prayers.insert("pater".to_string(), "Pater Noster".to_uppercase().to_string());
        prayers.insert("signum".to_string(), "Signum Crucis".to_uppercase().to_string());
        
        app
        .add_event::<PrayerComplete>()
        .insert_resource(
            TextEntered{
                text_entered: "".to_string(),
                text_match: "".to_string(),
                text_code: "".to_string()
            }
        )
        .insert_resource(
            Prayers{
                prayers
            }
        )
        .add_system_set(
            SystemSet::on_update(GameState::Playing).with_system(prayer_inputed.system()),
        );
    }

    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }
}



pub struct TextEntered {
    pub text_entered: String,
    pub text_match: String,
    text_code: String
}

fn key_to_string(key_code: &KeyCode) -> &'static str {
    let key = match key_code {

        KeyCode::A => "A",
        KeyCode::B => "B",
        KeyCode::C => "C",
        KeyCode::D => "D",
        KeyCode::E => "E",
        KeyCode::F => "F",
        KeyCode::G => "G",
        KeyCode::H => "H",
        KeyCode::I => "I",
        KeyCode::J => "J",
        KeyCode::K => "K",
        KeyCode::L => "L",
        KeyCode::M => "M",
        KeyCode::N => "N",
        KeyCode::O => "O",
        KeyCode::P => "P",
        KeyCode::Q => "Q",
        KeyCode::R => "R",
        KeyCode::S => "S",
        KeyCode::T => "T",
        KeyCode::U => "U",
        KeyCode::V => "V",
        KeyCode::W => "W",
        KeyCode::X => "X",
        KeyCode::Y => "Y",
        KeyCode::Z => "Z",
        KeyCode::Space => " ",
        _ => return "",
    };
    key
}

pub struct PrayerComplete(pub String);

fn prayer_inputed(
    keyboard_input: Res<Input<KeyCode>>,
    prayers: Res<Prayers>,
    mut text: ResMut<TextEntered>,
    mut ev: EventWriter<PrayerComplete>,
) {
    let keys = keyboard_input.get_just_pressed();
    for key in keys{
        let key_literal = key_to_string(key);
        println!("{}", key_literal);
        // Start new prayer
        if text.text_match.len() == 0{
            for (key, value) in &prayers.prayers {
                if value.starts_with(key_literal){
                    println!("{}", key);
                    text.text_match = String::from(value);
                    text.text_entered = key_literal.to_string();
                    text.text_code = key.to_string();
                }
            }
        } 
        else{
            let mut new_text = text.text_entered.clone();
            new_text.push_str(key_literal.into());
            if text.text_match.starts_with(&new_text){
                text.text_entered = new_text;
                println!("Now prayer: '{}'", text.text_entered);
                if text.text_entered == text.text_match{
                    ev.send(PrayerComplete(text.text_code.clone()));
                    text.text_entered = "".to_string();
                    text.text_code = "".to_string();
                    text.text_match = "".to_string();
                    println!("Completed prayer");
                } 
            }
        }
    }
}
