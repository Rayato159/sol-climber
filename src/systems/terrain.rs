use bevy::{
    prelude::*,
    render::mesh::{Indices, PrimitiveTopology},
    scene::SceneInstanceReady,
};
use bevy_rapier3d::prelude::*;

const TERRAIN_SCENE_PATH: &str = "terrains/Mountain.glb";
const TERRAIN_MESH_PATH: &str = "terrains/MountainPath.glb";

pub fn spawn_terrain(mut commands: Commands, asset_server: Res<AssetServer>) {
    let terrain_mesh = asset_server.load(GltfAssetLabel::Scene(0).from_asset(TERRAIN_MESH_PATH));
    let terrain_scene = asset_server.load(GltfAssetLabel::Scene(0).from_asset(TERRAIN_SCENE_PATH));

    commands.spawn((Name::new("TerrainScene"), SceneRoot(terrain_scene)));

    commands
        .spawn((Name::new("TerrainMesh"), SceneRoot(terrain_mesh)))
        .observe(on_terrian_spawned);
}

pub fn on_terrian_spawned(
    trigger: Trigger<SceneInstanceReady>,
    children: Query<&Children>,
    meshes: Res<Assets<Mesh>>,
    mesh_query: Query<&Mesh3d>,
    mut commands: Commands,
) {
    for entity in children.iter_descendants(trigger.entity()) {
        let Ok(mesh) = mesh_query.get(entity) else {
            continue;
        };

        let Some(terrain_mesh) = meshes.get(mesh.0.id()) else {
            continue;
        };

        if let Some((vertices, indices)) = extract_mesh_data(terrain_mesh) {
            commands.entity(entity).insert((
                RigidBody::Fixed,
                Collider::trimesh_with_flags(vertices, indices, TriMeshFlags::FIX_INTERNAL_EDGES),
            ));
        }
    }
}

fn extract_mesh_data(mesh: &Mesh) -> Option<(Vec<Vec3>, Vec<[u32; 3]>)> {
    if mesh.primitive_topology() != PrimitiveTopology::TriangleList {
        return None;
    }

    let vertex_positions = mesh.attribute(Mesh::ATTRIBUTE_POSITION)?.as_float3()?;
    let vertices = vertex_positions.iter().map(|v| Vec3::from(*v)).collect();

    let indices = match mesh.indices()? {
        Indices::U32(ind) => ind.chunks(3).map(|i| [i[0], i[1], i[2]]).collect(),
        Indices::U16(ind) => ind
            .chunks(3)
            .map(|i| [i[0] as u32, i[1] as u32, i[2] as u32])
            .collect(),
    };

    Some((vertices, indices))
}
