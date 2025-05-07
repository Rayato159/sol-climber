use bevy::prelude::*;
use std::ops::Range;

#[derive(Debug, Resource)]
pub struct CameraSetting {
    pub zoom_range: Range<f32>,
    pub zoom_speed: f32,
}

impl Default for CameraSetting {
    fn default() -> Self {
        Self {
            zoom_range: 0.5..1.0,
            zoom_speed: 0.1,
        }
    }
}

#[derive(Debug, Resource)]
pub struct CameraOrbit {
    pub yaw: f32,
    pub pitch: f32,
    pub radius: f32,
}

impl Default for CameraOrbit {
    fn default() -> Self {
        Self {
            yaw: 0.0,
            pitch: 10_f32.to_radians(),
            radius: 10.0,
        }
    }
}
