use bevy::prelude::*;

use crate::{entities::player::Player, terrains::death_zone::DeathZoneBounds};

pub fn player_in_death_zone_check(
    mut player_query: Query<&mut Transform, With<Player>>,
    bounds: Res<DeathZoneBounds>,
) {
    for mut transform in player_query.iter_mut() {
        if let Some(bbox) = &bounds.0 {
            if transform.translation.y <= bbox.min.y {
                transform.translation = Vec3::new(0.0, 0.3, 0.0);
            }
        }
    }
}
