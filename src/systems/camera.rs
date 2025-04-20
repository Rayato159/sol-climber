use bevy::{
    input::mouse::{AccumulatedMouseScroll, MouseMotion},
    prelude::*,
};

use crate::{
    entities::player::Player,
    resources::camera::{CameraOrbit, CameraSetting},
};

pub fn spawn_camera(mut commands: Commands, camera_setting: Res<CameraSetting>) {
    let init_pos = Vec3::new(0.0, 2.0, 5.0);

    commands.spawn((
        Camera3d::default(),
        Projection::from(PerspectiveProjection {
            fov: camera_setting.zoom_range.start,
            aspect_ratio: 16.0 / 9.0,
            ..Default::default()
        }),
        Transform::from_translation(init_pos).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

pub fn camera_follow_player(
    player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (With<Camera3d>, Without<Player>)>,
    time: Res<Time>,
) {
    let player_transform = player_query.single();
    let mut camera_transform = camera_query.single_mut();

    let offset = Vec3::new(0.0, 4.0, 10.0);
    let target_pos = player_transform.translation + offset;

    let follow_speed = 0.1;
    camera_transform.translation = camera_transform
        .translation
        .lerp(target_pos, follow_speed * time.delta_secs());

    camera_transform.look_at(player_transform.translation, Vec3::Y);
}

pub fn camera_zoom(
    mut camera_query: Query<(&mut Projection, &Camera), With<Camera3d>>,
    camera_setting: Res<CameraSetting>,
    mouse_scroll_input: Res<AccumulatedMouseScroll>,
) {
    if let Projection::Perspective(ref mut perspective) = *camera_query.single_mut().0 {
        let delta_zoom = -mouse_scroll_input.delta.y * camera_setting.zoom_speed;

        perspective.fov = (perspective.fov + delta_zoom).clamp(
            camera_setting.zoom_range.start,
            camera_setting.zoom_range.end,
        );
    }
}

pub fn orbit_camera_control(
    mut orbit: ResMut<CameraOrbit>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    mut motion_evr: EventReader<MouseMotion>,
) {
    if mouse_button.pressed(MouseButton::Right) {
        let mut delta = Vec2::ZERO;

        for ev in motion_evr.read() {
            delta += ev.delta;
        }

        // Rotation speed
        let sensitivity = 0.005;

        orbit.yaw += delta.x * sensitivity;
        orbit.pitch += delta.y * sensitivity;

        // Clamp pitch to avoid flipping
        orbit.pitch = orbit.pitch.clamp(-89_f32.to_radians(), 89_f32.to_radians());
    }
}

pub fn camera_follow_orbit_player(
    orbit: Res<CameraOrbit>,
    mut camera_query: Query<&mut Transform, (With<Camera3d>, Without<Player>)>,
    player_query: Query<&Transform, With<Player>>,
) {
    let player = player_query.single();
    let mut camera_transform = camera_query.single_mut();

    let yaw = orbit.yaw;
    let pitch = orbit.pitch;
    let radius = orbit.radius;

    // Projection: Spherical to Cartesian

    // Project r to x axis and y axis
    // x = r * sin(yaw) * cos(pitch)

    // Project r to y axis
    // y = r * sin(pitch)

    // Project r to z axis and y axis
    // z = r * cos(yaw) * cos(pitch)

    let x = radius * yaw.cos() * pitch.cos();
    let y = radius * pitch.sin();
    let z = radius * yaw.sin() * pitch.cos();

    let offset = Vec3::new(x, y, z);
    let target = player.translation;

    camera_transform.translation = target + offset;
    camera_transform.look_at(target, Vec3::Y);
}
