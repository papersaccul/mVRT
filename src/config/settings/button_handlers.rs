use crate::state::*;
use crate::user_interface::ui::{ModernButton, UI_COLORS};
use bevy::prelude::*;
use bevy::window::{MonitorSelection, PrimaryWindow, WindowMode};

// Handle fullscreen toggle button
pub fn handle_fullscreen_toggle(
    mut interaction_query: Query<&Interaction, (With<FullscreenToggle>, Changed<Interaction>)>,
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
) {
    if let Ok(interaction) = interaction_query.get_single_mut() {
        if *interaction == Interaction::Pressed {
            if let Ok(mut window) = windows.get_single_mut() {
                window.mode = match window.mode {
                    WindowMode::Windowed => {
                        WindowMode::BorderlessFullscreen(MonitorSelection::Primary)
                    }
                    _ => WindowMode::Windowed,
                };
            }
        }
    }
}

// Система для обработки кликов по Fresnel настройкам
pub fn handle_fresnel_clicks(
    mut settings: ResMut<Settings>,
    mut query: Query<
        (
            Entity,
            &Interaction,
            &mut BackgroundColor,
            Option<&Children>,
        ),
        (Changed<Interaction>, With<FresnelEnabledCheckbox>),
    >,
    mut texts: Query<&mut Text>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    for (entity, interaction, mut bg, maybe_children) in query.iter_mut() {
        if *interaction == Interaction::Pressed {
            settings.fresnel_enabled = !settings.fresnel_enabled;

            *bg = BackgroundColor(if settings.fresnel_enabled {
                UI_COLORS.success
            } else {
                UI_COLORS.surface_light
            });
        }
    }
}

pub fn settings_button_system(
    mut next_state: ResMut<NextState<AppState>>,
    mut settings: ResMut<Settings>,
    mut interactions: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            Option<&BtnArenaColor>,
            Option<&BtnTargetColor>,
            Option<&BtnBackMenu>,
            Option<&BtnStartGame>,
            Option<&DiscordLink>,
        ),
        (
            Changed<Interaction>,
            Or<(With<ModernButton>, With<DiscordLink>)>,
        ),
    >,
) {
    for (interaction, mut bg, arena_btn, target_btn, back_btn, start_btn, discord_link) in
        interactions.iter_mut()
    {
        match *interaction {
            Interaction::Pressed => {
                // Visual feedback
                *bg = BackgroundColor(Color::srgb(0.3, 0.3, 0.3));

                if arena_btn.is_some() {
                    // Циклируем через предустановленные цвета арены
                    let current_hue = settings.arena_h;
                    if (current_hue - 180.0).abs() < 1.0 {
                        // Голубой -> Желтый
                        settings.arena_h = 60.0;
                        settings.arena_s = 1.0;
                        settings.arena_l = 0.5;
                    } else if (current_hue - 60.0).abs() < 1.0 {
                        // Желтый -> Пурпурный
                        settings.arena_h = 300.0;
                        settings.arena_s = 1.0;
                        settings.arena_l = 0.5;
                    } else {
                        // Пурпурный -> Голубой
                        settings.arena_h = 180.0;
                        settings.arena_s = 1.0;
                        settings.arena_l = 0.5;
                    }
                }

                if target_btn.is_some() {
                    // Циклируем через предустановленные цвета цели
                    let current_hue = settings.target_h;
                    if current_hue < 1.0 {
                        // Красный -> Зеленый
                        settings.target_h = 120.0;
                        settings.target_s = 1.0;
                        settings.target_l = 0.5;
                    } else if (current_hue - 120.0).abs() < 1.0 {
                        // Зеленый -> Синий
                        settings.target_h = 240.0;
                        settings.target_s = 1.0;
                        settings.target_l = 0.5;
                    } else {
                        // Синий -> Красный
                        settings.target_h = 0.0;
                        settings.target_s = 1.0;
                        settings.target_l = 0.5;
                    }
                }

                if back_btn.is_some() {
                    next_state.set(AppState::Game);
                }

                if start_btn.is_some() {
                    next_state.set(AppState::Game);
                }

                if discord_link.is_some() {
                    // Open Discord link in default browser
                    let _ = std::process::Command::new("cmd")
                        .args(&["/C", "start", "https://discord.gg/rWH3BJAcED"])
                        .spawn();
                }
            }
            Interaction::Hovered => {
                if discord_link.is_some() {
                    *bg = BackgroundColor(Color::srgb(0.2, 0.2, 0.5));
                } else {
                    *bg = BackgroundColor(Color::srgb(0.35, 0.35, 0.35));
                }
            }
            Interaction::None => {
                if discord_link.is_some() {
                    *bg = BackgroundColor(Color::srgb(0.1, 0.1, 0.3));
                } else {
                    *bg = BackgroundColor(Color::srgb(0.2, 0.2, 0.2));
                }
            }
        }
    }
}
