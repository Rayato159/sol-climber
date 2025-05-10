use anchor_lang::prelude::*;

declare_id!("C17Vg2mNNQ6tjYLFUuheUfQwQoGdopf6dW2goEbWLtM6");

pub mod errors;
pub mod instructions;
pub mod states;

use errors::*;
use instructions::*;

#[program]
pub mod sol_climber_program {
    use anchor_spl::token::{mint_to, MintTo};
    use mpl_token_metadata::{
        instructions::{
            CreateMasterEditionV3Cpi, CreateMasterEditionV3CpiAccounts,
            CreateMasterEditionV3InstructionArgs, CreateMetadataAccountV3Cpi,
            CreateMetadataAccountV3CpiAccounts, CreateMetadataAccountV3InstructionArgs,
        },
        types::{Creator, DataV2},
    };

    use super::*;

    pub fn initialize_player(ctx: Context<InitializePlayer>) -> Result<()> {
        let player = &mut ctx.accounts.player;

        player.initialize();

        Ok(())
    }

    pub fn dead_increment(ctx: Context<DeadIncrement>) -> Result<()> {
        let player = &mut ctx.accounts.player;
        player.dead_increment();
        Ok(())
    }

    pub fn reach_summit_increment(ctx: Context<ReachSummitIncrement>) -> Result<()> {
        let player = &mut ctx.accounts.player;
        player.reach_summit_increment();
        Ok(())
    }

    pub fn mint_nft_to_player(
        ctx: Context<MintNftToPlayer>,
        name: String,
        symbol: String,
        uri: String,
    ) -> Result<()> {
        // ✅ Validate input
        require!(name.len() <= 32, MintFailed::NameIsTooLong);
        require!(symbol.len() <= 10, MintFailed::SymbolIsTooLong);
        require!(uri.len() <= 200, MintFailed::UriIsTooLong);

        // ✅ Create mint account
        let cpi_accounts = MintTo {
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.ata.to_account_info(),
            authority: ctx.accounts.payer.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);
        mint_to(cpi_ctx, 1)?;

        // ✅ Create metadata
        // Create creators list (use payer as the creator)
        let creators = vec![Creator {
            address: ctx.accounts.payer.key(),
            verified: false,
            share: 100,
        }];

        // Perform CPI to create the Metadata account
        CreateMetadataAccountV3Cpi::new(
            &ctx.accounts.metadata_program,
            CreateMetadataAccountV3CpiAccounts {
                metadata: &ctx.accounts.metadata,
                mint: &ctx.accounts.mint.to_account_info(),
                mint_authority: &ctx.accounts.payer.to_account_info(),
                payer: &ctx.accounts.payer,
                update_authority: (&ctx.accounts.payer.to_account_info(), true),
                system_program: &ctx.accounts.system_program,
                rent: Some(&ctx.accounts.rent.to_account_info()),
            },
            CreateMetadataAccountV3InstructionArgs {
                data: DataV2 {
                    name: name.clone(),
                    symbol,
                    uri,
                    seller_fee_basis_points: 0,
                    creators: Some(creators),
                    collection: None,
                    uses: None,
                },
                is_mutable: false,
                collection_details: None,
            },
        )
        .invoke()
        .map_err(|e| {
            msg!("Failed to create Metadata account: {:?}", e);
            MintFailed::MetadataCreationFailed
        })?;

        msg!(
            "NFT minted with mint: {} and metadata: {}",
            ctx.accounts.mint.key(),
            ctx.accounts.metadata.key()
        );

        CreateMasterEditionV3Cpi::new(
            &ctx.accounts.metadata_program,
            CreateMasterEditionV3CpiAccounts {
                edition: &ctx.accounts.master_edition,
                mint: &ctx.accounts.mint.to_account_info(),
                update_authority: &ctx.accounts.payer.to_account_info(),
                mint_authority: &ctx.accounts.payer.to_account_info(),
                payer: &ctx.accounts.payer.to_account_info(),
                metadata: &ctx.accounts.metadata,
                token_program: &ctx.accounts.token_program,
                system_program: &ctx.accounts.system_program,
                rent: Some(&ctx.accounts.rent.to_account_info()),
            },
            CreateMasterEditionV3InstructionArgs {
                max_supply: Some(0),
            },
        )
        .invoke()
        .map_err(|e| {
            msg!("Failed to create Master Edition: {:?}", e);
            MintFailed::MasterEditionCreationFailed
        })?;

        msg!(
            "Master Edition created with mint: {} and metadata: {}",
            ctx.accounts.mint.key(),
            ctx.accounts.metadata.key()
        );

        Ok(())
    }
}
