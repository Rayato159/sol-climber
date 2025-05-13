pub mod routes;

use std::sync::Arc;

use anyhow::Result;
use ntex::{http, web};
use ntex_cors::Cors;

use crate::application::SolClimberUseCase;

use super::anchor::SolClimberAnchorClient;

pub async fn start_server() -> Result<()> {
    let sol_climber_use_case = {
        let sol_climber_anchor_client = SolClimberAnchorClient::default();
        SolClimberUseCase::new(sol_climber_anchor_client)
    };
    let sol_climber_use_case_artifact = Arc::new(sol_climber_use_case);

    web::HttpServer::new(move || {
        web::App::new()
            .wrap(
                Cors::new()
                    .send_wildcard()
                    .allowed_methods(vec![
                        http::Method::GET,
                        http::Method::POST,
                        http::Method::PUT,
                        http::Method::PATCH,
                        http::Method::DELETE,
                    ])
                    .finish(),
            )
            .state(Arc::clone(&sol_climber_use_case_artifact))
            .default_service(web::route().to(not_found))
            .route("/health-check", web::get().to(health_check))
            .configure(routes::player)
            .configure(routes::statistics_recorder)
            .configure(routes::nft)
    })
    .workers(1)
    .bind(("0.0.0.0", 8080))?
    .run()
    .await?;

    Ok(())
}

async fn not_found() -> impl web::Responder {
    web::HttpResponse::NotFound().body("Not Found")
}

async fn health_check() -> impl web::Responder {
    web::HttpResponse::Ok().body("OK")
}
