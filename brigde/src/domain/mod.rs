use anyhow::Result;

#[async_trait::async_trait]
#[mockall::automock]
pub trait SolClimberOnChain {
    async fn initialize_player(&self, initialize_player_req: InitializePlayerReq) -> Result<()>;
    async fn summit_record(&self, player_address: &str) -> Result<u32>;
    async fn death_record(&self, player_address: &str) -> Result<u32>;
    async fn nft_minting(&self) -> Result<()>;
}

#[derive(Debug, Clone)]
pub struct InitializePlayerReq {
    pub program_id: &'static str,
}
