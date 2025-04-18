use bevy::{prelude::*, render::camera::ScalingMode};

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
