use anchor_client::{
    solana_client::rpc_client::RpcClient, solana_sdk::commitment_config::CommitmentConfig,
};
use anyhow::Result;
use brigde::infrastructure::ntex_http::start_server;
use tracing::info;

#[ntex::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();
    info!("Tracing subscriber initialized");

    let dotenvy_config = brigde::config::load()?;
    info!("Dotenvy config loaded.");

    let rpc_client = RpcClient::new_with_commitment(
        dotenvy_config.rpc_url.clone(),
        CommitmentConfig::confirmed(),
    );

    start_server(dotenvy_config, rpc_client).await?;
    info!("Server stopped.");

    Ok(())
}
