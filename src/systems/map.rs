use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub fn spawn_floor(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let size_x = 15.0;
    let size_z = 15.0;
    let thickness = 0.01;

    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(size_x, size_z))),
        MeshMaterial3d(materials.add(Color::srgb(50. / 225., 142. / 225., 110. / 225.))),
        RigidBody::Fixed,
        Collider::cuboid(size_x / 2.0, thickness / 2.0, size_z / 2.0),
        Transform::from_xyz(0.0, -thickness, 0.0),
    ));
}
