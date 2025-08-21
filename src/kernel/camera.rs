use crate::state::*;
use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use std::f32::consts::PI;

pub fn update_camera(
    mut mouse_motion: EventReader<MouseMotion>,
    mut test: ResMut<ReactionTest>,
    mut camera_query: Query<&mut Transform, With<PlayerCamera>>,
    settings: Res<Settings>,
) {
    if !test.is_running {
        return;
    }

    let mut delta = Vec2::ZERO;
    for motion in mouse_motion.read() {
        delta += motion.delta;
    }

    if delta.length_squared() > 0.0 {
        let mouse_sensitivity = settings.mouse_sensitivity();
        test.camera_yaw -= delta.x * mouse_sensitivity;
        test.camera_pitch -= delta.y * mouse_sensitivity;
        test.camera_pitch = test.camera_pitch.clamp(-PI / 2.0, PI / 2.0);

        let yaw_quat = Quat::from_axis_angle(Vec3::Y, test.camera_yaw);
        let pitch_quat = Quat::from_axis_angle(Vec3::X, test.camera_pitch);
        test.crosshair_direction = yaw_quat * pitch_quat * Vec3::NEG_Z;

        if let Ok(mut camera_transform) = camera_query.single_mut() {
            camera_transform.rotation = yaw_quat * pitch_quat;
        }
    }
}

pub fn apply_fov_to_camera(
    settings: Res<Settings>,
    windows: Query<&Window, With<PrimaryWindow>>,
    mut q: Query<&mut Projection, With<PlayerCamera>>,
) {
    let aspect_ratio = if let Ok(window) = windows.get_single() {
        window.width() / window.height()
    } else {
        1200.0 / 800.0
    };

    let horizontal_fov = settings.fov * PI / 180.0;
    let vertical_fov = 2.0 * ((horizontal_fov / 2.0).tan() / aspect_ratio).atan();

    if let Ok(mut projection) = q.get_single_mut() {
        if let Projection::Perspective(ref mut persp) = *projection {
            persp.fov = vertical_fov;
            persp.aspect_ratio = aspect_ratio;
        }
    }
}
