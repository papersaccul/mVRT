use crate::state::*;
use crate::ui::{ModernButton, UI_COLORS};
use bevy::input::ButtonInput;
use bevy::prelude::*;

// Константы для UI компонентов
const UI_PANEL_RADIUS: f32 = 16.0;
const UI_BUTTON_RADIUS: f32 = 8.0;
const UI_INPUT_HEIGHT: f32 = 40.0;
const UI_LABEL_WIDTH: f32 = 120.0;

// Компонент для отключения взаимодействий
#[derive(Component)]
pub struct DisabledInteraction;

pub struct CardBuilder {
    asset_server: AssetServer,
    font_file: String,
}

pub struct InputRowBuilder {
    parent: Entity,
    asset_server: AssetServer,
    font_file: String,
}

impl CardBuilder {
    pub fn new(asset_server: &AssetServer, font_file: &str) -> Self {
        Self {
            asset_server: asset_server.clone(),
            font_file: font_file.to_string(),
        }
    }

    pub fn spawn_settings_card(
        &self,
        commands: &mut Commands,
        parent: Entity,
        title: &str,
    ) -> Entity {
        commands
            .spawn((
                Node {
                    width: Val::Px(420.0),
                    padding: UiRect::all(Val::Px(20.0)),
                    margin: UiRect::all(Val::Px(8.0)),
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(16.0),
                    ..default()
                },
                BackgroundColor(UI_COLORS.surface_light),
                BorderRadius::all(Val::Px(16.0)),
            ))
            .set_parent(parent)
            .id()
    }
}

impl InputRowBuilder {
    pub fn new(parent: Entity, asset_server: &AssetServer, font_file: &str) -> Self {
        Self {
            parent,
            asset_server: asset_server.clone(),
            font_file: font_file.to_string(),
        }
    }

    pub fn spawn_numeric_input_row<T, B>(
        &self,
        commands: &mut Commands,
        label: &str,
        value: f32,
        decimal_places: u8, // Вместо format_str
        input_component: T,
        buffer_component: B,
    ) -> Entity
    where
        T: Component,
        B: Component,
    {
        let formatted_value = match decimal_places {
            0 => format!("{:.0}", value),
            1 => format!("{:.1}", value),
            2 => format!("{:.2}", value),
            _ => format!("{:.2}", value),
        };

        let row = commands
            .spawn((Node {
                flex_direction: FlexDirection::Row,
                column_gap: Val::Px(16.0),
                align_items: AlignItems::Center,
                ..default()
            },))
            .set_parent(self.parent)
            .id();

        // Label
        commands
            .spawn((
                Text::new(label),
                TextFont {
                    font: self.asset_server.load(&self.font_file),
                    font_size: 18.0,
                    ..default()
                },
                TextColor(UI_COLORS.text_secondary),
                Node {
                    width: Val::Px(120.0),
                    ..default()
                },
            ))
            .set_parent(row);

        // Input container
        let input_container = commands
            .spawn((
                Node {
                    width: Val::Px(150.0),
                    height: Val::Px(40.0),
                    padding: UiRect::all(Val::Px(8.0)),
                    ..default()
                },
                BackgroundColor(UI_COLORS.surface_light),
                BorderRadius::all(Val::Px(8.0)),
                Interaction::default(),
                input_component,
            ))
            .set_parent(row)
            .id();

        // Input text
        commands
            .spawn((
                Text::new(formatted_value),
                TextFont {
                    font: self.asset_server.load(&self.font_file),
                    font_size: 16.0,
                    ..default()
                },
                TextColor(UI_COLORS.text_primary),
                buffer_component,
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
            ))
            .set_parent(input_container);

        row
    }

