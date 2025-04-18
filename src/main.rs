use bevy::prelude::*;
use sol_climber::{components, resources::camera_setting::CameraSetting, systems};

fn main() {
    let camera_settings = CameraSetting {
        viewport_height: 5.,
        zoom_range: 0.5..2.0,
        zoom_speed: 0.1,
    };

    let spawn_bundle = (
        systems::camera::spawn_camera,
        systems::map::spawn_floor,
        systems::light::spawn_light,
    );

    let camera_system_bundle = (
        systems::camera::camera_movement,
        systems::camera::camera_zoom,
    );

    App::new()
        .insert_resource(camera_settings)
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, spawn_bundle)
        .add_systems(Update, camera_system_bundle)
        .run();
}
