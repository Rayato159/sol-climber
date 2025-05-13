use std::sync::Arc;

use ntex::web;
use tracing::error;

use crate::{application::SolClimberUseCase, infrastructure::anchor::SolClimberAnchorClient};

pub fn player(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/player").service(initialize_player));
}

#[web::post("/initialize")]
pub async fn initialize_player(
    sol_climber_use_case: web::types::State<Arc<SolClimberUseCase<SolClimberAnchorClient>>>,
) -> impl web::Responder {
    match sol_climber_use_case.initialize_player().await {
        Ok(player_pubkey) => web::HttpResponse::Ok().body(format!(
            "Player initialized successfully: {}",
            player_pubkey
        )),
        Err(e) => {
            error!("Error: {}", e);
            web::HttpResponse::InternalServerError().body("Internal Server Error")
        }
    }
}

pub fn statistics_recorder(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/statistics")
            .service(summit_record)
            .service(death_record),
    );
}

#[web::post("/{player_address}/summit")]
pub async fn summit_record(
    sol_climber_use_case: web::types::State<Arc<SolClimberUseCase<SolClimberAnchorClient>>>,
    player_address: web::types::Path<String>,
) -> impl web::Responder {
    match sol_climber_use_case
        .summit_record(player_address.as_str())
        .await
    {
        Ok(summit) => web::HttpResponse::Ok().body(format!("Summit record: {}", summit)),
        Err(e) => {
            error!("Error: {}", e);
            web::HttpResponse::InternalServerError().body("Internal Server Error")
        }
    }
}

#[web::post("/{player_address}/death")]
pub async fn death_record(
    sol_climber_use_case: web::types::State<Arc<SolClimberUseCase<SolClimberAnchorClient>>>,
    player_address: web::types::Path<String>,
) -> impl web::Responder {
    match sol_climber_use_case
        .death_record(player_address.as_str())
        .await
    {
        Ok(death) => web::HttpResponse::Ok().body(format!("Death record: {}", death)),
        Err(e) => {
            error!("Error: {}", e);
            web::HttpResponse::InternalServerError().body("Internal Server Error")
        }
    }
}

pub fn nft(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/nft").service(mint_nft_to_player));
}

#[web::post("/mint_nft_to_player")]
pub async fn mint_nft_to_player(
    sol_climber_use_case: web::types::State<Arc<SolClimberUseCase<SolClimberAnchorClient>>>,
) -> impl web::Responder {
    match sol_climber_use_case.mint_nft_to_player().await {
        Ok(item_name) => {
            web::HttpResponse::Ok().body(format!("Mint NFT to player, item: {}", item_name))
        }
        Err(e) => {
            error!("Error: {}", e);
            web::HttpResponse::InternalServerError().body("Internal Server Error")
        }
    }
}
