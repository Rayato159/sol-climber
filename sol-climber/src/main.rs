use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier3d::prelude::*;
use sol_climber::{
    entities::{self, player::IsSummitReached},
    resources::camera::{CameraOrbit, CameraSetting},
    systems,
    terrains::{self, death_zone::DeathZoneBounds},
};

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameStartupSet {
    World,
    UI,
    Player,
    Camera,
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameUpdateSet {
    Input,
    Physics,
    Camera,
    Animation,
    PlayerEvent,
}

fn main() {
    App::new()
        .insert_resource(CameraSetting::default())
        .insert_resource(CameraOrbit::default())
        .insert_resource(DeathZoneBounds::default())
        .insert_resource(IsSummitReached::default())
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        // .add_plugins(RapierDebugRenderPlugin::default())
        // .add_plugins(WorldInspectorPlugin::new())
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
            terrains::map::spawn_terrain.in_set(GameStartupSet::World),
        )
        .add_systems(
            Startup,
            terrains::death_zone::spawn_death_zone.in_set(GameStartupSet::World),
        )
        .add_systems(
            Startup,
            terrains::sun::spawn_sun.in_set(GameStartupSet::World),
        )
        .add_systems(
            Startup,
            entities::player::spawn_player.in_set(GameStartupSet::Player),
        )
        .add_systems(
            Startup,
            systems::camera::spawn_3d_camera.in_set(GameStartupSet::Camera),
        )
        // .add_systems(
        //     Startup,
        //     systems::ui::spawn_2d_camera.in_set(GameStartupSet::UI),
        // )
        // 🎮 Runtime Phase
        .configure_sets(
            Update,
            (
                GameUpdateSet::Input,
                GameUpdateSet::Physics,
                GameUpdateSet::Camera,
                GameUpdateSet::Animation,
                GameUpdateSet::PlayerEvent,
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
                terrains::skybox::skybox_loaded,
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
        .add_systems(
            Update,
            (
                systems::summit_zone::check_if_player_reached_summit,
                systems::player_death::player_in_death_zone_check,
            )
                .in_set(GameUpdateSet::PlayerEvent),
        )
        .run();
}
