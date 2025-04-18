use bevy::prelude::*;

pub fn spawn_light(mut commands: Commands) {
    let light_pos = Vec3::new(10.0, 20.0, 10.0);
    let target = Vec3::ZERO;

    commands.spawn((
        DirectionalLight {
            illuminance: 10_000.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_translation(light_pos).looking_at(target, Vec3::Y),
    ));
}
