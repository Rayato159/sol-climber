use bevy::prelude::*;

#[derive(Component, Default)]
pub struct UICamera;

pub fn spawn_2d_camera(mut commands: Commands) {
    commands.spawn((
        UICamera::default(),
        Camera {
            order: 1,
            ..Default::default()
        },
        Camera2d::default(),
    ));
}
