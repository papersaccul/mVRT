use crate::state::*;
use crate::ui_components::process_numeric_input;
use bevy::prelude::*;

// DPI input box system
pub fn dpi_input_box_system(
    mut settings: ResMut<Settings>,
    mut focus: ResMut<InputFocus>,
    mut dpi_nodes: Query<(Entity, &Interaction, &Children, Option<&DpiEditing>), With<DpiInput>>,
    mut dpi_texts: Query<(&mut Text, &mut DpiBuffer), Without<CmBuffer>>,
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
) {
    for (entity, interaction, children, editing) in dpi_nodes.iter_mut() {
        if *interaction == Interaction::Pressed {
            focus.focused = Some(InputField::Dpi);
            commands.entity(entity).insert(DpiEditing);
        }
        if *interaction == Interaction::None
            && keys.just_pressed(KeyCode::Escape)
            && editing.is_some()
        {
            commands.entity(entity).remove::<DpiEditing>();
            if let Some(&child) = children.first() {
                if let Ok((mut text, _buf)) = dpi_texts.get_mut(child) {
                    text.0 = format!("{:.0}", settings.dpi);
                }
            }
            if matches!(focus.focused, Some(InputField::Dpi)) {
                focus.focused = None;
            }
        }
        if editing.is_some() && matches!(focus.focused, Some(InputField::Dpi)) {
            if let Some(&child) = children.first() {
                if let Ok((mut text, mut buf)) = dpi_texts.get_mut(child) {
                    process_numeric_input(&keys, &mut buf.0, true);
                    if let Ok(val) = buf.0.parse::<f32>() {
                        settings.dpi = val.clamp(100.0, 20000.0);
                    }
                    text.0 = format!("{}|", buf.0);
                }
            }
        }
    }
}

// CM/360 input box system
pub fn cm_input_box_system(
    mut settings: ResMut<Settings>,
    mut focus: ResMut<InputFocus>,
    mut cm_nodes: Query<(Entity, &Interaction, &Children, Option<&CmEditing>), With<Cm360Input>>,
    mut cm_texts: Query<(&mut Text, &mut CmBuffer), Without<DpiBuffer>>,
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
) {
    for (entity, interaction, children, editing) in cm_nodes.iter_mut() {
        if *interaction == Interaction::Pressed {
            focus.focused = Some(InputField::Cm);
            commands.entity(entity).insert(CmEditing);
        }
        if *interaction == Interaction::None
            && keys.just_pressed(KeyCode::Escape)
            && editing.is_some()
        {
            commands.entity(entity).remove::<CmEditing>();
            if let Some(&child) = children.first() {
                if let Ok((mut text, _buf)) = cm_texts.get_mut(child) {
                    text.0 = format!("{:.2}", settings.cm_360);
                }
            }
            if matches!(focus.focused, Some(InputField::Cm)) {
                focus.focused = None;
            }
        }
        if editing.is_some() && matches!(focus.focused, Some(InputField::Cm)) {
            if let Some(&child) = children.first() {
                if let Ok((mut text, mut buf)) = cm_texts.get_mut(child) {
                    process_numeric_input(&keys, &mut buf.0, true);
                    if let Ok(val) = buf.0.parse::<f32>() {
                        settings.cm_360 = val.clamp(0.1, 300.0);
                    }
                    text.0 = format!("{}|", buf.0);
                }
            }
        }
    }
}

// FOV input box system
pub fn fov_input_box_system(
    mut settings: ResMut<Settings>,
    mut focus: ResMut<InputFocus>,
    mut fov_nodes: Query<(Entity, &Interaction, &Children, Option<&FovEditing>), With<FovInput>>,
    mut fov_texts: Query<(&mut Text, &mut FovBuffer), (Without<DpiBuffer>, Without<CmBuffer>)>,
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
) {
    for (entity, interaction, children, editing) in fov_nodes.iter_mut() {
        if *interaction == Interaction::Pressed {
            focus.focused = Some(InputField::Fov);
            commands.entity(entity).insert(FovEditing);
        }
        if *interaction == Interaction::None
            && keys.just_pressed(KeyCode::Escape)
            && editing.is_some()
        {
            commands.entity(entity).remove::<FovEditing>();
            if let Some(&child) = children.first() {
                if let Ok((mut text, _buf)) = fov_texts.get_mut(child) {
                    text.0 = format!("{:.0}", settings.fov);
                }
            }
            if matches!(focus.focused, Some(InputField::Fov)) {
                focus.focused = None;
            }
        }
        if editing.is_some() && matches!(focus.focused, Some(InputField::Fov)) {
            if let Some(&child) = children.first() {
                if let Ok((mut text, mut buf)) = fov_texts.get_mut(child) {
                    process_numeric_input(&keys, &mut buf.0, true);
                    if let Ok(val) = buf.0.parse::<f32>() {
                        settings.fov = val.clamp(50.0, 150.0);
                    }
                    text.0 = format!("{}|", buf.0);
                }
            }
        }
    }
}

