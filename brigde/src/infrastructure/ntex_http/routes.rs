use std::sync::Arc;

use ntex::web;
use tracing::error;

use crate::{application::SolClimberUseCase, infrastructure::anchor::SolClimberAnchorClient};

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
