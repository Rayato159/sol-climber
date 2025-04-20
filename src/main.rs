use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use sol_climber::{
    entities,
    resources::camera::{CameraOrbit, CameraSetting},
    systems,
};

fn main() {
    let start_up_bundle = (
        systems::camera::spawn_camera,
        systems::map::spawn_floor,
        systems::sun::spawn_sun,
        systems::fog::spawn_fog,
        entities::player::spawn_player,
    );

    let camera_control = (
        systems::camera::camera_follow_player,
        systems::camera::camera_zoom,
        systems::camera::orbit_camera_control,
        systems::camera::camera_follow_orbit_player,
    );

    let player_physics = (
        entities::player::player_movement,
        entities::player::player_jump,
        entities::player::player_rotation,
        entities::player::player_ground_check,
    );

    App::new()
        .insert_resource(CameraSetting::default())
        .insert_resource(CameraOrbit::default())
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_systems(Startup, start_up_bundle)
        .add_systems(Update, camera_control)
        .add_systems(Update, player_physics)
        .run();
}
