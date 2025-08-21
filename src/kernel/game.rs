use crate::fresnel::*;
use crate::state::*;
use crate::user_interface::ui::UI_COLORS;
use bevy::math::primitives::Sphere as SpherePrim;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use std::f32::consts::PI;

pub fn setup_game_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut extended_materials: ResMut<Assets<ExtendedMaterial>>,
    settings: Res<Settings>,
    windows: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    // Загружаем текстуру для стен
    let texture_handle: Handle<Image> = asset_server.load("texture.png");

    // Получаем цвет арены из настроек
    let wall_color = settings.get_arena_color();

    let half_size = ARENA_SIZE / 2.0;

    let game_audio = GameAudio {
        hit_sound: asset_server.load("hit.ogg"),
    };
    commands.insert_resource(game_audio);

    // Пол (нижняя стена)
    commands.spawn((
        Mesh3d(meshes.add(Mesh::from(bevy::math::primitives::Plane3d {
            normal: Dir3::Y,
            half_size: Vec2::new(half_size, half_size),
        }))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color_texture: Some(texture_handle.clone()),
            base_color: wall_color,
            unlit: true,
            ..default()
        })),
        Transform::from_translation(Vec3::new(0.0, -half_size, 0.0)),
        Visibility::default(),
        Arena,
        ArenaWall,
    ));

    // Потолок (верхняя стена)
    commands.spawn((
        Mesh3d(meshes.add(Mesh::from(bevy::math::primitives::Plane3d {
            normal: Dir3::Y,
            half_size: Vec2::new(half_size, half_size),
        }))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color_texture: Some(texture_handle.clone()),
            base_color: wall_color,
            unlit: true,
            ..default()
        })),
        Transform::from_translation(Vec3::new(0.0, half_size, 0.0))
            .with_rotation(Quat::from_rotation_x(PI)),
        Visibility::default(),
        Arena,
        ArenaWall,
    ));

    // Задняя стена
    commands.spawn((
        Mesh3d(meshes.add(Mesh::from(bevy::math::primitives::Plane3d {
            normal: Dir3::Z,
            half_size: Vec2::new(half_size, half_size),
        }))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color_texture: Some(texture_handle.clone()),
            base_color: wall_color,
            unlit: true,
            ..default()
        })),
        Transform::from_translation(Vec3::new(0.0, 0.0, -half_size)),
        Visibility::default(),
        Arena,
        ArenaWall,
    ));

    // Передняя стена
    commands.spawn((
        Mesh3d(meshes.add(Mesh::from(bevy::math::primitives::Plane3d {
            normal: Dir3::Z,
            half_size: Vec2::new(half_size, half_size),
        }))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color_texture: Some(texture_handle.clone()),
            base_color: wall_color,
            unlit: true,
            ..default()
        })),
        Transform::from_translation(Vec3::new(0.0, 0.0, half_size))
            .with_rotation(Quat::from_rotation_x(PI)),
        Visibility::default(),
        Arena,
        ArenaWall,
    ));

    // Левая стена
    commands.spawn((
        Mesh3d(meshes.add(Mesh::from(bevy::math::primitives::Plane3d {
            normal: Dir3::X,
            half_size: Vec2::new(half_size, half_size),
        }))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color_texture: Some(texture_handle.clone()),
            base_color: wall_color,
            unlit: true,
            ..default()
        })),
        Transform::from_translation(Vec3::new(-half_size, 0.0, 0.0)),
        Visibility::default(),
        Arena,
        ArenaWall,
    ));

    // Правая стена
    commands.spawn((
        Mesh3d(meshes.add(Mesh::from(bevy::math::primitives::Plane3d {
            normal: Dir3::X,
            half_size: Vec2::new(half_size, half_size),
        }))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color_texture: Some(texture_handle.clone()),
            base_color: wall_color,
            unlit: true,
            ..default()
        })),
        Transform::from_translation(Vec3::new(half_size, 0.0, 0.0))
            .with_rotation(Quat::from_rotation_y(PI)),
        Visibility::default(),
        Arena,
        ArenaWall,
    ));

    // Target sphere (spawn at center initially; will be positioned at fixed distance on test start)
    if settings.fresnel_enabled {
        // Use extended material with Fresnel effect
        let fresnel_color = settings.get_fresnel_color();
        let fresnel_material = FresnelMaterial {
            fresnel_color: Vec4::new(
                fresnel_color.to_linear().red,
                fresnel_color.to_linear().green,
                fresnel_color.to_linear().blue,
                settings.fresnel_intensity, // intensity in alpha channel
            ),
            fresnel_params: Vec4::new(
                settings.fresnel_power,
                1.0, // enabled
                0.0, // unused
                0.0, // unused
            ),
        };

        commands.spawn((
            Mesh3d(meshes.add(Mesh::from(SpherePrim {
                radius: TARGET_SIZE,
            }))),
            MeshMaterial3d(extended_materials.add(ExtendedMaterial {
                base: StandardMaterial {
                    base_color: settings.get_target_color(),
                    unlit: false,
                    ..default()
                },
                extension: fresnel_material,
            })),
            Transform::from_translation(Vec3::ZERO),
            Visibility::default(),
            Target,
        ));
    } else {
        // Use standard material
        commands.spawn((
            Mesh3d(meshes.add(Mesh::from(SpherePrim {
                radius: TARGET_SIZE,
            }))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: settings.get_target_color(),
                unlit: false,
                ..default()
            })),
            Transform::from_translation(Vec3::ZERO),
            Visibility::default(),
            Target,
        ));
    }

    // Camera with FOV setting
    let aspect_ratio = if let Ok(window) = windows.single() {
        window.width() / window.height()
    } else {
        1200.0 / 800.0
    };

    // Конвертируем горизонтальный FOV в вертикальный
    let horizontal_fov = settings.fov * PI / 180.0;
    let vertical_fov = 2.0 * ((horizontal_fov / 2.0).tan() / aspect_ratio).atan();

    commands.spawn((
        Camera3d::default(),
        Camera {
            order: 0,
            clear_color: ClearColorConfig::Default,
            ..default()
        },
        Projection::Perspective(PerspectiveProjection {
            fov: vertical_fov,
            near: 0.1,
            far: 1000.0,
            aspect_ratio,
        }),
        Transform::from_xyz(0.0, 0.0, 10.0),
        PlayerCamera,
    ));

    // Lighting for PBR
    commands.spawn((
        DirectionalLight {
            illuminance: settings.directional_light_illuminance,
            shadows_enabled: false,
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
        DirectionalLightEntity,
    ));
    // ambient light
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: settings.ambient_light_brightness,
        ..default()
    });

    // Game UI
    setup_game_ui(&mut commands, &settings, &asset_server);
}

