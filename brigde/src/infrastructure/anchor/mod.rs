use anyhow::Result;
use std::sync::Arc;
use tracing::info;

use crate::domain::{InitializePlayerReq, SolClimberOnChain};
use anchor_client::{
    Client, Cluster,
    solana_sdk::{
        commitment_config::CommitmentConfig,
        signature::{read_keypair_file},
        signer::Signer,
        system_program,
    },
};
use anchor_lang::prelude::*;

declare_program!(sol_climber_program);
use sol_climber_program::{
    accounts::Player,
    client::{accounts, args},
};

// pub struct SolClimberAnchorClient {
//     rpc_client: Arc<RpcClient>,
// }

// impl SolClimberAnchorClient {
//     pub fn new(rpc_client: Arc<RpcClient>) -> Self {
//         Self { rpc_client }
//     }
// }

#[derive(Debug, Clone, Default)]
pub struct SolClimberAnchorClient;

#[async_trait::async_trait]
impl SolClimberOnChain for SolClimberAnchorClient {
    async fn initialize_player(&self, initialize_player_req: InitializePlayerReq) -> Result<()> {
        let program_id = initialize_player_req.program_id.parse::<Pubkey>()?;

        // Fixed the path to the keypair file for POC
        let signer = Arc::new(
            read_keypair_file("/home/ramune/.config/solana/id.json")
                .map_err(|e| anyhow::anyhow!(e.to_string()))?,
        );

        let client = Client::new_with_options(
            Cluster::Devnet,
            signer.clone(),
            CommitmentConfig::confirmed(),
        );
        let program = client.program(program_id)?;

        let (player_pda, _bump) =
            Pubkey::find_program_address(&[b"player", signer.pubkey().as_ref()], &program_id);

        let sig = program
            .request()
            .accounts(accounts::InitializePlayer {
                signer: signer.pubkey(),
                player: player_pda,
                system_program: system_program::ID,
            })
            .args(args::InitializePlayer)
            .signer(signer)
            .send()
            .await?;

        info!("Transaction Signature: {}", sig);

        Ok(())
    }

    async fn summit_record(&self, player_address: &str) -> Result<u32> {
        let player_address = player_address.parse::<Pubkey>()?;
        let program_id = sol_climber_program::ID;

        // Fixed the path to the keypair file for POC
        let signer = Arc::new(
            read_keypair_file("/home/ramune/.config/solana/id.json")
                .map_err(|e| anyhow::anyhow!(e.to_string()))?,
        );

        let client = Client::new_with_options(
            Cluster::Devnet,
            signer.clone(),
            CommitmentConfig::confirmed(),
        );
        let program = client.program(program_id)?;

        let (player_pda, _bump) =
            Pubkey::find_program_address(&[b"player", player_address.as_ref()], &program_id);

        let sig = program
            .request()
            .accounts(accounts::ReachSummitIncrement {
                player: player_pda,
                wallet: player_address,
            })
            .args(args::ReachSummitIncrement)
            .send()
            .await?;

        info!("Transaction Signature: {}", sig);

        let player_account = program.account::<Player>(player_pda).await?;
        info!("Player Account: {:?}", player_account);

        Ok(player_account.summit_count)
    }

    async fn death_record(&self, player_address: &str) -> Result<u32> {
        let player_address = player_address.parse::<Pubkey>()?;
        let program_id = sol_climber_program::ID;

        // Fixed the path to the keypair file for POC
        let signer = Arc::new(
            read_keypair_file("/home/ramune/.config/solana/id.json")
                .map_err(|e| anyhow::anyhow!(e.to_string()))?,
        );

        let client = Client::new_with_options(
            Cluster::Devnet,
            signer.clone(),
            CommitmentConfig::confirmed(),
        );
        let program = client.program(program_id)?;

        let (player_pda, _bump) =
            Pubkey::find_program_address(&[b"player", player_address.as_ref()], &program_id);

        let sig = program
            .request()
            .accounts(accounts::DeadIncrement {
                player: player_pda,
                wallet: player_address,
            })
            .args(args::DeadIncrement)
            .send()
            .await?;

        info!("Transaction Signature: {}", sig);

        let player_account = program.account::<Player>(player_pda).await?;
        info!("Player Account: {:?}", player_account);

        Ok(player_account.summit_count)
    }

    async fn nft_minting(&self) -> Result<()> {
        todo!()
    }
}
