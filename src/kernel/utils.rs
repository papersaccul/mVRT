use crate::fresnel::*;
use crate::game;
use crate::state::*;
use bevy::prelude::*;
use bevy::window::{CursorGrabMode, PrimaryWindow};

pub fn handle_mouse_lock(
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
    test: Res<ReactionTest>,
    mut state: ResMut<GameState>,
    app_state: Res<State<AppState>>,
) {
    if let Ok(mut window) = windows.single_mut() {
        match *app_state.get() {
            AppState::Game => {
                if test.is_running && !state.cursor_locked {
                    window.cursor_options.grab_mode = CursorGrabMode::Locked;
                    window.cursor_options.visible = false;
                    state.cursor_locked = true;
                } else if !test.is_running && state.cursor_locked {
                    window.cursor_options.grab_mode = CursorGrabMode::None;
                    window.cursor_options.visible = true;
                    state.cursor_locked = false;
                }
            }
            AppState::Settings => {
                window.cursor_options.grab_mode = CursorGrabMode::None;
                window.cursor_options.visible = true;
                if state.cursor_locked {
                    state.cursor_locked = false;
                }
            }
            AppState::Loading => {
                window.cursor_options.grab_mode = CursorGrabMode::None;
                window.cursor_options.visible = true;
                if state.cursor_locked {
                    state.cursor_locked = false;
                }
            }
        }
    }
}

pub fn apply_theme_to_scene(
    settings: Res<Settings>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    arena_query: Query<&MeshMaterial3d<StandardMaterial>, With<Arena>>,
    target_query: Query<&MeshMaterial3d<StandardMaterial>, (With<Target>, Without<Arena>)>,
) {
    if !settings.is_changed() {
        return;
    }

    let arena_color = settings.get_arena_color();
    let target_color = settings.get_target_color();

    for handle in arena_query.iter() {
        if let Some(mat) = materials.get_mut(&handle.0) {
            mat.base_color = arena_color;
        }
    }

    for handle in target_query.iter() {
        if let Some(mat) = materials.get_mut(&handle.0) {
            mat.base_color = target_color;
        }
    }
}

pub fn restart_test_on_settings_enter(
    mut test: ResMut<ReactionTest>,
    mut camera_query: Query<&mut Transform, With<PlayerCamera>>,
) {
    game::reset_test(&mut test);
    if let Ok(mut cam) = camera_query.single_mut() {
        cam.rotation = Quat::IDENTITY;
    }
}

pub fn restart_test_on_settings_exit(
    mut test: ResMut<ReactionTest>,
    mut camera_query: Query<&mut Transform, With<PlayerCamera>>,
) {
    game::reset_test(&mut test);
    if let Ok(mut cam) = camera_query.single_mut() {
        cam.rotation = Quat::IDENTITY;
    }
}

pub fn update_fresnel_target_material(
    settings: Res<Settings>,
    mut fresnel_tracker: ResMut<FresnelTracker>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut extended_materials: ResMut<Assets<ExtendedMaterial>>,
    target_query: Query<(Entity, &Transform), With<Target>>,
    test: Res<ReactionTest>,
) {
    let current_fresnel_color = settings.get_fresnel_color();

    // Check if any Fresnel-related settings have changed
    let fresnel_changed = fresnel_tracker.last_fresnel_enabled != settings.fresnel_enabled
        || (fresnel_tracker.last_fresnel_color != current_fresnel_color
            && settings.fresnel_enabled)
        || (fresnel_tracker.last_fresnel_intensity != settings.fresnel_intensity
            && settings.fresnel_enabled)
        || (fresnel_tracker.last_fresnel_power != settings.fresnel_power
            && settings.fresnel_enabled);

    if !fresnel_changed {
        return;
    }

    // Update tracker
    fresnel_tracker.last_fresnel_enabled = settings.fresnel_enabled;
    fresnel_tracker.last_fresnel_color = current_fresnel_color;
    fresnel_tracker.last_fresnel_intensity = settings.fresnel_intensity;
    fresnel_tracker.last_fresnel_power = settings.fresnel_power;

    // Get current target position before despawning
    let current_position = if let Ok((_, transform)) = target_query.get_single() {
        transform.translation
    } else if test.is_running {
        // If test is running, use test target position
        test.target_position
    } else {
        Vec3::ZERO
    };

    // Only despawn if target exists
    for (entity, _) in target_query.iter() {
        commands.entity(entity).despawn();
    }

    let target_color = settings.get_target_color();

    // Spawn new target with appropriate material and preserve position
    if settings.fresnel_enabled {
        let fresnel_material = FresnelMaterial {
            fresnel_color: Vec4::new(
                current_fresnel_color.to_linear().red,
                current_fresnel_color.to_linear().green,
                current_fresnel_color.to_linear().blue,
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
            Mesh3d(meshes.add(Mesh::from(bevy::math::primitives::Sphere {
                radius: TARGET_SIZE,
            }))),
            MeshMaterial3d(extended_materials.add(ExtendedMaterial {
                base: StandardMaterial {
                    base_color: target_color,
                    unlit: false,
                    ..default()
                },
                extension: fresnel_material,
            })),
            Transform::from_translation(current_position),
            Visibility::default(),
            Target,
        ));
    } else {
        commands.spawn((
            Mesh3d(meshes.add(Mesh::from(bevy::math::primitives::Sphere {
                radius: TARGET_SIZE,
            }))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: target_color,
                unlit: false,
                ..default()
            })),
            Transform::from_translation(current_position),
            Visibility::default(),
            Target,
        ));
    }
}

pub fn refresh_target_on_game_enter(
    settings: Res<Settings>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut extended_materials: ResMut<Assets<ExtendedMaterial>>,
    target_query: Query<Entity, With<Target>>,
    mut fresnel_tracker: ResMut<FresnelTracker>,
) {
    // Always refresh target when entering game to ensure proper material
    for entity in target_query.iter() {
        commands.entity(entity).despawn();
    }

    let target_color = settings.get_target_color();
    let fresnel_color = settings.get_fresnel_color();

    // Spawn target with correct material based on current settings
    if settings.fresnel_enabled {
        let fresnel_material = FresnelMaterial {
            fresnel_color: Vec4::new(
                fresnel_color.to_linear().red,
                fresnel_color.to_linear().green,
                fresnel_color.to_linear().blue,
                settings.fresnel_intensity,
            ),
            fresnel_params: Vec4::new(settings.fresnel_power, 1.0, 0.0, 0.0),
        };

        commands.spawn((
            Mesh3d(meshes.add(Mesh::from(bevy::math::primitives::Sphere {
                radius: TARGET_SIZE,
            }))),
            MeshMaterial3d(extended_materials.add(ExtendedMaterial {
                base: StandardMaterial {
                    base_color: target_color,
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
        commands.spawn((
            Mesh3d(meshes.add(Mesh::from(bevy::math::primitives::Sphere {
                radius: TARGET_SIZE,
            }))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: target_color,
                unlit: false,
                ..default()
            })),
            Transform::from_translation(Vec3::ZERO),
            Visibility::default(),
            Target,
        ));
    }

    // Update tracker to current settings
    fresnel_tracker.last_fresnel_enabled = settings.fresnel_enabled;
    fresnel_tracker.last_fresnel_color = fresnel_color;
    fresnel_tracker.last_fresnel_intensity = settings.fresnel_intensity;
    fresnel_tracker.last_fresnel_power = settings.fresnel_power;
}
