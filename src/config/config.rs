use bevy::prelude::*;
use log::{error, info, warn};
use std::fs;
use std::path::Path;

//use super::cfgsettings::{color_to_hex, hex_to_color, GameConfig};
use crate::state::*;

pub struct ConfigPlugin;

impl Plugin for ConfigPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ConfigState>()
            .add_event::<SaveConfigEvent>()
            .add_systems(OnEnter(AppState::Loading), load_config_and_init_settings)
            .add_systems(Update, (auto_save_config_system, save_config_event_system))
            .add_systems(OnExit(AppState::Settings), save_config_on_settings_exit);
    }
}

// Конвертация из GameConfig в Settings
impl From<GameConfig> for Settings {
    fn from(config: GameConfig) -> Self {
        let mut settings = Self::default();

        settings.dpi = config.dpi;
        settings.cm_360 = config.cm_360;
        settings.fov = config.fov;
        settings.directional_light_illuminance = config.directional_light_illuminance;
        settings.ambient_light_brightness = config.ambient_light_brightness;
        settings.crosshair_size = config.crosshair_size;
        settings.crosshair_thickness = config.crosshair_thickness;
        settings.fresnel_enabled = config.fresnel_enabled;
        settings.fresnel_intensity = config.fresnel_intensity;
        settings.fresnel_power = config.fresnel_power;
        settings.texture_file = config.texture_file;
        settings.font_file = config.font_file;
        settings.hit_sound_file = config.hit_sound_file;
        settings.key_restart = config.key_restart;
        settings.key_start = config.key_start;
        settings.key_settings = config.key_settings;
        settings.key_fullscreen = config.key_fullscreen;

        // Конвертируем HEX цвета в HSL компоненты
        if let Ok(crosshair_color) = hex_to_color(&config.crosshair_color) {
            settings.set_crosshair_color(crosshair_color);
        }

        if let Ok(arena_color) = hex_to_color(&config.arena_color) {
            settings.set_arena_color(arena_color);
        }

        if let Ok(target_color) = hex_to_color(&config.target_color) {
            settings.set_target_color(target_color);
        }

        if let Ok(fresnel_color) = hex_to_color(&config.fresnel_color) {
            settings.set_fresnel_color(fresnel_color);
        }

        settings
    }
}

// Конвертация из Settings в GameConfig
impl From<&Settings> for GameConfig {
    fn from(settings: &Settings) -> Self {
        Self {
            dpi: settings.dpi,
            cm_360: settings.cm_360,
            fov: settings.fov,
            directional_light_illuminance: settings.directional_light_illuminance,
            ambient_light_brightness: settings.ambient_light_brightness,
            // Конвертируем HSL цвета в HEX для сохранения
            crosshair_color: color_to_hex(settings.get_crosshair_color()),
            crosshair_size: settings.crosshair_size,
            crosshair_thickness: settings.crosshair_thickness,
            arena_color: color_to_hex(settings.get_arena_color()),
            target_color: color_to_hex(settings.get_target_color()),
            fresnel_enabled: settings.fresnel_enabled,
            fresnel_color: color_to_hex(settings.get_fresnel_color()),
            fresnel_intensity: settings.fresnel_intensity,
            fresnel_power: settings.fresnel_power,
            texture_file: settings.texture_file.clone(),
            font_file: settings.font_file.clone(),
            hit_sound_file: settings.hit_sound_file.clone(),
            key_restart: settings.key_restart,
            key_start: settings.key_start,
            key_settings: settings.key_settings,
            key_fullscreen: settings.key_fullscreen,
        }
    }
}

// Система для автоматического сохранения при изменении настроек
pub fn auto_save_config_system(
    settings: Res<Settings>,
    config_state: Res<ConfigState>,
    mut save_events: EventWriter<SaveConfigEvent>,
) {
    if settings.is_changed() && config_state.config_loaded {
        save_events.send(SaveConfigEvent);
    }
}

// Система для обработки события сохранения конфига
pub fn save_config_event_system(
    mut save_events: EventReader<SaveConfigEvent>,
    settings: Res<Settings>,
    config_state: Res<ConfigState>,
) {
    for _event in save_events.read() {
        let config = GameConfig::from(settings.as_ref());
        match save_config(&config, &config_state.config_path) {
            Ok(_) => {
                info!("Config saved successfully");
            }
            Err(e) => {
                error!("Failed to save config: {}", e);
            }
        }
    }
}

// Функция для загрузки конфига из файла
fn load_config(path: &str) -> Result<GameConfig, Box<dyn std::error::Error>> {
    if !Path::new(path).exists() {
        return Err("Config file does not exist".into());
    }

    let config_content = fs::read_to_string(path)?;
    let config: GameConfig = serde_json::from_str(&config_content)?;
    Ok(config)
}

// Функция для сохранения конфига в файл
fn save_config(config: &GameConfig, path: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Создаем директорию assets если она не существует
    if let Some(parent) = Path::new(path).parent() {
        fs::create_dir_all(parent)?;
    }

    let config_json = serde_json::to_string_pretty(config)?;
    fs::write(path, config_json)?;
    Ok(())
}

// Система для загрузки конфига и инициализации Settings в PreStartup
pub fn load_config_and_init_settings(mut commands: Commands, config_state: Res<ConfigState>) {
    let settings = match load_config(&config_state.config_path) {
        Ok(config) => {
            info!(
                "Config loaded successfully from {}",
                config_state.config_path
            );
            Settings::from(config)
        }
        Err(e) => {
            warn!("Failed to load config: {}. Creating default config.", e);
            let default_config = GameConfig::default();
            if let Err(save_err) = save_config(&default_config, &config_state.config_path) {
                error!("Failed to create default config file: {}", save_err);
            } else {
                info!("Default config created at {}", config_state.config_path);
            }
            Settings::from(default_config)
        }
    };

    // Инициализируем Settings ресурс
    commands.insert_resource(settings);

    // Отмечаем, что конфиг загружен
    commands.insert_resource(ConfigState {
        config_loaded: true,
        config_path: config_state.config_path.clone(),
    });
}

// Система для сохранения конфига при выходе из настроек
pub fn save_config_on_settings_exit(settings: Res<Settings>, config_state: Res<ConfigState>) {
    if config_state.config_loaded {
        let config = GameConfig::from(settings.as_ref());
        if let Err(e) = save_config(&config, &config_state.config_path) {
            error!("Failed to save config on settings exit: {}", e);
        } else {
            info!("Config saved on settings exit");
        }
    }
}

// Функция для ручного сохранения (можно вызывать из других частей кода)
pub fn manual_save_config(
    settings: &Settings,
    config_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let config = GameConfig::from(settings);
    save_config(&config, config_path)
}

pub fn config_loaded(config_state: Res<ConfigState>) -> bool {
    config_state.config_loaded
}
