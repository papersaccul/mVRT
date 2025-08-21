use crate::state::*;
use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;

pub fn settings_slider_system(
    mut settings: ResMut<Settings>,
    mut arena_h_bar: Query<(&Interaction, &Children), (With<ArenaHueSlider>, Changed<Interaction>)>,
    mut arena_s_bar: Query<
        (&Interaction, &Children),
        (With<ArenaSaturationSlider>, Changed<Interaction>),
    >,
    mut arena_l_bar: Query<
        (&Interaction, &Children),
        (With<ArenaLightnessSlider>, Changed<Interaction>),
    >,
    mut target_h_bar: Query<
        (&Interaction, &Children),
        (With<TargetHueSlider>, Changed<Interaction>),
    >,
    mut target_s_bar: Query<
        (&Interaction, &Children),
        (With<TargetSaturationSlider>, Changed<Interaction>),
    >,
    mut target_l_bar: Query<
        (&Interaction, &Children),
        (With<TargetLightnessSlider>, Changed<Interaction>),
    >,
    mut fills: Query<
        &mut Node,
        Or<(
            With<ArenaHueFill>,
            With<ArenaSaturationFill>,
            With<ArenaLightnessFill>,
            With<TargetHueFill>,
            With<TargetSaturationFill>,
            With<TargetLightnessFill>,
        )>,
    >,
    mut mouse_motion: EventReader<MouseMotion>,
    mouse: Res<ButtonInput<MouseButton>>,
) {
    // accumulate horizontal mouse delta this frame
    let mut dx = 0.0f32;
    for m in mouse_motion.read() {
        dx += m.delta.x;
    }

    let scale = 1.0 / 200.0; // 200 px full range

    let mut apply_hsl = |val: &mut f32, children: &Children, is_hue: bool| {
        if mouse.pressed(MouseButton::Left) && dx != 0.0 {
            if is_hue {
                // Hue циклический (0-360)
                *val = (*val + dx * scale * 360.0) % 360.0;
                if *val < 0.0 {
                    *val += 360.0;
                }
            } else {
                // Saturation и Lightness (0-1)
                *val = (*val + dx * scale).clamp(0.0, 1.0);
            }

            if let Some(&child) = children.first() {
                if let Ok(mut node) = fills.get_mut(child) {
                    let width = if is_hue {
                        200.0 * (*val / 360.0)
                    } else {
                        200.0 * *val
                    };
                    node.width = Val::Px(width);
                }
            }
        }
    };

    // Arena HSL sliders
    for (i, children) in arena_h_bar.iter_mut() {
        if *i == Interaction::Pressed {
            apply_hsl(&mut settings.arena_h, &children, true);
        }
    }

    for (i, children) in arena_s_bar.iter_mut() {
        if *i == Interaction::Pressed {
            apply_hsl(&mut settings.arena_s, &children, false);
        }
    }

    for (i, children) in arena_l_bar.iter_mut() {
        if *i == Interaction::Pressed {
            apply_hsl(&mut settings.arena_l, &children, false);
        }
    }

    // Target HSL sliders
    for (i, children) in target_h_bar.iter_mut() {
        if *i == Interaction::Pressed {
            apply_hsl(&mut settings.target_h, &children, true);
        }
    }

    for (i, children) in target_s_bar.iter_mut() {
        if *i == Interaction::Pressed {
            apply_hsl(&mut settings.target_s, &children, false);
        }
    }

    for (i, children) in target_l_bar.iter_mut() {
        if *i == Interaction::Pressed {
            apply_hsl(&mut settings.target_l, &children, false);
        }
    }
}
