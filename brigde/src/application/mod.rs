use anyhow::Result;
use std::sync::Arc;

use crate::domain::{InitializePlayerReq, SolClimberOnChain};

#[derive(Debug, Clone)]
pub struct SolClimberUseCase<T> {
    pub sol_climber_on_chain: T,
}

impl<T> SolClimberUseCase<T>
where
    T: SolClimberOnChain + Send + Sync + 'static,
{
    pub fn new(sol_climber_on_chain: T) -> Self {
        Self {
            sol_climber_on_chain,
        }
    }

    pub async fn initialize_player(
        &self,
        initialize_player_req: InitializePlayerReq,
    ) -> Result<()> {
        self.sol_climber_on_chain
            .initialize_player(initialize_player_req)
            .await
    }

    pub async fn summit_record(&self, player_address: &str) -> Result<u32> {
        self.sol_climber_on_chain
            .summit_record(player_address)
            .await
    }

    pub async fn death_record(&self, player_address: &str) -> Result<u32> {
        self.sol_climber_on_chain.death_record(player_address).await
    }

    pub async fn nft_minting(&self) -> Result<()> {
        todo!()
    }
}