    pub fn spawn_color_picker_row<T>(
        &self,
        commands: &mut Commands,
        label: &str,
        color: Color,
        picker_component: T,
    ) -> Entity
    where
        T: Component,
    {
        let row = commands
            .spawn((Node {
                flex_direction: FlexDirection::Row,
                column_gap: Val::Px(16.0),
                align_items: AlignItems::Center,
                ..default()
            },))
            .set_parent(self.parent)
            .id();

        // Label
        commands
            .spawn((
                Text::new(label),
                TextFont {
                    font: self.asset_server.load(&self.font_file),
                    font_size: 18.0,
                    ..default()
                },
                TextColor(UI_COLORS.text_secondary),
                Node {
                    width: Val::Px(120.0),
                    ..default()
                },
            ))
            .set_parent(row);

        // Color picker button
        commands
            .spawn((
                Node {
                    width: Val::Px(40.0),
                    height: Val::Px(40.0),
                    border: UiRect::all(Val::Px(2.0)),
                    ..default()
                },
                BackgroundColor(color),
                BorderColor(UI_COLORS.text_muted),
                BorderRadius::all(Val::Px(6.0)),
                picker_component,
                Interaction::default(),
            ))
            .set_parent(row);

        row
    }

    pub fn spawn_checkbox_row<T>(
        &self,
        commands: &mut Commands,
        label: &str,
        checked: bool,
        checkbox_component: T,
    ) -> Entity
    where
        T: Component,
    {
        let row = commands
            .spawn((Node {
                flex_direction: FlexDirection::Row,
                column_gap: Val::Px(16.0),
                align_items: AlignItems::Center,
                ..default()
            },))
            .set_parent(self.parent)
            .id();

        // Label
        commands
            .spawn((
                Text::new(label),
                TextFont {
                    font: self.asset_server.load(&self.font_file),
                    font_size: 18.0,
                    ..default()
                },
                TextColor(UI_COLORS.text_secondary),
                Node {
                    width: Val::Px(120.0),
                    ..default()
                },
            ))
            .set_parent(row);

        // Checkbox
        let checkbox = commands
            .spawn((
                Node {
                    width: Val::Px(24.0),
                    height: Val::Px(24.0),
                    border: UiRect::all(Val::Px(2.0)),
                    ..default()
                },
                BackgroundColor(if checked {
                    UI_COLORS.success
                } else {
                    UI_COLORS.surface_light
                }),
                BorderColor(UI_COLORS.text_muted),
                BorderRadius::all(Val::Px(4.0)),
                checkbox_component,
                Interaction::default(),
            ))
            .set_parent(row)
            .id();

        if checked {
            commands
                .spawn((
                    TextFont {
                        font: self.asset_server.load(&self.font_file),
                        font_size: 16.0,
                        ..default()
                    },
                    TextColor(UI_COLORS.text_primary),
                    Node {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                ))
                .set_parent(checkbox);
        }

        row
    }
}

// Исправить вспомогательные функции
pub fn spawn_section_header(
    commands: &mut Commands,
    parent: Entity,
    asset_server: &AssetServer,
    font_file: &str,
    text: &str,
) -> Entity {
    commands
        .spawn((
            Text::new(text),
            TextFont {
                font: asset_server.load(font_file),
                font_size: 24.0,
                ..default()
            },
            TextColor(UI_COLORS.text_primary),
            Node {
                align_self: AlignSelf::Center,
                margin: UiRect::bottom(Val::Px(24.0)),
                ..default()
            },
        ))
        .set_parent(parent)
        .id()
}

pub fn spawn_top_controls(
    commands: &mut Commands,
    parent: Entity,
    asset_server: &AssetServer,
    font_file: &str,
) {
    // ESC hint
    commands
        .spawn((
            Text::new("[Esc] to exit\n[F12] Fullscreen\n[R] to restart"),
            TextFont {
                font: asset_server.load(font_file),
                font_size: 16.0,
                ..default()
            },
            TextColor(UI_COLORS.text_secondary),
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(8.0),
                left: Val::Px(12.0),
                ..default()
            },
            TopLeftEscHint,
        ))
        .set_parent(parent);

