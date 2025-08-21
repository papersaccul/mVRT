use crate::state::*;
use crate::ui_components::*;
use crate::user_interface::ui::{ModernButton, UI_COLORS};
use bevy::prelude::*;

pub fn setup_settings_ui(
    mut commands: Commands,
    settings: Res<Settings>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((
        Camera2d,
        Camera {
            order: 100,
            clear_color: ClearColorConfig::None,
            ..default()
        },
        SettingsUI,
    ));

    // Main container
    let main_container = commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                padding: UiRect::all(Val::Px(40.0)),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceBetween,
                row_gap: Val::Px(24.0),
                ..default()
            },
            BackgroundColor(UI_COLORS.background),
            SettingsUI,
        ))
        .id();

    // Top controls
    spawn_top_controls(
        &mut commands,
        main_container,
        &asset_server,
        &settings.font_file,
    );

    // Main title
    spawn_main_title(&mut commands, main_container, &asset_server, &settings);

    // Settings title
    spawn_settings_title(&mut commands, main_container, &asset_server, &settings);

    // Cards container
    spawn_settings_cards_container(&mut commands, main_container, &asset_server, &settings);

    // Discord link
    spawn_discord_link(&mut commands, main_container, &asset_server, &settings);
}

fn spawn_main_title(
    commands: &mut Commands,
    parent: Entity,
    asset_server: &AssetServer,
    settings: &Settings,
) {
    commands
        .spawn((
            Text::new(format!(
                "MODERN REACTION TEST - beta {}",
                env!("CARGO_PKG_VERSION")
            )),
            TextFont {
                font: asset_server.load(&settings.font_file),
                font_size: 42.0,
                ..default()
            },
            TextColor(UI_COLORS.text_primary),
            Node {
                margin: UiRect::bottom(Val::Px(24.0)),
                ..default()
            },
        ))
        .set_parent(parent);
}

fn spawn_settings_title(
    commands: &mut Commands,
    parent: Entity,
    asset_server: &AssetServer,
    settings: &Settings,
) {
    commands
        .spawn((
            Text::new("SETTINGS"),
            TextFont {
                font: asset_server.load(&settings.font_file),
                font_size: 38.0,
                ..default()
            },
            TextColor(UI_COLORS.text_primary),
            Node {
                margin: UiRect::bottom(Val::Px(34.0)),
                ..default()
            },
        ))
        .set_parent(parent);
}

