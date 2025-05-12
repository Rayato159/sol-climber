use bevy::{prelude::*, tasks::AsyncComputeTaskPool};

use crate::{entities::player::Player, terrains::death_zone::DeathZoneBounds};

pub fn player_in_death_zone_check(
    mut player_query: Query<(&mut Transform, &Player), With<Player>>,
    bounds: Res<DeathZoneBounds>,
) {
    for (mut transform, player) in player_query.iter_mut() {
        if let Some(bbox) = &bounds.0 {
            if transform.translation.y <= bbox.min.y {
                let thread_pool = AsyncComputeTaskPool::get();
                let player_address = player.address.clone();

                thread_pool
                    .spawn(async move {
                        let url =
                            format!("http://localhost:8080/statistics/{}/death", player_address);
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

                transform.translation = Vec3::new(0.0, 0.3, 0.0);
            }
        }
    }
}