pub fn setup_game_ui(
    commands: &mut Commands,
    settings: &Settings,
    asset_server: &Res<AssetServer>,
) {
    // 2D camera for UI
    commands.spawn((
        Camera2d,
        Camera {
            order: 100,
            clear_color: ClearColorConfig::None,
            ..default()
        },
        GameUI,
    ));

    // Основной игровой UI с современным дизайном
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(20.0),
                top: Val::Px(20.0),
                padding: UiRect::all(Val::Px(16.0)),
                ..default()
            },
            BackgroundColor(UI_COLORS.surface_light),
            BorderRadius::all(Val::Px(12.0)),
            GameUI,
        ))
        .with_children(|panel| {
            panel.spawn((
                Text::new("MODERN REACTION TEST\n\nPress SPACE to start test"),
                TextFont {
                    font: asset_server.load(&settings.font_file),
                    font_size: 18.0,
                    ..default()
                },
                TextColor(UI_COLORS.text_primary),
                GameInfoText,
            ));
        });

    // FPS индикатор
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                right: Val::Px(16.0),
                top: Val::Px(16.0),
                padding: UiRect::all(Val::Px(8.0)),
                ..default()
            },
            BackgroundColor(UI_COLORS.surface_light),
            BorderRadius::all(Val::Px(8.0)),
            GameUI,
        ))
        .with_children(|fps_panel| {
            fps_panel.spawn((
                Text::new("FPS: --"),
                TextFont {
                    font: asset_server.load(&settings.font_file),
                    font_size: 14.0,
                    ..default()
                },
                TextColor(UI_COLORS.text_secondary),
                FpsText,
            ));
        });

    // Центральная подсказка с современным дизайном
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            GameUI,
            StartCenterText,
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    Node {
                        padding: UiRect::all(Val::Px(24.0)),
                        ..default()
                    },
                    BackgroundColor(UI_COLORS.background),
                    BorderRadius::all(Val::Px(16.0)),
                ))
                .with_children(|hint_panel| {
                    hint_panel.spawn((
                        Text::new("Press SPACE to start"),
                        TextFont {
                            font: asset_server.load(&settings.font_file),
                            font_size: 28.0,
                            ..default()
                        },
                        TextColor(UI_COLORS.text_primary),
                    ));
                });
        });

    // UI Crosshair - основной элемент с улучшенным дизайном
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                left: Val::Percent(50.0),
                top: Val::Percent(50.0),
                width: Val::Px(settings.crosshair_size),
                height: Val::Px(settings.crosshair_size),
                margin: UiRect {
                    left: Val::Px(-settings.crosshair_size / 2.0),
                    top: Val::Px(-settings.crosshair_size / 2.0),
                    right: Val::Px(0.0),
                    bottom: Val::Px(0.0),
                },
                ..default()
            },
            Crosshair,
            GameUI,
        ))
        .with_children(|parent| {
            // Горизонтальная линия с тенью
            parent.spawn((
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Px(0.0),
                    top: Val::Percent(50.0),
                    width: Val::Percent(100.0),
                    height: Val::Px(settings.crosshair_thickness),
                    margin: UiRect {
                        top: Val::Px(-settings.crosshair_thickness / 2.0),
                        ..default()
                    },
                    ..default()
                },
                BackgroundColor(settings.get_crosshair_color()),
            ));

            // Вертикальная линия с тенью
            parent.spawn((
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Percent(50.0),
                    top: Val::Px(0.0),
                    width: Val::Px(settings.crosshair_thickness),
                    height: Val::Percent(100.0),
                    margin: UiRect {
                        left: Val::Px(-settings.crosshair_thickness / 2.0),
                        ..default()
                    },
                    ..default()
                },
                BackgroundColor(settings.get_crosshair_color()),
            ));
        });
}