// Fresnel Intensity input box system
pub fn fresnel_intensity_input_box_system(
    mut settings: ResMut<Settings>,
    mut focus: ResMut<InputFocus>,
    mut fresnel_intensity_nodes: Query<
        (
            Entity,
            &Interaction,
            &Children,
            Option<&FresnelIntensityEditing>,
        ),
        With<FresnelIntensityInput>,
    >,
    mut fresnel_intensity_texts: Query<
        (&mut Text, &mut FresnelIntensityBuffer),
        (Without<DpiBuffer>, Without<CmBuffer>, Without<FovBuffer>),
    >,
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
) {
    for (entity, interaction, children, editing) in fresnel_intensity_nodes.iter_mut() {
        if *interaction == Interaction::Pressed {
            focus.focused = Some(InputField::FresnelIntensity);
            commands.entity(entity).insert(FresnelIntensityEditing);
        }
        if *interaction == Interaction::None
            && keys.just_pressed(KeyCode::Escape)
            && editing.is_some()
        {
            commands.entity(entity).remove::<FresnelIntensityEditing>();
            if let Some(&child) = children.first() {
                if let Ok((mut text, _buf)) = fresnel_intensity_texts.get_mut(child) {
                    text.0 = format!("{:.1}", settings.fresnel_intensity);
                }
            }
            if matches!(focus.focused, Some(InputField::FresnelIntensity)) {
                focus.focused = None;
            }
        }
        if editing.is_some() && matches!(focus.focused, Some(InputField::FresnelIntensity)) {
            if let Some(&child) = children.first() {
                if let Ok((mut text, mut buf)) = fresnel_intensity_texts.get_mut(child) {
                    process_numeric_input(&keys, &mut buf.0, true);
                    if let Ok(val) = buf.0.parse::<f32>() {
                        settings.fresnel_intensity = val.clamp(0.0, 5.0);
                    }
                    text.0 = format!("{}|", buf.0);
                }
            }
        }
    }
}

// Fresnel Power input box system
pub fn fresnel_power_input_box_system(
    mut settings: ResMut<Settings>,
    mut focus: ResMut<InputFocus>,
    mut fresnel_power_nodes: Query<
        (
            Entity,
            &Interaction,
            &Children,
            Option<&FresnelPowerEditing>,
        ),
        With<FresnelPowerInput>,
    >,
    mut fresnel_power_texts: Query<
        (&mut Text, &mut FresnelPowerBuffer),
        (
            Without<DpiBuffer>,
            Without<CmBuffer>,
            Without<FovBuffer>,
            Without<FresnelIntensityBuffer>,
        ),
    >,
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
) {
    for (entity, interaction, children, editing) in fresnel_power_nodes.iter_mut() {
        if *interaction == Interaction::Pressed {
            focus.focused = Some(InputField::FresnelPower);
            commands.entity(entity).insert(FresnelPowerEditing);
        }
        if *interaction == Interaction::None
            && keys.just_pressed(KeyCode::Escape)
            && editing.is_some()
        {
            commands.entity(entity).remove::<FresnelPowerEditing>();
            if let Some(&child) = children.first() {
                if let Ok((mut text, _buf)) = fresnel_power_texts.get_mut(child) {
                    text.0 = format!("{:.1}", settings.fresnel_power);
                }
            }
            if matches!(focus.focused, Some(InputField::FresnelPower)) {
                focus.focused = None;
            }
        }
        if editing.is_some() && matches!(focus.focused, Some(InputField::FresnelPower)) {
            if let Some(&child) = children.first() {
                if let Ok((mut text, mut buf)) = fresnel_power_texts.get_mut(child) {
                    process_numeric_input(&keys, &mut buf.0, true);
                    if let Ok(val) = buf.0.parse::<f32>() {
                        settings.fresnel_power = val.clamp(0.1, 10.0);
                    }
                    text.0 = format!("{}|", buf.0);
                }
            }
        }
    }
}

