use crate::constants::*;
use bevy::prelude::KeyCode;
use bevy::prelude::*;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GameConfig {
    pub dpi: f32,
    pub cm_360: f32,
    pub fov: f32,
    pub directional_light_illuminance: f32,
    pub ambient_light_brightness: f32,
    pub crosshair_color: String, // HEX цвет для удобства пользователя
    pub crosshair_size: f32,
    pub crosshair_thickness: f32,
    pub arena_color: String,  // HEX цвет
    pub target_color: String, // HEX цвет
    pub fresnel_enabled: bool,
    pub fresnel_color: String, // HEX цвет
    pub fresnel_intensity: f32,
    pub fresnel_power: f32,
    pub texture_file: String,
    pub font_file: String,
    pub hit_sound_file: String,
    #[serde(
        serialize_with = "serialize_keycode",
        deserialize_with = "deserialize_keycode"
    )]
    pub key_restart: KeyCode,
    #[serde(
        serialize_with = "serialize_keycode",
        deserialize_with = "deserialize_keycode"
    )]
    pub key_start: KeyCode,
    #[serde(
        serialize_with = "serialize_keycode",
        deserialize_with = "deserialize_keycode"
    )]
    pub key_settings: KeyCode,
    #[serde(
        serialize_with = "serialize_keycode",
        deserialize_with = "deserialize_keycode"
    )]
    pub key_fullscreen: KeyCode,
}

impl Default for GameConfig {
    fn default() -> Self {
        Self {
            dpi: DEFAULT_DPI,
            cm_360: DEFAULT_CM_360,
            fov: DEFAULT_FOV,
            directional_light_illuminance: 4000.0,
            ambient_light_brightness: 250.0,
            crosshair_color: "#FF0000".to_string(), // Красный
            crosshair_size: 10.0,
            crosshair_thickness: 2.0,
            arena_color: "#00FFFF".to_string(),  // Голубой
            target_color: "#FF0000".to_string(), // Красный
            fresnel_enabled: false,
            fresnel_color: "#FFFF00".to_string(), // Желтый
            fresnel_intensity: 1.0,
            fresnel_power: 3.0,
            texture_file: String::from("texture.png"),
            font_file: String::from("font.ttf"),
            hit_sound_file: String::from("hit.ogg"),
            key_restart: KeyCode::KeyR,
            key_start: KeyCode::Space,
            key_settings: KeyCode::Escape,
            key_fullscreen: KeyCode::F12,
        }
    }
}

pub fn color_to_hex(color: Color) -> String {
    let srgb = color.to_srgba();
    format!(
        "#{:02X}{:02X}{:02X}",
        (srgb.red * 255.0) as u8,
        (srgb.green * 255.0) as u8,
        (srgb.blue * 255.0) as u8
    )
}

pub fn hex_to_color(hex: &str) -> Result<Color, String> {
    let hex = hex.trim_start_matches('#');
    if hex.len() != 6 {
        return Err("Hex color must be 6 characters long".to_string());
    }

    let r = u8::from_str_radix(&hex[0..2], 16).map_err(|_| "Invalid hex color")?;
    let g = u8::from_str_radix(&hex[2..4], 16).map_err(|_| "Invalid hex color")?;
    let b = u8::from_str_radix(&hex[4..6], 16).map_err(|_| "Invalid hex color")?;

    Ok(Color::srgb(
        r as f32 / 255.0,
        g as f32 / 255.0,
        b as f32 / 255.0,
    ))
}

