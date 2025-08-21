use crate::state::*;
use crate::user_interface::ui::{ModernButton, UI_COLORS};
use crate::user_interface::ui_components::*;
use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;

// Система для обработки кликов по цветовым пикерам
pub fn handle_color_picker_clicks(
    mut interaction_query: Query<
        (
            &Interaction,
            Option<&ArenaColorPicker>,
            Option<&TargetColorPicker>,
            Option<&FresnelColorPicker>,
        ),
        Changed<Interaction>,
    >,
    mut settings: ResMut<Settings>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    for (interaction, arena_picker, target_picker, fresnel_picker) in interaction_query.iter_mut() {
        if *interaction == Interaction::Pressed {
            if arena_picker.is_some() {
                settings.picker_target = Some(ColorTarget::Arena);
                let arena_hsv = Hsva::from(settings.arena_color());
                settings.picker_hue = arena_hsv.hue;
                settings.picker_saturation = arena_hsv.saturation;
                settings.picker_brightness = arena_hsv.value;
                settings.color_picker_open = true;
                spawn_color_picker(&mut commands, &asset_server, &settings);
            } else if target_picker.is_some() {
                settings.picker_target = Some(ColorTarget::Target);
                let target_hsv = Hsva::from(settings.target_color());
                settings.picker_hue = target_hsv.hue;
                settings.picker_saturation = target_hsv.saturation;
                settings.picker_brightness = target_hsv.value;
                settings.color_picker_open = true;
                spawn_color_picker(&mut commands, &asset_server, &settings);
            } else if fresnel_picker.is_some() {
                settings.picker_target = Some(ColorTarget::Fresnel);
                let fresnel_hsv = Hsva::from(settings.fresnel_color());
                settings.picker_hue = fresnel_hsv.hue;
                settings.picker_saturation = fresnel_hsv.saturation;
                settings.picker_brightness = fresnel_hsv.value;
                settings.color_picker_open = true;
                spawn_color_picker(&mut commands, &asset_server, &settings);
            }
        }
    }
}

pub fn handle_color_picker_buttons(
    mut settings: ResMut<Settings>,
    mut commands: Commands,
    mut apply_query: Query<&Interaction, (With<ApplyColorPicker>, Changed<Interaction>)>,
    mut close_query: Query<&Interaction, (With<CloseColorPicker>, Changed<Interaction>)>,
    picker_entities: Query<Entity, Or<(With<ColorPickerOverlay>, With<ColorPickerWindow>)>>,
    mut arena_picker_query: Query<
        &mut BackgroundColor,
        (With<ArenaColorPicker>, Without<ColorDisplay>),
    >,
    mut target_picker_query: Query<
        &mut BackgroundColor,
        (
            With<TargetColorPicker>,
            Without<ColorDisplay>,
            Without<ArenaColorPicker>,
            Without<FresnelColorPicker>,
        ),
    >,
    mut fresnel_picker_query: Query<
        &mut BackgroundColor,
        (
            With<FresnelColorPicker>,
            Without<ColorDisplay>,
            Without<ArenaColorPicker>,
            Without<TargetColorPicker>,
        ),
    >,
) {
    // Handle Apply button
    for interaction in apply_query.iter() {
        if *interaction == Interaction::Pressed {
            // Создаем цвет из текущих значений picker'а (HSV)
            let current_color = Color::from(Hsva {
                hue: settings.picker_hue,
                saturation: settings.picker_saturation,
                value: settings.picker_brightness,
                alpha: 1.0,
            });

            if let Some(target) = settings.picker_target {
                match target {
                    ColorTarget::Arena => {
                        // Обновляем HSL компоненты арены
                        settings.set_arena_color(current_color);
                        if let Ok(mut bg) = arena_picker_query.single_mut() {
                            *bg = BackgroundColor(current_color);
                        }
                    }
                    ColorTarget::Target => {
                        // Обновляем HSL компоненты цели
                        settings.set_target_color(current_color);
                        if let Ok(mut bg) = target_picker_query.single_mut() {
                            *bg = BackgroundColor(current_color);
                        }
                    }
                    ColorTarget::Fresnel => {
                        // Обновляем HSL компоненты Fresnel
                        settings.set_fresnel_color(current_color);
                        if let Ok(mut bg) = fresnel_picker_query.single_mut() {
                            *bg = BackgroundColor(current_color);
                        }
                    }
                }
            }

            close_color_picker(&mut settings, &mut commands, &picker_entities);
        }
    }

    // Handle Close button
    for interaction in close_query.iter() {
        if *interaction == Interaction::Pressed {
            close_color_picker(&mut settings, &mut commands, &picker_entities);
        }
    }
}

