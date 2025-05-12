use anyhow::Result;

#[derive(Debug, Clone)]
pub struct DotEnvyConfig {
    pub rpc_url: String,
}

pub fn load() -> Result<DotEnvyConfig> {
    dotenvy::dotenv().ok();

    let rpc_url = std::env::var("RPC_URL").expect("RPC_URL is invalid");

    Ok(DotEnvyConfig { rpc_url })
}