pub fn serialize_keycode<S>(key: &KeyCode, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let key_str = match key {
        KeyCode::Digit1 => "Digit1",
        KeyCode::Digit2 => "Digit2",
        KeyCode::Digit3 => "Digit3",
        KeyCode::Digit4 => "Digit4",
        KeyCode::Digit5 => "Digit5",
        KeyCode::Digit6 => "Digit6",
        KeyCode::Digit7 => "Digit7",
        KeyCode::Digit8 => "Digit8",
        KeyCode::Digit9 => "Digit9",
        KeyCode::Digit0 => "Digit0",
        KeyCode::KeyA => "KeyA",
        KeyCode::KeyB => "KeyB",
        KeyCode::KeyC => "KeyC",
        KeyCode::KeyD => "KeyD",
        KeyCode::KeyE => "KeyE",
        KeyCode::KeyF => "KeyF",
        KeyCode::KeyG => "KeyG",
        KeyCode::KeyH => "KeyH",
        KeyCode::KeyI => "KeyI",
        KeyCode::KeyJ => "KeyJ",
        KeyCode::KeyK => "KeyK",
        KeyCode::KeyL => "KeyL",
        KeyCode::KeyM => "KeyM",
        KeyCode::KeyN => "KeyN",
        KeyCode::KeyO => "KeyO",
        KeyCode::KeyP => "KeyP",
        KeyCode::KeyQ => "KeyQ",
        KeyCode::KeyR => "KeyR",
        KeyCode::KeyS => "KeyS",
        KeyCode::KeyT => "KeyT",
        KeyCode::KeyU => "KeyU",
        KeyCode::KeyV => "KeyV",
        KeyCode::KeyW => "KeyW",
        KeyCode::KeyX => "KeyX",
        KeyCode::KeyY => "KeyY",
        KeyCode::KeyZ => "KeyZ",
        KeyCode::Escape => "Escape",
        KeyCode::Space => "Space",
        KeyCode::F1 => "F1",
        KeyCode::F2 => "F2",
        KeyCode::F3 => "F3",
        KeyCode::F4 => "F4",
        KeyCode::F5 => "F5",
        KeyCode::F6 => "F6",
        KeyCode::F7 => "F7",
        KeyCode::F8 => "F8",
        KeyCode::F9 => "F9",
        KeyCode::F10 => "F10",
        KeyCode::F11 => "F11",
        KeyCode::F12 => "F12",
        _ => "F35",
    };
    serializer.serialize_str(key_str)
}

// Функция для десериализации строки в KeyCode
pub fn deserialize_keycode<'de, D>(deserializer: D) -> Result<KeyCode, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    Ok(match s.as_str() {
        "Digit1" => KeyCode::Digit1,
        "Digit2" => KeyCode::Digit2,
        "Digit3" => KeyCode::Digit3,
        "Digit4" => KeyCode::Digit4,
        "Digit5" => KeyCode::Digit5,
        "Digit6" => KeyCode::Digit6,
        "Digit7" => KeyCode::Digit7,
        "Digit8" => KeyCode::Digit8,
        "Digit9" => KeyCode::Digit9,
        "Digit0" => KeyCode::Digit0,
        "KeyA" => KeyCode::KeyA,
        "KeyB" => KeyCode::KeyB,
        "KeyC" => KeyCode::KeyC,
        "KeyD" => KeyCode::KeyD,
        "KeyE" => KeyCode::KeyE,
        "KeyF" => KeyCode::KeyF,
        "KeyG" => KeyCode::KeyG,
        "KeyH" => KeyCode::KeyH,
        "KeyI" => KeyCode::KeyI,
        "KeyJ" => KeyCode::KeyJ,
        "KeyK" => KeyCode::KeyK,
        "KeyL" => KeyCode::KeyL,
        "KeyM" => KeyCode::KeyM,
        "KeyN" => KeyCode::KeyN,
        "KeyO" => KeyCode::KeyO,
        "KeyP" => KeyCode::KeyP,
        "KeyQ" => KeyCode::KeyQ,
        "KeyR" => KeyCode::KeyR,
        "KeyS" => KeyCode::KeyS,
        "KeyT" => KeyCode::KeyT,
        "KeyU" => KeyCode::KeyU,
        "KeyV" => KeyCode::KeyV,
        "KeyW" => KeyCode::KeyW,
        "KeyX" => KeyCode::KeyX,
        "KeyY" => KeyCode::KeyY,
        "KeyZ" => KeyCode::KeyZ,
        "Escape" => KeyCode::Escape,
        "Space" => KeyCode::Space,
        "F1" => KeyCode::F1,
        "F2" => KeyCode::F2,
        "F3" => KeyCode::F3,
        "F4" => KeyCode::F4,
        "F5" => KeyCode::F5,
        "F6" => KeyCode::F6,
        "F7" => KeyCode::F7,
        "F8" => KeyCode::F8,
        "F9" => KeyCode::F9,
        "F10" => KeyCode::F10,
        "F11" => KeyCode::F11,
        "F12" => KeyCode::F12,
        _ => KeyCode::F35,
    })
}
