use anchor_spl::{
    associated_token::ID as ASSOCIATED_TOKEN_PROGRAM_ID,
    associated_token::get_associated_token_address, token::ID as TOKEN_PROGRAM_ID,
};
use anyhow::Result;
use rand::{Rng, SeedableRng, rngs::StdRng};
use std::sync::Arc;
use tracing::info;

use crate::domain::{NFT_COLLECTIONS, PROGRAM_ID, SolClimberOnChain};
use anchor_client::{
    Client, Cluster,
    solana_sdk::{
        commitment_config::CommitmentConfig,
        signature::{Keypair, read_keypair_file},
        signer::Signer,
        system_program::ID as SYSTEM_PROGRAM_ID,
    },
};
use anchor_lang::{prelude::*, solana_program::sysvar::rent::ID as RENT_ID};
use mpl_token_metadata::ID as TOKEN_METADATA_PROGRAM_ID;

declare_program!(sol_climber_program);
use sol_climber_program::{
    accounts::Player,
    client::{accounts, args},
};

const METADATA_SEED: &str = "metadata";

#[derive(Debug, Clone, Default)]
pub struct SolClimberAnchorClient;

#[async_trait::async_trait]
impl SolClimberOnChain for SolClimberAnchorClient {
    async fn initialize_player(&self) -> Result<String> {
        let program_id = PROGRAM_ID.parse::<Pubkey>()?;

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
        let singer_pubkey = signer.pubkey();

        let (player_pda, _bump) =
            Pubkey::find_program_address(&[b"player", singer_pubkey.as_ref()], &program_id);

        let sig = program
            .request()
            .accounts(accounts::InitializePlayer {
                signer: singer_pubkey,
                player: player_pda,
                system_program: SYSTEM_PROGRAM_ID,
            })
            .args(args::InitializePlayer)
            .signer(signer)
            .send()
            .await?;

        info!("Transaction Signature: {}", sig);

        Ok(singer_pubkey.to_string())
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

    async fn mint_nft_to_player(&self) -> Result<String> {
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

        let mint = Arc::new(Keypair::new());
        let mint_pubkey = mint.pubkey();

        let ata = get_associated_token_address(&signer.pubkey(), &mint_pubkey);

        let metadata_seeds = &[
            METADATA_SEED.as_bytes(),
            TOKEN_METADATA_PROGRAM_ID.as_ref(),
            mint_pubkey.as_ref(),
        ];
        let (metadata, _bump1) =
            Pubkey::find_program_address(metadata_seeds, &TOKEN_METADATA_PROGRAM_ID);

        let edition_seeds = &[
            METADATA_SEED.as_bytes(),
            TOKEN_METADATA_PROGRAM_ID.as_ref(),
            mint_pubkey.as_ref(),
            b"edition",
        ];
        let (master_edition, _bump2) =
            Pubkey::find_program_address(edition_seeds, &TOKEN_METADATA_PROGRAM_ID);

        let mut rng = StdRng::from_os_rng();
        let index = rng.random_range(0..NFT_COLLECTIONS.len());
        let selected = &NFT_COLLECTIONS[index];

        let sig = program
            .request()
            .accounts(accounts::MintNftToPlayer {
                payer: signer.pubkey(),
                mint: mint_pubkey,
                ata,
                metadata,
                master_edition,
                metadata_program: TOKEN_METADATA_PROGRAM_ID,
                associated_token_program: ASSOCIATED_TOKEN_PROGRAM_ID,
                rent: RENT_ID,
                token_program: TOKEN_PROGRAM_ID,
                system_program: SYSTEM_PROGRAM_ID,
            })
            .args(args::MintNftToPlayer {
                name: selected.name.to_string(),
                symbol: selected.symbol.to_string(),
                uri: selected.uri.to_string(),
            })
            .signer(mint.clone())
            .signer(signer.clone())
            .send()
            .await?;

        info!("Mint NFT to player with sig: {:?}", sig);

        Ok(selected.name.to_string())
    }
}
