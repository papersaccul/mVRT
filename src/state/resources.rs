use crate::constants::*;
use crate::{ColorTarget, DataPoint, InputField};
use bevy::prelude::*;
use std::f32::consts::PI;

// Ресурс для отслеживания статуса загрузки конфига
#[derive(Resource)]
pub struct ConfigState {
    pub config_loaded: bool,
    pub config_path: String,
}

impl Default for ConfigState {
    fn default() -> Self {
        Self {
            config_loaded: false,
            config_path: "assets/config.json".to_string(),
        }
    }
}

#[derive(Resource)]
pub struct Settings {
    pub dpi: f32,
    pub cm_360: f32,
    pub fov: f32,
    pub directional_light_illuminance: f32,
    pub ambient_light_brightness: f32,
    // Основные цвета в HSL формате для логики программы
    pub crosshair_h: f32,
    pub crosshair_s: f32,
    pub crosshair_l: f32,
    pub crosshair_size: f32,
    pub crosshair_thickness: f32,
    pub arena_h: f32,
    pub arena_s: f32,
    pub arena_l: f32,
    pub target_h: f32,
    pub target_s: f32,
    pub target_l: f32,
    // Fresnel settings
    pub fresnel_enabled: bool,
    pub fresnel_h: f32,
    pub fresnel_s: f32,
    pub fresnel_l: f32,
    pub fresnel_intensity: f32,
    pub fresnel_power: f32,
    pub texture_file: String,
    pub font_file: String,
    pub hit_sound_file: String,
    pub key_restart: KeyCode,
    pub key_start: KeyCode,
    pub key_settings: KeyCode,
    pub key_fullscreen: KeyCode,
    // UI состояние для color picker
    pub color_picker_open: bool,
    pub picker_hue: f32,
    pub picker_saturation: f32,
    pub picker_brightness: f32,
    pub picker_target: Option<ColorTarget>,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            dpi: DEFAULT_DPI,
            cm_360: DEFAULT_CM_360,
            fov: DEFAULT_FOV,
            directional_light_illuminance: 4000.0,
            ambient_light_brightness: 250.0,
            // Crosshair: красный цвет (0° hue, 100% saturation, 50% lightness)
            crosshair_h: 0.0,
            crosshair_s: 1.0,
            crosshair_l: 0.5,
            crosshair_size: 10.0,
            crosshair_thickness: 2.0,
            // Arena: голубой цвет (180° hue, 100% saturation, 50% lightness)
            arena_h: 180.0,
            arena_s: 1.0,
            arena_l: 0.5,
            // Target: красный цвет (0° hue, 100% saturation, 50% lightness)
            target_h: 0.0,
            target_s: 1.0,
            target_l: 0.5,
            // Fresnel: желтый цвет (60° hue, 100% saturation, 50% lightness)
            fresnel_enabled: false,
            fresnel_h: 60.0,
            fresnel_s: 1.0,
            fresnel_l: 0.5,
            fresnel_intensity: 1.0,
            fresnel_power: 3.0,
            texture_file: String::from("texture.png"),
            font_file: String::from("font.ttf"),
            hit_sound_file: String::from("hit.ogg"),
            key_restart: KeyCode::KeyR,
            key_start: KeyCode::Space,
            key_settings: KeyCode::Escape,
            key_fullscreen: KeyCode::F12,
            color_picker_open: false,
            picker_hue: 0.0,
            picker_saturation: 1.0,
            picker_brightness: 1.0,
            picker_target: None,
        }
    }
}

impl Settings {
    pub fn mouse_sensitivity(&self) -> f32 {
        let inches_360 = self.cm_360 / 2.54;
        let dots_360 = inches_360 * self.dpi;
        (2.0 * PI) / dots_360
    }

    // Получение цветов в формате Color из HSL компонентов
    pub fn get_crosshair_color(&self) -> Color {
        Color::hsl(self.crosshair_h, self.crosshair_s, self.crosshair_l)
    }

    pub fn get_arena_color(&self) -> Color {
        Color::hsl(self.arena_h, self.arena_s, self.arena_l)
    }

    pub fn get_target_color(&self) -> Color {
        Color::hsl(self.target_h, self.target_s, self.target_l)
    }

    pub fn get_fresnel_color(&self) -> Color {
        Color::hsl(self.fresnel_h, self.fresnel_s, self.fresnel_l)
    }

