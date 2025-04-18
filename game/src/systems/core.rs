use bevy::{input::mouse::AccumulatedMouseScroll, prelude::*};

use crate::resources::camera_setting::CameraSetting;

pub fn camera_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut camera: Single<(&mut Transform, &Camera3d)>,
) {
    if keyboard_input.pressed(KeyCode::KeyD) {
        camera.0.translation.x += 5. * time.delta_secs();
    }

    if keyboard_input.pressed(KeyCode::KeyA) {
        camera.0.translation.x -= 5. * time.delta_secs();
    }

    if keyboard_input.pressed(KeyCode::KeyW) {
        camera.0.translation.z -= 5. * time.delta_secs();
    }

    if keyboard_input.pressed(KeyCode::KeyS) {
        camera.0.translation.z += 5. * time.delta_secs();
    }
}

pub fn camera_zoom(
    mut camera: Single<(&mut Projection, &Camera)>,
    setting: Res<CameraSetting>,
    mouse_scroll_input: Res<AccumulatedMouseScroll>,
) {
    let delta_zoom = -mouse_scroll_input.delta.y * setting.zoom_speed;
    let multiplicative_zoom = 1. + delta_zoom;

    if let Projection::Orthographic(ref mut orthographic) = *camera.0 {
        orthographic.scale = (orthographic.scale * multiplicative_zoom)
            .clamp(setting.zoom_range.start, setting.zoom_range.end);
    }
}
