use bevy::prelude::*;

pub fn spawn_light(mut commands: Commands) {
    commands.spawn((
        DirectionalLight::default(),
        Transform::from_translation(Vec3::ONE).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}