    // Установка цветов из Color в HSL компоненты
    pub fn set_crosshair_color(&mut self, color: Color) {
        let hsla = Hsla::from(color);
        self.crosshair_h = hsla.hue;
        self.crosshair_s = hsla.saturation;
        self.crosshair_l = hsla.lightness;
    }

    pub fn set_arena_color(&mut self, color: Color) {
        let hsla = Hsla::from(color);
        self.arena_h = hsla.hue;
        self.arena_s = hsla.saturation;
        self.arena_l = hsla.lightness;
    }

    pub fn set_target_color(&mut self, color: Color) {
        let hsla = Hsla::from(color);
        self.target_h = hsla.hue;
        self.target_s = hsla.saturation;
        self.target_l = hsla.lightness;
    }

    pub fn set_fresnel_color(&mut self, color: Color) {
        let hsla = Hsla::from(color);
        self.fresnel_h = hsla.hue;
        self.fresnel_s = hsla.saturation;
        self.fresnel_l = hsla.lightness;
    }

    // Для совместимости с существующим кодом
    pub fn crosshair_color(&self) -> Color {
        self.get_crosshair_color()
    }

    pub fn arena_color(&self) -> Color {
        self.get_arena_color()
    }

    pub fn target_color(&self) -> Color {
        self.get_target_color()
    }

    pub fn fresnel_color(&self) -> Color {
        self.get_fresnel_color()
    }
}

#[derive(Resource)]
pub struct ReactionTest {
    pub data: Vec<DataPoint>,
    pub is_running: bool,
    pub start_time: f32,
    pub target_position: Vec3,
    pub crosshair_direction: Vec3,
    pub target_velocity: Vec3,
    pub target_distance: f32,
    pub start_cam_pos: Vec3,
    pub start_cam_forward: Vec3,
    pub next_direction_change: f32,
    pub change_interval: f32,
    pub rms_distance: f32,
    pub peak_angular_error: f32,
    pub test_completed: bool,
    pub camera_yaw: f32,
    pub camera_pitch: f32,
    pub last_direction_change_time: f32,
    pub hits: u32,
    pub misses: u32,
    pub last_shot_time: f32,
    pub shot_interval: f32,
    pub count_directions: usize,
    pub react_directions: usize,
    pub median_delay: f32,
    pub average_delay: f32,
}

#[derive(Resource)]
pub struct GameState {
    pub cursor_locked: bool,
}

#[derive(Resource)]
pub struct GameAudio {
    pub hit_sound: Handle<AudioSource>,
}

impl Default for ReactionTest {
    fn default() -> Self {
        Self {
            data: Vec::new(),
            is_running: false,
            start_time: 0.0,
            target_position: Vec3::ZERO,
            crosshair_direction: Vec3::NEG_Z,
            target_velocity: Vec3::new(1.0, 1.0, 1.0).normalize() * TARGET_SPEED,
            target_distance: 15.0,
            start_cam_pos: Vec3::ZERO,
            start_cam_forward: Vec3::NEG_Z,
            next_direction_change: 0.1,
            change_interval: 0.1,
            average_delay: 0.0,
            rms_distance: 0.0,
            peak_angular_error: 0.0,
            test_completed: false,
            camera_yaw: 0.0,
            camera_pitch: 0.0,
            last_direction_change_time: 0.0,
            hits: 0,
            misses: 0,
            last_shot_time: 0.0,
            shot_interval: 1.0 / 30.0,
            median_delay: 0.0,
            count_directions: 0,
            react_directions: 0,
        }
    }
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            cursor_locked: false,
        }
    }
}

#[derive(Resource, Default)]
pub struct InputFocus {
    pub focused: Option<InputField>,
}

#[derive(Resource, Default)]
pub struct FpsUiState {
    pub last_update_secs: f32,
}

#[derive(Resource)]
pub struct FresnelTracker {
    pub last_fresnel_enabled: bool,
    pub last_fresnel_color: Color,
    pub last_fresnel_intensity: f32,
    pub last_fresnel_power: f32,
}

impl Default for FresnelTracker {
    fn default() -> Self {
        Self {
            last_fresnel_enabled: false,
            last_fresnel_color: Color::hsl(180.0, 1.0, 0.5),
            last_fresnel_intensity: 1.0,
            last_fresnel_power: 2.0,
        }
    }
}
