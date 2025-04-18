use bevy::{input::mouse::AccumulatedMouseScroll, prelude::*, render::camera::ScalingMode};

use crate::resources::camera_setting::CameraSetting;

pub fn spawn_camera(mut commands: Commands, camera_settings: Res<CameraSetting>) {
    let init_pos = Vec3::new(5., 5., 5.);

    commands.spawn((
        Camera3d::default(),
        Projection::from(OrthographicProjection {
            scaling_mode: ScalingMode::FixedVertical {
                viewport_height: camera_settings.viewport_height,
            },
            scale: 1.,
            ..OrthographicProjection::default_3d()
        }),
        Transform::from_translation(init_pos).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

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