pub fn cleanup_game_scene(
    mut commands: Commands,
    arena_query: Query<Entity, With<Arena>>,
    target_query: Query<Entity, With<Target>>,
    camera_query: Query<Entity, With<PlayerCamera>>,
    ui_query: Query<Entity, With<GameUI>>,
    crosshair_query: Query<Entity, With<Crosshair>>,
    light_query: Query<Entity, With<DirectionalLightEntity>>,
) {
    for entity in arena_query
        .iter()
        .chain(target_query.iter())
        .chain(camera_query.iter())
        .chain(ui_query.iter())
        .chain(crosshair_query.iter())
        .chain(light_query.iter())
    {
        commands.entity(entity).despawn();
    }
}

pub fn update_lighting(
    settings: Res<Settings>,
    mut directional_light_query: Query<
        (Entity, &mut DirectionalLight),
        With<DirectionalLightEntity>,
    >,
    mut ambient_light: ResMut<AmbientLight>,
    mut commands: Commands,
) {
    // Update directional light by recreating it to avoid "stuck" values
    if settings.is_changed() {
        // Обновляем направленный свет, пересоздавая его, чтобы избежать "застрявших" значений
        if let Ok((entity, mut directional_light)) = directional_light_query.get_single_mut() {
            let target_illuminance = settings.directional_light_illuminance;

            // Обновляем, только если значение действительно изменилось
            if (directional_light.illuminance - target_illuminance).abs() > f32::EPSILON {
                // Удаляем старый источник света
                commands.entity(entity).despawn();

                // Создаем новый направленный свет с обновленными настройками
                commands.spawn((
                    DirectionalLight {
                        illuminance: target_illuminance,
                        shadows_enabled: false,
                        ..default()
                    },
                    // Убедитесь, что вы снова задали его transform
                    Transform::from_xyz(0.0, 0.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
                    DirectionalLightEntity,
                ));
            }
        }

        // Update ambient light
        ambient_light.brightness = settings.ambient_light_brightness;
    }
}

pub fn game_input_system(
    keys: Res<ButtonInput<KeyCode>>,
    mut test: ResMut<ReactionTest>,
    mut next_state: ResMut<NextState<AppState>>,
    time_fixed: Res<Time<Fixed>>,
    mut camera_query: Query<&mut Transform, With<PlayerCamera>>,
    mut target_query: Query<&mut Transform, (With<Target>, Without<PlayerCamera>)>,
    settings: Res<Settings>,
) {
    // Quick restart
    if keys.just_pressed(settings.key_restart) {
        reset_test(&mut test);
        if let Ok(mut cam) = camera_query.get_single_mut() {
            cam.rotation = Quat::IDENTITY;
            start_test(&mut test, &time_fixed);
            test.start_cam_pos = cam.translation;
            let forward = cam.forward();
            test.start_cam_forward = *forward;
            test.target_position =
                test.start_cam_pos + test.start_cam_forward.normalize() * test.target_distance;
            if let Ok(mut t) = target_query.get_single_mut() {
                t.translation = test.target_position;
            }
        }
    }

    if keys.just_pressed(settings.key_start) && !test.is_running {
        start_test(&mut test, &time_fixed);
        if let Ok(mut cam) = camera_query.get_single_mut() {
            cam.rotation = Quat::IDENTITY;
            test.start_cam_pos = cam.translation;
            let forward = cam.forward();
            test.start_cam_forward = *forward;
            test.target_position =
                test.start_cam_pos + test.start_cam_forward.normalize() * test.target_distance;
            if let Ok(mut t) = target_query.get_single_mut() {
                t.translation = test.target_position;
            }
        }
    }

    if keys.just_pressed(settings.key_settings) {
        next_state.set(AppState::Settings);
    }
}

pub fn start_test(test: &mut ReactionTest, time_fixed: &Time<Fixed>) {
    test.is_running = true;
    test.test_completed = false;
    test.start_time = time_fixed.elapsed().as_secs_f32();
    test.data.clear();

    test.crosshair_direction = Vec3::NEG_Z;
    // Place target directly under crosshair at a fixed distance along -Z
    test.target_position = test.crosshair_direction.normalize() * test.target_distance;
    // Initialize lateral velocity; movement will be constrained to sphere of radius target_distance
    test.target_velocity = Vec3::new(1.0, 0.5, 0.5).normalize() * TARGET_SPEED;
    test.next_direction_change = 0.1;

    test.camera_yaw = 0.0;
    test.camera_pitch = 0.0;
}

pub fn reset_test(test: &mut ReactionTest) {
    test.is_running = false;
    test.test_completed = false;
    test.start_time = 0.0;
    test.hits = 0;
    test.misses = 0;
    test.last_shot_time = 0.0;
    test.data.clear();
    test.crosshair_direction = Vec3::NEG_Z;
    test.target_position = Vec3::ZERO;
    test.target_velocity = Vec3::new(1.0, 0.5, 0.5).normalize() * TARGET_SPEED;
    test.next_direction_change = 0.1;
    test.change_interval = 0.1;
    test.camera_yaw = 0.0;
    test.camera_pitch = 0.0;
}

// Система для обновления цвета стен арены
pub fn update_arena_walls_color(
    wall_query: Query<&MeshMaterial3d<StandardMaterial>, With<ArenaWall>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    settings: Res<Settings>,
) {
    if settings.is_changed() {
        let wall_color = settings.get_arena_color();

        for material in wall_query.iter() {
            // Получаем Handle из MeshMaterial3d и обновляем материал
            if let Some(material_asset) = materials.get_mut(material) {
                material_asset.base_color = wall_color;
            }
        }
    }
}
