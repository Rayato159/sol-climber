use anyhow::Result;
use brigde::infrastructure::ntex_http::start_server;
use tracing::info;

#[ntex::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();
    info!("Tracing subscriber initialized");

    start_server().await?;
    info!("Server stopped.");

    Ok(())
}
