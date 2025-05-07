use bevy::{prelude::*, scene::SceneInstanceReady};
use bevy_rapier3d::prelude::*;

use super::extract_mesh_data;

const TERRAIN_PATH: &str = "terrains/Terrain.glb";
const TERRAIN_MESH_PATH: &str = "terrains/TerrainMesh.glb";
const SUMMIT_ZONE_MESH_PATH: &str = "terrains/SummitZoneMesh.glb";

#[derive(Debug, Default, Component)]
pub struct Terrain;

#[derive(Debug, Default, Component)]
pub struct SummitZone;

pub fn spawn_terrain(mut commands: Commands, asset_server: Res<AssetServer>) {
    let terrain = asset_server.load(GltfAssetLabel::Scene(0).from_asset(TERRAIN_PATH));
    let terrain_mesh = asset_server.load(GltfAssetLabel::Scene(0).from_asset(TERRAIN_MESH_PATH));
    let summit_zone_mesh =
        asset_server.load(GltfAssetLabel::Scene(0).from_asset(SUMMIT_ZONE_MESH_PATH));

    commands.spawn((
        Name::new("TerrainScene"),
        Terrain::default(),
        SceneRoot(terrain),
    ));
    commands
        .spawn((Name::new("TerrainMesh"), SceneRoot(terrain_mesh)))
        .observe(on_mesh_spawned);

    commands
        .spawn((Name::new("SummitZoneMesh"), SceneRoot(summit_zone_mesh)))
        .observe(on_summit_zone_mesh_spawned);
}

pub fn on_mesh_spawned(
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
                Restitution::coefficient(0.0),
                Friction::coefficient(1.0),
            ));
        }
    }
}

pub fn on_summit_zone_mesh_spawned(
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
                Restitution::coefficient(0.0),
                Friction::coefficient(1.0),
                ActiveEvents::COLLISION_EVENTS,
                SummitZone::default(),
            ));
        }
    }
}