// Directional Light input box system
pub fn directional_light_input_box_system(
    mut settings: ResMut<Settings>,
    mut focus: ResMut<InputFocus>,
    mut directional_light_nodes: Query<
        (
            Entity,
            &Interaction,
            &Children,
            Option<&DirectionalLightEditing>,
        ),
        With<DirectionalLightInput>,
    >,
    mut directional_light_texts: Query<
        (&mut Text, &mut DirectionalLightBuffer),
        (Without<DpiBuffer>, Without<CmBuffer>, Without<FovBuffer>),
    >,
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
) {
    for (entity, interaction, children, editing) in directional_light_nodes.iter_mut() {
        if *interaction == Interaction::Pressed {
            focus.focused = Some(InputField::DirectionalLight);
            commands.entity(entity).insert(DirectionalLightEditing);
        }
        if *interaction == Interaction::None
            && keys.just_pressed(KeyCode::Escape)
            && editing.is_some()
        {
            commands.entity(entity).remove::<DirectionalLightEditing>();
            if let Some(&child) = children.first() {
                if let Ok((mut text, _buf)) = directional_light_texts.get_mut(child) {
                    text.0 = format!("{:.1}", settings.directional_light_illuminance);
                }
            }
            if matches!(focus.focused, Some(InputField::DirectionalLight)) {
                focus.focused = None;
            }
        }
        if editing.is_some() && matches!(focus.focused, Some(InputField::DirectionalLight)) {
            if let Some(&child) = children.first() {
                if let Ok((mut text, mut buf)) = directional_light_texts.get_mut(child) {
                    process_numeric_input(&keys, &mut buf.0, true);
                    if let Ok(val) = buf.0.parse::<f32>() {
                        settings.directional_light_illuminance = val.clamp(-1000.0, 10000.0);
                    }
                    text.0 = format!("{}|", buf.0);
                }
            }
        }
    }
}

// Ambient Light input box system
pub fn ambient_light_input_box_system(
    mut settings: ResMut<Settings>,
    mut focus: ResMut<InputFocus>,
    mut ambient_light_nodes: Query<
        (
            Entity,
            &Interaction,
            &Children,
            Option<&AmbientLightEditing>,
        ),
        With<AmbientLightInput>,
    >,
    mut ambient_light_texts: Query<
        (&mut Text, &mut AmbientLightBuffer),
        (Without<DpiBuffer>, Without<CmBuffer>, Without<FovBuffer>),
    >,
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
) {
    for (entity, interaction, children, editing) in ambient_light_nodes.iter_mut() {
        if *interaction == Interaction::Pressed {
            focus.focused = Some(InputField::AmbientLight);
            commands.entity(entity).insert(AmbientLightEditing);
        }
        if *interaction == Interaction::None
            && keys.just_pressed(KeyCode::Escape)
            && editing.is_some()
        {
            commands.entity(entity).remove::<AmbientLightEditing>();
            if let Some(&child) = children.first() {
                if let Ok((mut text, _buf)) = ambient_light_texts.get_mut(child) {
                    text.0 = format!("{:.0}", settings.ambient_light_brightness);
                }
            }
            if matches!(focus.focused, Some(InputField::AmbientLight)) {
                focus.focused = None;
            }
        }
        if editing.is_some() && matches!(focus.focused, Some(InputField::AmbientLight)) {
            if let Some(&child) = children.first() {
                if let Ok((mut text, mut buf)) = ambient_light_texts.get_mut(child) {
                    process_numeric_input(&keys, &mut buf.0, false);
                    if let Ok(val) = buf.0.parse::<f32>() {
                        settings.ambient_light_brightness = val.clamp(0.0, 10000.0);
                    }
                    text.0 = format!("{}|", buf.0);
                }
            }
        }
    }
}

