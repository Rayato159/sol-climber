use std::collections::HashSet;

use bevy::prelude::*;
use bevy::tasks::AsyncComputeTaskPool;
use bevy_rapier3d::prelude::*;

use crate::{
    entities::player::{IsSummitReached, Player},
    terrains::map::SummitZone,
};

pub fn check_if_player_reached_summit(
    summit_zone_query: Query<Entity, With<SummitZone>>,
    player_query: Query<&Player, With<Player>>,
    mut is_summit_reached: ResMut<IsSummitReached>,
    mut collision_events: EventReader<CollisionEvent>,
) {
    let summit_zone_entities: HashSet<Entity> = summit_zone_query.iter().collect();
    for event in collision_events.read() {
        if let CollisionEvent::Started(e1, e2, _) = event {
            let (player_entity, _) =
                if player_query.get(*e1).is_ok() && summit_zone_entities.contains(e2) {
                    (*e1, *e2)
                } else if player_query.get(*e2).is_ok() && summit_zone_entities.contains(e1) {
                    (*e2, *e1)
                } else {
                    continue;
                };

            let thread_pool = AsyncComputeTaskPool::get();

            if is_summit_reached.0 {
                continue;
            }

            is_summit_reached.0 = true;

            if let Ok(player) = player_query.get(player_entity) {
                let player_address = player.address.clone();

                thread_pool
                    .spawn(async move {
                        let url =
                            format!("http://localhost:8080/statistics/{}/summit", player_address);
                        match ureq::post(&url).send_empty() {
                            Ok(response) if response.status() == 200 => {
                                info!("✅ Summit record sent successfully.");
                            }
                            Ok(response) => {
                                error!("❌ Summit RPC failed: {}", response.status());
                            }
                            Err(e) => {
                                error!("❌ Error sending summit RPC: {:?}", e);
                            }
                        }
                    })
                    .detach();
            }
        }
    }
}