pub fn handle_color_picker_sliders(
    mut settings: ResMut<Settings>,
    hue_bar: Query<&Interaction, With<HueBar>>,
    sat_bar: Query<&Interaction, With<SaturationBar>>,
    bright_bar: Query<&Interaction, With<BrightnessBar>>,
    mouse: Res<ButtonInput<MouseButton>>,
    mut mouse_motion: EventReader<MouseMotion>,
) {
    if !settings.color_picker_open {
        return;
    }

    let mut mouse_delta = 0.0;
    for motion in mouse_motion.read() {
        mouse_delta += motion.delta.x;
    }

    if mouse.pressed(MouseButton::Left) && mouse_delta.abs() > 0.1 {
        let sensitivity = 1.0 / 300.0;

        // Hue slider
        if let Ok(interaction) = hue_bar.single() {
            if *interaction == Interaction::Pressed {
                settings.picker_hue =
                    (settings.picker_hue + mouse_delta * sensitivity * 360.0).rem_euclid(360.0);
            }
        }

        // Saturation slider
        if let Ok(interaction) = sat_bar.single() {
            if *interaction == Interaction::Pressed {
                settings.picker_saturation =
                    (settings.picker_saturation + mouse_delta * sensitivity).clamp(0.0, 1.0);
            }
        }

        // Brightness slider
        if let Ok(interaction) = bright_bar.single() {
            if *interaction == Interaction::Pressed {
                settings.picker_brightness =
                    (settings.picker_brightness + mouse_delta * sensitivity).clamp(0.0, 1.0);
            }
        }
    }
}

pub fn update_color_picker_display(
    settings: Res<Settings>,
    mut queries: ParamSet<(
        Query<&mut BackgroundColor, With<ColorDisplay>>,
        Query<(&mut BackgroundColor, &SaturationSegment), Without<BrightnessSegment>>,
        Query<(&mut BackgroundColor, &BrightnessSegment), Without<SaturationSegment>>,
    )>,
) {
    if !settings.color_picker_open {
        return;
    }

    let current_color = Color::from(Hsva {
        hue: settings.picker_hue,
        saturation: settings.picker_saturation,
        value: settings.picker_brightness,
        alpha: 1.0,
    });

    // Update color display
    if let Ok(mut bg) = queries.p0().get_single_mut() {
        *bg = BackgroundColor(current_color);
    }

    // Update segmented saturation gradient: from s=0 to s=1 at current brightness
    for (mut bg, seg) in queries.p1().iter_mut() {
        let t = (seg.index as f32 + 0.5) / seg.count as f32;
        *bg = BackgroundColor(Color::from(Hsva {
            hue: settings.picker_hue,
            saturation: t,
            value: settings.picker_brightness,
            alpha: 1.0,
        }));
    }
    // Update segmented brightness gradient: from v=0 to v=1 at current saturation
    for (mut bg, seg) in queries.p2().iter_mut() {
        let t = (seg.index as f32 + 0.5) / seg.count as f32;
        *bg = BackgroundColor(Color::from(Hsva {
            hue: settings.picker_hue,
            saturation: settings.picker_saturation,
            value: t,
            alpha: 1.0,
        }));
    }
}

pub fn update_color_picker_handles(
    settings: Res<Settings>,
    mut hue_handle: Query<
        &mut Node,
        (
            With<HueHandle>,
            Without<SaturationHandle>,
            Without<BrightnessHandle>,
        ),
    >,
    mut sat_handle: Query<
        &mut Node,
        (
            With<SaturationHandle>,
            Without<HueHandle>,
            Without<BrightnessHandle>,
        ),
    >,
    mut bright_handle: Query<
        &mut Node,
        (
            With<BrightnessHandle>,
            Without<HueHandle>,
            Without<SaturationHandle>,
        ),
    >,
) {
    if !settings.color_picker_open {
        return;
    }

    if let Ok(mut handle) = hue_handle.single_mut() {
        handle.left = Val::Percent((settings.picker_hue / 360.0) * 100.0);
    }

    if let Ok(mut handle) = sat_handle.single_mut() {
        handle.left = Val::Percent(settings.picker_saturation * 100.0);
    }

    if let Ok(mut handle) = bright_handle.single_mut() {
        handle.left = Val::Percent(settings.picker_brightness * 100.0);
    }
}

pub fn handle_color_picker_escape(
    mut settings: ResMut<Settings>,
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    picker_entities: Query<Entity, Or<(With<ColorPickerOverlay>, With<ColorPickerWindow>)>>,
) {
    if settings.color_picker_open && keys.just_pressed(KeyCode::Escape) {
        close_color_picker(&mut settings, &mut commands, &picker_entities);
    }
}

fn close_color_picker(
    settings: &mut Settings,
    commands: &mut Commands,
    picker_entities: &Query<Entity, Or<(With<ColorPickerOverlay>, With<ColorPickerWindow>)>>,
) {
    settings.color_picker_open = false;
    settings.picker_target = None;

    for entity in picker_entities.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