    // Fullscreen button
    let button = commands
        .spawn((
            Node {
                width: Val::Px(120.0),
                height: Val::Px(32.0),
                position_type: PositionType::Absolute,
                top: Val::Px(8.0),
                right: Val::Px(12.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(UI_COLORS.surface_light),
            BorderRadius::all(Val::Px(8.0)),
            BorderColor(UI_COLORS.text_muted),
            Interaction::default(),
            ModernButton,
            FullscreenToggle,
        ))
        .set_parent(parent)
        .id();

    commands
        .spawn((
            Text::new("Fullscreen"),
            TextFont {
                font: asset_server.load(font_file),
                font_size: 16.0,
                ..default()
            },
            TextColor(UI_COLORS.text_primary),
        ))
        .set_parent(button);
}

// Система для отключения взаимодействий с UI когда открыт color picker
pub fn handle_color_picker_blocking(
    settings: Res<Settings>,
    mut commands: Commands,
    // Все интерактивные элементы кроме color picker'а
    interactive_query: Query<
        Entity,
        (
            With<Interaction>,
            Without<ColorPickerWindow>,
            Without<HueBar>,
            Without<SaturationBar>,
            Without<BrightnessBar>,
            Without<ApplyColorPicker>,
            Without<CloseColorPicker>,
            Without<ColorPickerOverlay>,
            Without<HueHandle>,
            Without<SaturationHandle>,
            Without<BrightnessHandle>,
        ),
    >,
    disabled_query: Query<Entity, With<DisabledInteraction>>,
) {
    if settings.color_picker_open {
        // Отключаем все интерактивные элементы
        for entity in interactive_query.iter() {
            commands.entity(entity).insert(DisabledInteraction);
        }
    } else {
        // Включаем обратно все интерактивные элементы
        for entity in disabled_query.iter() {
            commands.entity(entity).remove::<DisabledInteraction>();
        }
    }
}

// Система для блокировки взаимодействий с отключенными элементами
pub fn block_disabled_interactions(
    mut interaction_query: Query<&mut Interaction, With<DisabledInteraction>>,
) {
    for mut interaction in interaction_query.iter_mut() {
        *interaction = Interaction::None;
    }
}

// Основная функция создания color picker'а с правильной блокировкой
pub fn spawn_color_picker(
    commands: &mut Commands,
    asset_server: &AssetServer,
    settings: &Settings,
) {
    // Color picker overlay - затемнение фона
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::hsla(0.0, 0.0, 0.0, 0.7)),
            ColorPickerOverlay,
        ))
        .with_children(|overlay| {
            // Color picker window - само окно picker'а
            overlay
                .spawn((
                    Node {
                        width: Val::Px(400.0),
                        height: Val::Px(450.0),
                        padding: UiRect::all(Val::Px(24.0)),
                        flex_direction: FlexDirection::Column,
                        row_gap: Val::Px(16.0),
                        ..default()
                    },
                    BackgroundColor(UI_COLORS.surface),
                    BorderRadius::all(Val::Px(16.0)),
                    ColorPickerWindow {
                        target_type: ColorTarget::Arena,
                    },
                ))
                .with_children(|window| {
                    // Title
                    window.spawn((
                        Text::new("Color Picker"),
                        TextFont {
                            font: asset_server.load(&settings.font_file),
                            font_size: 24.0,
                            ..default()
                        },
                        TextColor(UI_COLORS.text_primary),
                    ));

                    // Color display
                    window.spawn((
                        Node {
                            width: Val::Percent(100.0),
                            height: Val::Px(80.0),
                            margin: UiRect::bottom(Val::Px(16.0)),
                            border: UiRect::all(Val::Px(2.0)),
                            ..default()
                        },
                        BackgroundColor(Color::hsl(0.0, 1.0, 0.5)),
                        BorderColor(UI_COLORS.text_muted),
                        BorderRadius::all(Val::Px(8.0)),
                        ColorDisplay,
                    ));

                    // Hue slider
                    window
                        .spawn((Node {
                            flex_direction: FlexDirection::Column,
                            row_gap: Val::Px(4.0),
                            ..default()
                        },))
                        .with_children(|container| {
                            container.spawn((
                                Text::new("Hue"),
                                TextFont {
                                    font: asset_server.load(&settings.font_file),
                                    font_size: 16.0,
                                    ..default()
                                },
                                TextColor(UI_COLORS.text_secondary),
                            ));

                            container
                                .spawn((
                                    Node {
                                        width: Val::Percent(100.0),
                                        height: Val::Px(20.0),
                                        border: UiRect::all(Val::Px(1.0)),
                                        position_type: PositionType::Relative,
                                        ..default()
                                    },
                                    BackgroundColor(Color::NONE),
                                    BorderColor(UI_COLORS.text_muted),
                                    BorderRadius::all(Val::Px(10.0)),
                                    HueBar,
                                    Interaction::default(),
                                ))
                                .with_children(|slider_container| {
                                    // Smooth hue gradient using fine segments
                                    let segments: u16 = 128;
                                    let width_percent = 100.0 / segments as f32;
                                    for i in 0..segments {
                                        let hue = (i as f32 / segments as f32) * 360.0;
                                        slider_container.spawn((
                                            Node {
                                                width: Val::Percent(width_percent),
                                                height: Val::Percent(100.0),
                                                position_type: PositionType::Absolute,
                                                left: Val::Percent(i as f32 * width_percent),
                                                ..default()
                                            },
                                            BackgroundColor(Color::hsl(hue, 1.0, 0.5)),
                                            if i == 0 {
                                                BorderRadius::left(Val::Px(9.0))
                                            } else if i == segments - 1 {
                                                BorderRadius::right(Val::Px(9.0))
                                            } else {
                                                BorderRadius::ZERO
                                            },
                                            HueGradient,
                                        ));
                                    }

                                    // Slider handle
                                    slider_container.spawn((
                                        Node {
                                            width: Val::Px(4.0),
                                            height: Val::Px(24.0),
                                            position_type: PositionType::Absolute,
                                            left: Val::Px(0.0),
                                            top: Val::Px(-2.0),
                                            ..default()
                                        },
                                        BackgroundColor(UI_COLORS.text_primary),
                                        BorderRadius::all(Val::Px(2.0)),
                                        HueHandle,
                                    ));
                                });
                        });

                    // Saturation slider
                    window
                        .spawn((Node {
                            flex_direction: FlexDirection::Column,
                            row_gap: Val::Px(4.0),
                            ..default()
                        },))
                        .with_children(|container| {
                            container.spawn((
                                Text::new("Saturation"),
                                TextFont {
                                    font: asset_server.load(&settings.font_file),
                                    font_size: 16.0,
                                    ..default()
                                },
                                TextColor(UI_COLORS.text_secondary),
                            ));

                            container
                                .spawn((
                                    Node {
                                        width: Val::Percent(100.0),
                                        height: Val::Px(20.0),
                                        border: UiRect::all(Val::Px(1.0)),
                                        position_type: PositionType::Relative,
                                        ..default()
                                    },
                                    BackgroundColor(Color::NONE),
                                    BorderColor(UI_COLORS.text_muted),
                                    BorderRadius::all(Val::Px(10.0)),
                                    SaturationBar,
                                    Interaction::default(),
                                ))
                                .with_children(|slider_container| {
                                    // Segmented saturation gradient background (updated by system)
                                    let segments: u8 = 16;
                                    let width_percent = 100.0 / segments as f32;
                                    for i in 0..segments {
                                        slider_container.spawn((
                                            Node {
                                                width: Val::Percent(width_percent),
                                                height: Val::Percent(100.0),
                                                position_type: PositionType::Absolute,
                                                left: Val::Percent(i as f32 * width_percent),
                                                ..default()
                                            },
                                            BackgroundColor(Color::BLACK),
                                            if i == 0 {
                                                BorderRadius::left(Val::Px(9.0))
                                            } else if i == segments - 1 {
                                                BorderRadius::right(Val::Px(9.0))
                                            } else {
                                                BorderRadius::ZERO
                                            },
                                            SaturationSegment {
                                                index: i,
                                                count: segments,
                                            },
                                        ));
                                    }

                                    // Slider handle
                                    slider_container.spawn((
                                        Node {
                                            width: Val::Px(4.0),
                                            height: Val::Px(24.0),
                                            position_type: PositionType::Absolute,
                                            left: Val::Px(0.0),
                                            top: Val::Px(-2.0),
                                            ..default()
                                        },
                                        BackgroundColor(UI_COLORS.text_primary),
                                        BorderRadius::all(Val::Px(2.0)),
                                        SaturationHandle,
                                    ));
                                });
                        });

                    // Brightness slider
                    window
                        .spawn((Node {
                            flex_direction: FlexDirection::Column,
                            row_gap: Val::Px(4.0),
                            ..default()
                        },))
                        .with_children(|container| {
                            container.spawn((
                                Text::new("Brightness"),
                                TextFont {
                                    font: asset_server.load(&settings.font_file),
                                    font_size: 16.0,
                                    ..default()
                                },
                                TextColor(UI_COLORS.text_secondary),
                            ));

                            container
                                .spawn((
                                    Node {
                                        width: Val::Percent(100.0),
                                        height: Val::Px(20.0),
                                        border: UiRect::all(Val::Px(1.0)),
                                        position_type: PositionType::Relative,
                                        ..default()
                                    },
                                    BackgroundColor(Color::NONE),
                                    BorderColor(UI_COLORS.text_muted),
                                    BorderRadius::all(Val::Px(10.0)),
                                    BrightnessBar,
                                    Interaction::default(),
                                ))
                                .with_children(|slider_container| {
                                    // Segmented brightness gradient background (updated by system)
                                    let segments: u8 = 16;
                                    let width_percent = 100.0 / segments as f32;
                                    for i in 0..segments {
                                        slider_container.spawn((
                                            Node {
                                                width: Val::Percent(width_percent),
                                                height: Val::Percent(100.0),
                                                position_type: PositionType::Absolute,
                                                left: Val::Percent(i as f32 * width_percent),
                                                ..default()
                                            },
                                            BackgroundColor(Color::BLACK),
                                            if i == 0 {
                                                BorderRadius::left(Val::Px(9.0))
                                            } else if i == segments - 1 {
                                                BorderRadius::right(Val::Px(9.0))
                                            } else {
                                                BorderRadius::ZERO
                                            },
                                            BrightnessSegment {
                                                index: i,
                                                count: segments,
                                            },
                                        ));
                                    }

                                    // Slider handle
                                    slider_container.spawn((
                                        Node {
                                            width: Val::Px(4.0),
                                            height: Val::Px(24.0),
                                            position_type: PositionType::Absolute,
                                            left: Val::Px(0.0),
                                            top: Val::Px(-2.0),
                                            ..default()
                                        },
                                        BackgroundColor(UI_COLORS.text_primary),
                                        BorderRadius::all(Val::Px(2.0)),
                                        BrightnessHandle,
                                    ));
                                });
                        });

                    // Buttons
                    window
                        .spawn((Node {
                            flex_direction: FlexDirection::Row,
                            column_gap: Val::Px(12.0),
                            justify_content: JustifyContent::Center,
                            margin: UiRect::top(Val::Px(16.0)),
                            ..default()
                        },))
                        .with_children(|buttons| {
                            // Apply button
                            buttons
                                .spawn((
                                    Node {
                                        width: Val::Px(100.0),
                                        height: Val::Px(40.0),
                                        justify_content: JustifyContent::Center,
                                        align_items: AlignItems::Center,
                                        margin: UiRect::all(Val::Px(4.0)),
                                        ..default()
                                    },
                                    BackgroundColor(UI_COLORS.success),
                                    BorderRadius::all(Val::Px(UI_BUTTON_RADIUS)),
                                    ModernButton,
                                    ApplyColorPicker,
                                    Interaction::default(),
                                ))
                                .with_children(|btn| {
                                    btn.spawn((
                                        Text::new("Apply"),
                                        TextFont {
                                            font: asset_server.load(&settings.font_file),
                                            font_size: 20.0,
                                            ..default()
                                        },
                                        TextColor(UI_COLORS.text_primary),
                                    ));
                                });

                            // Cancel button
                            buttons
                                .spawn((
                                    Node {
                                        width: Val::Px(100.0),
                                        height: Val::Px(40.0),
                                        margin: UiRect::all(Val::Px(4.0)),
                                        justify_content: JustifyContent::Center,
                                        align_items: AlignItems::Center,
                                        ..default()
                                    },
                                    BackgroundColor(UI_COLORS.secondary),
                                    BorderRadius::all(Val::Px(UI_BUTTON_RADIUS)),
                                    ModernButton,
                                    CloseColorPicker,
                                    Interaction::default(),
                                ))
                                .with_children(|btn| {
                                    btn.spawn((
                                        Text::new("Cancel"),
                                        TextFont {
                                            font: asset_server.load(&settings.font_file),
                                            font_size: 20.0,
                                            ..default()
                                        },
                                        TextColor(UI_COLORS.text_primary),
                                    ));
                                });
                        });
                });
        });
}

