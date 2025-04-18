use bevy::prelude::*;
use std::ops::Range;

#[derive(Debug, Resource)]
pub struct CameraSetting {
    pub viewport_height: f32,
    pub zoom_range: Range<f32>,
    pub zoom_speed: f32,
}