// System to clear other editing states when one input field gains focus
pub fn clear_other_editing_states_system(
    mut commands: Commands,
    mut dpi_nodes: Query<(Entity, Option<&DpiEditing>, &Children), With<DpiInput>>,
    mut cm_nodes: Query<(Entity, Option<&CmEditing>, &Children), With<Cm360Input>>,
    mut fov_nodes: Query<(Entity, Option<&FovEditing>, &Children), With<FovInput>>,
    mut fresnel_intensity_nodes: Query<
        (Entity, Option<&FresnelIntensityEditing>, &Children),
        With<FresnelIntensityInput>,
    >,
    mut fresnel_power_nodes: Query<
        (Entity, Option<&FresnelPowerEditing>, &Children),
        With<FresnelPowerInput>,
    >,
    mut directional_light_nodes: Query<
        (Entity, Option<&DirectionalLightEditing>, &Children),
        With<DirectionalLightInput>,
    >,
    mut ambient_light_nodes: Query<
        (Entity, Option<&AmbientLightEditing>, &Children),
        With<AmbientLightInput>,
    >,
    mut all_texts: Query<(
        &mut Text,
        Option<&DpiBuffer>,
        Option<&CmBuffer>,
        Option<&FovBuffer>,
        Option<&FresnelIntensityBuffer>,
        Option<&FresnelPowerBuffer>,
        Option<&DirectionalLightBuffer>,
        Option<&AmbientLightBuffer>,
    )>,
    settings: Res<Settings>,
    focus: Res<InputFocus>,
) {
    // Clear DPI editing state (if not focused)
    for (entity, editing, children) in dpi_nodes.iter_mut() {
        if editing.is_some() && !matches!(focus.focused, Some(InputField::Dpi)) {
            commands.entity(entity).remove::<DpiEditing>();
            if let Some(&child) = children.first() {
                if let Ok((mut text, dpi_buf, _, _, _, _, _, _)) = all_texts.get_mut(child) {
                    if dpi_buf.is_some() {
                        text.0 = format!("{:.0}", settings.dpi);
                    }
                }
            }
        }
    }

    // Clear CM editing state (if not focused)
    for (entity, editing, children) in cm_nodes.iter_mut() {
        if editing.is_some() && !matches!(focus.focused, Some(InputField::Cm)) {
            commands.entity(entity).remove::<CmEditing>();
            if let Some(&child) = children.first() {
                if let Ok((mut text, _, cm_buf, _, _, _, _, _)) = all_texts.get_mut(child) {
                    if cm_buf.is_some() {
                        text.0 = format!("{:.2}", settings.cm_360);
                    }
                }
            }
        }
    }

    // Clear FOV editing state (if not focused)
    for (entity, editing, children) in fov_nodes.iter_mut() {
        if editing.is_some() && !matches!(focus.focused, Some(InputField::Fov)) {
            commands.entity(entity).remove::<FovEditing>();
            if let Some(&child) = children.first() {
                if let Ok((mut text, _, _, fov_buf, _, _, _, _)) = all_texts.get_mut(child) {
                    if fov_buf.is_some() {
                        text.0 = format!("{:.0}", settings.fov);
                    }
                }
            }
        }
    }

    // Clear Fresnel Intensity editing state (if not focused)
    for (entity, editing, children) in fresnel_intensity_nodes.iter_mut() {
        if editing.is_some() && !matches!(focus.focused, Some(InputField::FresnelIntensity)) {
            commands.entity(entity).remove::<FresnelIntensityEditing>();
            if let Some(&child) = children.first() {
                if let Ok((mut text, _, _, _, fresnel_int_buf, _, _, _)) = all_texts.get_mut(child)
                {
                    if fresnel_int_buf.is_some() {
                        text.0 = format!("{:.2}", settings.fresnel_intensity);
                    }
                }
            }
        }
    }

    // Clear Fresnel Power editing state (if not focused)
    for (entity, editing, children) in fresnel_power_nodes.iter_mut() {
        if editing.is_some() && !matches!(focus.focused, Some(InputField::FresnelPower)) {
            commands.entity(entity).remove::<FresnelPowerEditing>();
            if let Some(&child) = children.first() {
                if let Ok((mut text, _, _, _, _, fresnel_pow_buf, _, _)) = all_texts.get_mut(child)
                {
                    if fresnel_pow_buf.is_some() {
                        text.0 = format!("{:.2}", settings.fresnel_power);
                    }
                }
            }
        }
    }

    // Clear Directional Light editing state (if not focused)
    for (entity, editing, children) in directional_light_nodes.iter_mut() {
        if editing.is_some() && !matches!(focus.focused, Some(InputField::DirectionalLight)) {
            commands.entity(entity).remove::<DirectionalLightEditing>();
            if let Some(&child) = children.first() {
                if let Ok((mut text, _, _, _, _, _, dir_light_buf, _)) = all_texts.get_mut(child) {
                    if dir_light_buf.is_some() {
                        text.0 = format!("{:.1}", settings.directional_light_illuminance);
                    }
                }
            }
        }
    }

    // Clear Ambient Light editing state (if not focused)
    for (entity, editing, children) in ambient_light_nodes.iter_mut() {
        if editing.is_some() && !matches!(focus.focused, Some(InputField::AmbientLight)) {
            commands.entity(entity).remove::<AmbientLightEditing>();
            if let Some(&child) = children.first() {
                if let Ok((mut text, _, _, _, _, _, _, amb_light_buf)) = all_texts.get_mut(child) {
                    if amb_light_buf.is_some() {
                        text.0 = format!("{:.0}", settings.ambient_light_brightness);
                    }
                }
            }
        }
    }
}