fn spawn_settings_cards_container(
    commands: &mut Commands,
    parent: Entity,
    asset_server: &AssetServer,
    settings: &Settings,
) {
    let container = commands
        .spawn((Node {
            width: Val::Percent(100.0),
            flex_direction: FlexDirection::Row,
            flex_wrap: FlexWrap::Wrap,
            column_gap: Val::Px(16.0),
            row_gap: Val::Px(16.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Start,
            ..default()
        },))
        .set_parent(parent)
        .id();

    spawn_mouse_sensitivity_card(commands, container, asset_server, settings);
    spawn_arena_settings_card(commands, container, asset_server, settings);
    spawn_target_settings_card(commands, container, asset_server, settings);
    spawn_lighting_settings_card(commands, container, asset_server, settings);
}

fn spawn_mouse_sensitivity_card(
    commands: &mut Commands,
    parent: Entity,
    asset_server: &AssetServer,
    settings: &Settings,
) {
    let card_builder = CardBuilder::new(asset_server, &settings.font_file);
    let card = card_builder.spawn_settings_card(commands, parent, "Mouse Sensitivity");

    spawn_section_header(
        commands,
        card,
        asset_server,
        &settings.font_file,
        "Mouse Sensitivity",
    );

    let input_builder = InputRowBuilder::new(card, asset_server, &settings.font_file);

    input_builder.spawn_numeric_input_row(
        commands,
        "DPI:",
        settings.dpi,
        0,
        DpiInput,
        DpiBuffer(format!("{:.0}", settings.dpi)),
    );

    input_builder.spawn_numeric_input_row(
        commands,
        "CM/360:",
        settings.cm_360,
        2,
        Cm360Input,
        CmBuffer(format!("{:.2}", settings.cm_360)),
    );

    input_builder.spawn_numeric_input_row(
        commands,
        "FOV:",
        settings.fov,
        0,
        FovInput,
        FovBuffer(format!("{:.0}", settings.fov)),
    );
}

fn spawn_arena_settings_card(
    commands: &mut Commands,
    parent: Entity,
    asset_server: &AssetServer,
    settings: &Settings,
) {
    let card_builder = CardBuilder::new(asset_server, &settings.font_file);
    let card = card_builder.spawn_settings_card(commands, parent, "Arena Settings");

    spawn_section_header(
        commands,
        card,
        asset_server,
        &settings.font_file,
        "Arena Settings",
    );

    let input_builder = InputRowBuilder::new(card, asset_server, &settings.font_file);

    input_builder.spawn_color_picker_row(
        commands,
        "Arena Color:",
        settings.arena_color(),
        ArenaColorPicker,
    );
}

fn spawn_target_settings_card(
    commands: &mut Commands,
    parent: Entity,
    asset_server: &AssetServer,
    settings: &Settings,
) {
    let card_builder = CardBuilder::new(asset_server, &settings.font_file);
    let card = card_builder.spawn_settings_card(commands, parent, "Target Settings");

    spawn_section_header(
        commands,
        card,
        asset_server,
        &settings.font_file,
        "Target Settings",
    );

    let input_builder = InputRowBuilder::new(card, asset_server, &settings.font_file);

    input_builder.spawn_color_picker_row(
        commands,
        "Target Color:",
        settings.target_color(),
        TargetColorPicker,
    );

    input_builder.spawn_checkbox_row(
        commands,
        "Enable Fresnel:",
        settings.fresnel_enabled,
        FresnelEnabledCheckbox,
    );

    input_builder.spawn_numeric_input_row(
        commands,
        "Fresnel Intensity:",
        settings.fresnel_intensity,
        1,
        FresnelIntensityInput,
        FresnelIntensityBuffer(format!("{:.1}", settings.fresnel_intensity)),
    );

    input_builder.spawn_numeric_input_row(
        commands,
        "Fresnel Power:",
        settings.fresnel_power,
        1,
        FresnelPowerInput,
        FresnelPowerBuffer(format!("{:.1}", settings.fresnel_power)),
    );

    input_builder.spawn_color_picker_row(
        commands,
        "Fresnel Color:",
        settings.fresnel_color(),
        FresnelColorPicker,
    );
}

fn spawn_lighting_settings_card(
    commands: &mut Commands,
    parent: Entity,
    asset_server: &AssetServer,
    settings: &Settings,
) {
    let card_builder = CardBuilder::new(asset_server, &settings.font_file);
    let card = card_builder.spawn_settings_card(commands, parent, "Lighting Settings");

    spawn_section_header(
        commands,
        card,
        asset_server,
        &settings.font_file,
        "Lighting Settings",
    );

    let input_builder = InputRowBuilder::new(card, asset_server, &settings.font_file);

    input_builder.spawn_numeric_input_row(
        commands,
        "Directional Light:",
        settings.directional_light_illuminance,
        0,
        DirectionalLightInput,
        DirectionalLightBuffer(format!("{:.0}", settings.directional_light_illuminance)),
    );

    input_builder.spawn_numeric_input_row(
        commands,
        "Ambient Light:",
        settings.ambient_light_brightness,
        0,
        AmbientLightInput,
        AmbientLightBuffer(format!("{:.0}", settings.ambient_light_brightness)),
    );
}

fn spawn_discord_link(
    commands: &mut Commands,
    parent: Entity,
    asset_server: &AssetServer,
    settings: &Settings,
) {
    let row = commands
        .spawn((Node {
            width: Val::Percent(100.0),
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            margin: UiRect::all(Val::Px(8.0)),
            ..default()
        },))
        .set_parent(parent)
        .id();

    let link_button = commands
        .spawn((
            Node {
                padding: UiRect::all(Val::Px(12.0)),
                ..default()
            },
            BackgroundColor(UI_COLORS.secondary),
            BorderRadius::all(Val::Px(12.0)),
            DiscordLink,
            Interaction::default(),
        ))
        .set_parent(row)
        .id();

    commands
        .spawn((
            Text::new("Join CisA community in Discord"),
            TextFont {
                font: asset_server.load(&settings.font_file),
                font_size: 16.0,
                ..default()
            },
            TextColor(UI_COLORS.text_primary),
        ))
        .set_parent(link_button);
}

pub fn cleanup_settings_ui(mut commands: Commands, query: Query<Entity, With<SettingsUI>>) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn settings_input_system(
    keys: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    if keys.just_pressed(KeyCode::Escape) {
        next_state.set(AppState::Game);
    }
}

pub fn update_settings_text(
    settings: Res<Settings>,
    mut dpi_query: Query<&mut Text, (With<DpiText>, Without<CmText>, Without<SettingsInfoText>)>,
    mut cm_query: Query<&mut Text, (With<CmText>, Without<DpiText>, Without<SettingsInfoText>)>,
    mut fov_query: Query<
        &mut Text,
        (
            With<FovText>,
            Without<DpiText>,
            Without<CmText>,
            Without<SettingsInfoText>,
        ),
    >,
    mut info_query: Query<&mut Text, (With<SettingsInfoText>, Without<DpiText>, Without<CmText>)>,
    dpi_editing: Query<(), With<DpiEditing>>,
    cm_editing: Query<(), With<CmEditing>>,
    fov_editing: Query<(), With<FovEditing>>,
    directional_light_editing: Query<(), With<DirectionalLightEditing>>,
    ambient_light_editing: Query<(), With<AmbientLightEditing>>,
    mut directional_light_query: Query<
        &mut Text,
        (
            With<DirectionalLightBuffer>,
            Without<DpiText>,
            Without<CmText>,
            Without<FovText>,
            Without<SettingsInfoText>,
        ),
    >,
    mut ambient_light_query: Query<
        &mut Text,
        (
            With<AmbientLightBuffer>,
            Without<DpiText>,
            Without<CmText>,
            Without<FovText>,
            Without<DirectionalLightBuffer>,
            Without<SettingsInfoText>,
        ),
    >,
) {
    // Update DPI text
    if dpi_editing.is_empty() {
        for mut text in dpi_query.iter_mut() {
            text.0 = format!("{:.0}", settings.dpi);
        }
    }

    // Update CM/360 text
    if cm_editing.is_empty() {
        for mut text in cm_query.iter_mut() {
            text.0 = format!("{:.2}", settings.cm_360);
        }
    }
    // Update FOV text
    if fov_editing.is_empty() {
        for mut text in fov_query.iter_mut() {
            text.0 = format!("{:.0}", settings.fov);
        }
    }

    // Update Directional Light text
    if directional_light_editing.is_empty() {
        for mut text in directional_light_query.iter_mut() {
            text.0 = format!("{:.0}", settings.directional_light_illuminance);
        }
    }

    // Update Ambient Light text
    if ambient_light_editing.is_empty() {
        for mut text in ambient_light_query.iter_mut() {
            text.0 = format!("{:.0}", settings.ambient_light_brightness);
        }
    }

    // Update sensitivity info
    for mut text in info_query.iter_mut() {
        text.0 = format!(
            "Current sensitivity: {:.6} rad/pixel",
            settings.mouse_sensitivity()
        );
    }
}

pub fn update_fresnel_ui(
    settings: Res<Settings>,
    mut fresnel_checkbox_query: Query<
        (Entity, &mut BackgroundColor, &Children),
        With<FresnelEnabledCheckbox>,
    >,
    mut fresnel_color_query: Query<
        &mut BackgroundColor,
        (With<FresnelColorPicker>, Without<FresnelEnabledCheckbox>),
    >,
    mut fresnel_intensity_text_query: Query<
        &mut Text,
        (With<FresnelIntensityBuffer>, Without<FresnelPowerBuffer>),
    >,
    mut fresnel_power_text_query: Query<
        &mut Text,
        (With<FresnelPowerBuffer>, Without<FresnelIntensityBuffer>),
    >,
    mut text_query: Query<
        &mut Text,
        (Without<FresnelIntensityBuffer>, Without<FresnelPowerBuffer>),
    >,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    // Update checkbox appearance and checkmark
    if let Ok((entity, mut bg, children)) = fresnel_checkbox_query.get_single_mut() {
        *bg = BackgroundColor(if settings.fresnel_enabled {
            UI_COLORS.success
        } else {
            UI_COLORS.surface_light
        });
    }
}
