use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::terrains::map::SummitZone;

pub fn check_if_player_reached_summit(
    summit_zone_query: Query<Entity, With<SummitZone>>,
    mut collision_events: EventReader<CollisionEvent>,
) {
    for summit_zone_entity in summit_zone_query.iter() {
        for event in collision_events.read() {
            if let CollisionEvent::Started(e1, e2, _) = event {
                if *e1 == summit_zone_entity || *e2 == summit_zone_entity {
                    info!(
                        "🏁 Player reached the summit zone! Entity: {:?}",
                        summit_zone_entity
                    );
                }
            }
        }
    }
}
