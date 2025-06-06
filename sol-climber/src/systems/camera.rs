use bevy::{
    core_pipeline::Skybox,
    input::mouse::{AccumulatedMouseScroll, MouseMotion},
    prelude::*,
};

use bevy_rapier3d::prelude::*;

use crate::{
    entities::player::{Player, PlayerPhysics},
    resources::camera::{CameraOrbit, CameraSetting},
    terrains::skybox::Cubemap,
};

#[derive(Component, Default)]
pub struct MainCamera;

pub fn spawn_3d_camera(
    mut commands: Commands,
    camera_setting: Res<CameraSetting>,
    asset_server: Res<AssetServer>,
) {
    let image_handle = asset_server.load("skybox/day/EverestSkyboxFlat.png");

    commands.insert_resource(Cubemap {
        image_handle: image_handle.clone(),
        is_loaded: false,
    });

    let init_pos = Vec3::new(0.0, 0.7, 5.0);

    commands.spawn((
        MainCamera::default(),
        Camera3d::default(),
        Projection::from(PerspectiveProjection {
            fov: camera_setting.zoom_range.start,
            aspect_ratio: 16.0 / 9.0,
            ..Default::default()
        }),
        Skybox {
            image: image_handle,
            brightness: 1000.0,
            ..Default::default()
        },
        Camera {
            order: 0,
            ..Default::default()
        },
        Transform::from_translation(init_pos).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

pub fn camera_follow_player(
    player_query: Query<(&Transform, &PlayerPhysics), With<Player>>,
    mut camera_query: Query<&mut Transform, (With<MainCamera>, Without<Player>)>,
    time: Res<Time>,
) {
    for (player_transform, player_physics) in player_query.iter() {
        let Ok(mut camera_transform) = camera_query.single_mut() else {
            continue;
        };

        let offset = Vec3::new(0.0, player_physics.player_height, 5.0);
        let target_pos = player_transform.translation + offset;

        let follow_speed = 0.1;
        camera_transform.translation = camera_transform
            .translation
            .lerp(target_pos, follow_speed * time.delta_secs());

        camera_transform.look_at(player_transform.translation, Vec3::Y);
    }
}

pub fn camera_zoom(
    mut camera_query: Query<(&mut Projection, &Camera), With<MainCamera>>,
    camera_setting: Res<CameraSetting>,
    mouse_scroll_input: Res<AccumulatedMouseScroll>,
) {
    let Ok(mut camera_query) = camera_query.single_mut() else {
        return;
    };

    if let Projection::Perspective(ref mut perspective) = *camera_query.0 {
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
        orbit.pitch = orbit.pitch.clamp(-40_f32.to_radians(), 89_f32.to_radians());
    }
}

pub fn camera_follow_orbit_player(
    orbit: Res<CameraOrbit>,
    rapier_context: ReadRapierContext,
    player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (With<MainCamera>, Without<Player>)>,
) {
    for player in player_query.iter() {
        let Ok(mut camera_transform) = camera_query.single_mut() else {
            continue;
        };

        let Ok(context) = rapier_context.single() else {
            continue;
        };

        let yaw = orbit.yaw;
        let pitch = orbit.pitch;
        let radius = orbit.radius;

        let x = radius * yaw.cos() * pitch.cos();
        let y = radius * pitch.sin();
        let z = radius * yaw.sin() * pitch.cos();

        let offset = Vec3::new(x, y, z);

        let player_eye_pos = player.translation + Vec3::Y * 1.2;

        let ideal_camera_pos = player_eye_pos + offset;
        let dir = (ideal_camera_pos - player_eye_pos).normalize();
        let max_dist = offset.length();
        let mut final_dist = max_dist;

        if let Some((_entity, toi)) = context.cast_ray(
            player_eye_pos,
            dir,
            max_dist,
            true,
            QueryFilter::default().exclude_sensors(),
        ) {
            final_dist = toi - 0.1;
        }

        let actual_camera_pos = player_eye_pos + dir * final_dist;
        camera_transform.translation = actual_camera_pos;
        camera_transform.look_at(player_eye_pos, Vec3::Y);
    }
}