// Система для обработки кликов по overlay (закрытие по клику вне окна)
pub fn handle_color_picker_overlay_clicks(
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<ColorPickerOverlay>)>,
    mut settings: ResMut<Settings>,
    mut commands: Commands,
    picker_entities: Query<Entity, With<ColorPickerOverlay>>,
) {
    for interaction in interaction_query.iter() {
        if *interaction == Interaction::Pressed {
            // Закрываем color picker при клике на overlay
            settings.color_picker_open = false;
            // Удаляем все сущности color picker'а (включая overlay)
            for entity in picker_entities.iter() {
                commands.entity(entity).despawn_recursive();
            }
        }
    }
}

// Функция для обработки нажатий клавиш в инпут боксе
pub fn process_numeric_input(keys: &ButtonInput<KeyCode>, buf: &mut String, allow_negative: bool) {
    for code in get_numeric_keycodes_with_minus() {
        if keys.just_pressed(code) {
            handle_numeric_input(code, buf, allow_negative);
        }
    }

    // Backspace и Delete
    if keys.just_pressed(KeyCode::Backspace) || keys.just_pressed(KeyCode::CapsLock) {
        let _ = buf.pop();
    } else if keys.pressed(KeyCode::Backspace) {
        let _ = buf.pop();
    }

    if keys.just_pressed(KeyCode::Delete) {
        buf.clear();
    }
}

