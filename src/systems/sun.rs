use bevy::{pbr::VolumetricLight, prelude::*};

pub fn spawn_sun(mut commands: Commands) {
    let sun_pos = Vec3::new(300.0, 300.0, 300.0);

    commands.spawn((
        DirectionalLight {
            illuminance: 50_000.0,
            shadows_enabled: true,
            color: Color::srgb(1.0, 0.95, 0.85),
            ..Default::default()
        },
        VolumetricLight,
        Transform::from_translation(sun_pos).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}
