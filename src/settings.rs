

use std::fs;
use notan::prelude::Color;


pub const SETTINGS_FILE: &str = "settings.ron";

#[derive(Clone, Copy, serde::Serialize, serde::Deserialize)]
pub struct Theme {
    pub ui_text: Color,
    pub ui_text_dark: Color,
    pub ui_bg: Color,
    pub ui_button: Color,
    pub ui_button_focus: Color,
    pub ui_button_flash: Color,
    pub bg_text: Color,
    pub bg: Color,
    pub seq_text: Color,
    pub seq_bar: Color,
    pub seq_bar_focused: Color,
    pub seq_invalid: Color,
    pub seq_field: Color,
    pub seq_field_focused: Color,
} 

/// The settings for the app
/// 
/// ATTENTION: All fields are private because a macro automatically generate getters and setters.
/// To access a field settings.f, use settings.f() to get a reference to the value, and use settings.set_f() to set the value.
/// The settings struct will be saved to the files when a setter is called
#[derive(serde::Serialize, serde::Deserialize, sequent_macros::SettingsMacro)]
pub struct Settings {
    bindings: crate::action::Bindings,
    theme: Theme,

    /// Should the keys be displayed during game?
    show_game_keys: bool,
}


pub struct LoadError {
    pub message: String,
}

pub struct SaveError {
    pub message: String,
}

pub fn get_default_settings() -> Settings {
    return Settings {
        bindings: crate::action::get_default_bindings(),
        show_game_keys: true,
        theme: get_default_theme(),
    }
}

pub fn save_settings(settings: &Settings) -> Result<(), SaveError> {
    let txt = ron::to_string(settings).map_err(|e| create_save_error(&format!("Couldn't serialize settings: {}", e), SETTINGS_FILE))?;
    return fs::write(SETTINGS_FILE, txt).map_err(|e| create_save_error(&format!("Couldn't write file: {}", e), SETTINGS_FILE));
}

pub fn load_settings() -> Result<Settings, LoadError> {
    match fs::read_to_string(SETTINGS_FILE) {
        Ok(text) => {
            let res = ron::from_str(&text).map_err(|e| create_load_error(&format!("Couldn't parse the file: {}", e), SETTINGS_FILE))?;
            return Ok(res);
        },
        Err(e) => {
            match e.kind() {
                std::io::ErrorKind::NotFound => { // File does not exists, so get default settings
                    return Ok(get_default_settings());
                },
                _ => Err(create_load_error(&format!("Couldn't read file: {}", e), SETTINGS_FILE)),
            }
        },
    }
}

pub fn get_default_theme() -> Theme {
    return Theme {
        ui_text: Color::from_hex(0xeeeeeeff),
        ui_text_dark: Color::from_hex(0x888888ff),
        ui_bg: Color::from_hex(0x222222ff),
        ui_button: Color::from_hex(0x222222ff),
        ui_button_focus: Color::from_hex(0x333333ff),
        ui_button_flash: Color::from_hex(0x555555ff),
        bg_text: Color::from_hex(0x151515ff),
        bg: Color::from_hex(0x080808ff),
        seq_text: Color::from_hex(0xeeeeeeff),
        seq_bar: Color::from_hex(0xeeeeeeff),
        seq_bar_focused: Color::from_hex(0xffccaaff),
        seq_invalid: Color::from_hex(0xff5555ff),
        seq_field: Color::from_hex(0x30308050),
        seq_field_focused: Color::from_hex(0xffffdd50),
    };
} 

fn create_save_error(message: &str, file_path: &str) -> SaveError {
    return SaveError { 
        message: format!("Error while saving settings file {}: {}", file_path, message), 
    };
}

fn create_load_error(message: &str, file_path: &str) -> LoadError {
    return LoadError { 
        message: format!("Error while loading settings file {}: {}", file_path, message), 
    };
}
