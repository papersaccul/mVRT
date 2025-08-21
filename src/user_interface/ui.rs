use crate::state::*;
use crate::target;

use bevy::diagnostic::DiagnosticsStore;
use bevy::prelude::*;

// Константы для современного UI
const UI_BORDER_RADIUS: f32 = 12.0;
const UI_BUTTON_RADIUS: f32 = 8.0;
const UI_PANEL_RADIUS: f32 = 16.0;
const UI_SHADOW_OFFSET: f32 = 2.0;
const UI_GRADIENT_OPACITY: f32 = 0.1;

// Цветовая палитра в HSL
pub const UI_COLORS: UiColors = UiColors {
    primary: Color::hsl(220.0, 0.8, 0.6),   // Синий
    secondary: Color::hsl(280.0, 0.7, 0.5), // Фиолетовый
    accent: Color::hsl(160.0, 0.8, 0.5),    // Зеленый
    success: Color::hsl(120.0, 0.8, 0.5),   // Ярко-зеленый
    warning: Color::hsl(45.0, 0.9, 0.6),    // Оранжевый
    danger: Color::hsl(0.0, 0.8, 0.6),      // Красный
    background: Color::hsla(220.0, 0.1, 0.05, 0.95),
    surface: Color::hsla(220.0, 0.1, 0.1, 0.9),
    surface_light: Color::hsla(220.0, 0.1, 0.15, 0.8),
    text_primary: Color::hsl(220.0, 0.1, 0.95),
    text_secondary: Color::hsl(220.0, 0.1, 0.7),
    text_muted: Color::hsl(220.0, 0.1, 0.5),
};

pub struct UiColors {
    pub primary: Color,
    pub secondary: Color,
    pub accent: Color,
    pub success: Color,
    pub warning: Color,
    pub danger: Color,
    pub background: Color,
    pub surface: Color,
    pub surface_light: Color,
    pub text_primary: Color,
    pub text_secondary: Color,
    pub text_muted: Color,
}

// Компоненты UI
#[derive(Component)]
pub struct ModernButton;

// Система для обновления стилей кнопок при наведении
pub fn update_button_styles(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<ModernButton>),
    >,
) {
    for (interaction, mut background_color) in button_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                *background_color = BackgroundColor(UI_COLORS.primary);
            }
            Interaction::Hovered => {
                *background_color = BackgroundColor(UI_COLORS.secondary);
            }
            Interaction::None => {
                *background_color = BackgroundColor(UI_COLORS.surface_light);
            }
        }
    }
}

pub fn update_ui_crosshair() {
    // UI прицел статичен в центре экрана
}

pub fn update_game_ui(
    mut text_query: Query<&mut Text, With<GameInfoText>>,
    test: Res<ReactionTest>,
    time: Res<Time>,
    mut hint_query: Query<&mut Visibility, With<StartCenterText>>,
    camera_query: Query<&Transform, With<PlayerCamera>>,
) {
    if let Ok(mut text) = text_query.single_mut() {
        if test.is_running {
            let current_time = time.elapsed_secs() - test.start_time;
            let remaining = TEST_DURATION - current_time;

            let cam_pos = camera_query.single().unwrap().translation;

            let target_dir = test.target_position - cam_pos;
            let crosshair_dir = test.crosshair_direction;
            let angular_error = target::calculate_angular_error(target_dir, crosshair_dir);

            text.0 = format!(
                "Time remaining: {:.1}s
Accuracy: {:.2}
Hits: {}
Misses: {}
Angular error: {:.1}°",
                remaining,
                test.hits as f32 / (test.hits + test.misses) as f32 * 100.0,
                test.hits,
                test.misses,
                angular_error
            );

            // Принудительно скрываем подсказку при запуске теста
            for mut v in hint_query.iter_mut() {
                *v = Visibility::Hidden;
            }
        } else if test.test_completed {
            let rating = match test.average_delay {
                d if d < 125.0 => "Supreme",
                d if d < 135.0 => "Grandmaster",
                d if d < 150.0 => "Master",
                d if d < 165.0 => "Diamond",
                d if d < 180.0 => "Platinum",
                d if d < 200.0 => "Gold",
                d if d < 220.0 => "Silver",
                d if d < 250.0 => "Bronze",
                _ => "Keep practicing",
            };

            // Results
            text.0 = format!(
                "TEST RESULTS
Avg Reaction: {:.1} ms
Count Dirs: {}
React Dirs: {}
Accuracy {:.2}%
Hits: {}
Miss: {}
Avg error: {:.4}°
Peak error: {:.4}°\n
Rating: {}\n
SPACE - new test\nESC - settings",
                test.average_delay,
                test.count_directions,
                test.react_directions,
                test.hits as f32 / (test.hits + test.misses) as f32 * 100.0,
                test.hits,
                test.misses,
                test.rms_distance,
                test.peak_angular_error,
                rating
            );

            // Скрываем подсказку при завершении теста
            for mut v in hint_query.iter_mut() {
                *v = Visibility::Hidden;
            }
        } else {
            text.0 = "MODERN REACTION TEST\n".to_string();

            for mut v in hint_query.iter_mut() {
                *v = Visibility::Visible;
            }
        }
    }
}

pub fn update_fps_ui(
    diagnostics: Res<DiagnosticsStore>,
    time: Res<Time>,
    mut fps_state: ResMut<FpsUiState>,
    mut fps_query: Query<&mut Text, With<FpsText>>,
) {
    let now = time.elapsed_secs();
    if now - fps_state.last_update_secs < 0.5 {
        return;
    }
    fps_state.last_update_secs = now;

    if let Some(fps) = diagnostics
        .get(&bevy::diagnostic::FrameTimeDiagnosticsPlugin::FPS)
        .and_then(|d| d.smoothed())
    {
        if let Ok(mut text) = fps_query.get_single_mut() {
            text.0 = format!("FPS: {:.0}", fps);
        }
    }
}