fn get_numeric_keycodes_with_minus() -> [KeyCode; 24] {
    [
        KeyCode::Digit0,
        KeyCode::Digit1,
        KeyCode::Digit2,
        KeyCode::Digit3,
        KeyCode::Digit4,
        KeyCode::Digit5,
        KeyCode::Digit6,
        KeyCode::Digit7,
        KeyCode::Digit8,
        KeyCode::Digit9,
        KeyCode::Numpad0,
        KeyCode::Numpad1,
        KeyCode::Numpad2,
        KeyCode::Numpad3,
        KeyCode::Numpad4,
        KeyCode::Numpad5,
        KeyCode::Numpad6,
        KeyCode::Numpad7,
        KeyCode::Numpad8,
        KeyCode::Numpad9,
        KeyCode::Period,
        KeyCode::NumpadDecimal,
        KeyCode::Minus,
        KeyCode::NumpadSubtract,
    ]
}

fn handle_numeric_input(code: KeyCode, buf: &mut String, allow_minus: bool) {
    match code {
        KeyCode::Digit0 | KeyCode::Numpad0 => buf.push('0'),
        KeyCode::Digit1 | KeyCode::Numpad1 => buf.push('1'),
        KeyCode::Digit2 | KeyCode::Numpad2 => buf.push('2'),
        KeyCode::Digit3 | KeyCode::Numpad3 => buf.push('3'),
        KeyCode::Digit4 | KeyCode::Numpad4 => buf.push('4'),
        KeyCode::Digit5 | KeyCode::Numpad5 => buf.push('5'),
        KeyCode::Digit6 | KeyCode::Numpad6 => buf.push('6'),
        KeyCode::Digit7 | KeyCode::Numpad7 => buf.push('7'),
        KeyCode::Digit8 | KeyCode::Numpad8 => buf.push('8'),
        KeyCode::Digit9 | KeyCode::Numpad9 => buf.push('9'),
        KeyCode::Period | KeyCode::NumpadDecimal => buf.push('.'),
        KeyCode::Minus | KeyCode::NumpadSubtract => {
            if allow_minus && buf.is_empty() {
                buf.push('-');
            }
        }
        _ => {}
    }
}
