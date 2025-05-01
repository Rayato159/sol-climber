use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier3d::prelude::*;
use sol_climber::{
    entities,
    resources::camera::{CameraOrbit, CameraSetting},
    systems,
};

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameStartupSet {
    World,
    Player,
    Camera,
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameUpdateSet {
    Input,
    Physics,
    Camera,
    Animation,
}

fn main() {
    App::new()
        .insert_resource(CameraSetting::default())
        .insert_resource(CameraOrbit::default())
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(WorldInspectorPlugin::new())
        .configure_sets(
            Startup,
            (
                GameStartupSet::World,
                GameStartupSet::Player,
                GameStartupSet::Camera,
            )
                .chain(),
        )
        .add_systems(
            Startup,
            systems::terrain::spawn_terrain.in_set(GameStartupSet::World),
        )
        .add_systems(
            Startup,
            systems::sun::spawn_sun.in_set(GameStartupSet::World),
        )
        .add_systems(
            Startup,
            entities::player::spawn_player.in_set(GameStartupSet::Player),
        )
        .add_systems(
            Startup,
            systems::camera::spawn_camera.in_set(GameStartupSet::Camera),
        )
        // 🎮 Runtime Phase
        .configure_sets(
            Update,
            (
                GameUpdateSet::Input,
                GameUpdateSet::Physics,
                GameUpdateSet::Camera,
                GameUpdateSet::Animation,
            )
                .chain(),
        )
        .add_systems(
            Update,
            (
                entities::player::player_movement,
                entities::player::player_jump,
            )
                .in_set(GameUpdateSet::Input),
        )
        .add_systems(
            Update,
            (entities::player::player_ground_check).in_set(GameUpdateSet::Physics),
        )
        .add_systems(
            Update,
            (
                systems::camera::camera_zoom,
                systems::camera::orbit_camera_control,
                systems::camera::camera_follow_orbit_player,
            )
                .in_set(GameUpdateSet::Camera),
        )
        .add_systems(
            Update,
            (
                entities::player::player_rotation,
                entities::player::init_player_animation,
                entities::player::player_jumping_animation,
            )
                .in_set(GameUpdateSet::Animation),
        )
        .run();
}
