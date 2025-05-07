use bevy::{prelude::*, render::mesh::MeshAabb, scene::SceneInstanceReady};

const DEATH_ZONE_PATH: &str = "terrains/DeathZone.glb";

#[derive(Component)]
pub struct DeathZone;

#[derive(Resource, Default)]
pub struct DeathZoneBounds(pub Option<BoundingBox>);

#[derive(Debug, Clone)]
pub struct BoundingBox {
    pub min: Vec3,
    pub max: Vec3,
}

pub fn spawn_death_zone(mut commands: Commands, asset_server: Res<AssetServer>) {
    let death_zone = asset_server.load(GltfAssetLabel::Scene(0).from_asset(DEATH_ZONE_PATH));
    commands
        .spawn((
            Name::new("DeathZone"),
            DeathZone,
            SceneRoot(death_zone),
            Transform::from_translation(Vec3::new(0.0, -5.0, 0.0)),
        ))
        .observe(cache_deathzone_bounds);
}

pub fn cache_deathzone_bounds(
    trigger: Trigger<SceneInstanceReady>,
    children: Query<&Children>,
    meshes: Res<Assets<Mesh>>,
    mesh3d_query: Query<&Mesh3d>,
    global_transform_query: Query<&GlobalTransform>,
    transform: Query<&Transform, With<DeathZone>>,
    mut bounds: ResMut<DeathZoneBounds>,
) {
    for entity in children.iter_descendants(trigger.entity()) {
        let Ok(mesh3d) = mesh3d_query.get(entity) else {
            continue;
        };

        let Ok(global_transform) = global_transform_query.get(entity) else {
            continue;
        };

        if let Some(mesh) = meshes.get(mesh3d.0.id()) {
            for transform in transform.iter() {
                if let Some(aabb) = mesh.compute_aabb() {
                    let aabb_min = global_transform.transform_point(aabb.min().into());
                    let aabb_max = global_transform.transform_point(aabb.max().into());

                    let min =
                        Vec3::new(aabb_min.x, aabb_min.y + transform.translation.y, aabb_min.z);
                    let max =
                        Vec3::new(aabb_max.x, aabb_max.y + transform.translation.y, aabb_max.z);

                    bounds.0 = Some(BoundingBox { min, max });
                }
            }
        }
    }
}
