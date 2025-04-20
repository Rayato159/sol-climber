use bevy::{pbr::FogVolume, prelude::*};

pub fn spawn_fog(mut commands: Commands) {
    commands.spawn((
        FogVolume {
            fog_color: Color::srgb(0.8, 0.85, 0.9),
            ..Default::default()
        },
        Transform::from_scale(Vec3::splat(35.0)),
    ));
}
