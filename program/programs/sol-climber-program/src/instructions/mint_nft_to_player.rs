use crate::states::Inventory;
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};
use mpl_token_metadata::ID as METADATA_PROGRAM_ID;

#[derive(Accounts)]
pub struct MintNftToPlayer<'info> {
    /// The payer of the transaction, who pays for the mint and metadata accounts.
    #[account(mut)]
    pub payer: Signer<'info>,

    /// The mint account for the NFT (decimals = 0, supply = 1).
    #[account(
        init,
        payer = payer,
        mint::decimals = 0,
        mint::authority = payer,
        mint::freeze_authority = payer
    )]
    pub mint: Account<'info, Mint>,

    #[account(
        init_if_needed,
        payer = payer,
        associated_token::mint = mint,
        associated_token::authority = payer,
    )]
    pub ata: Account<'info, TokenAccount>,

    /// CHECK: The Metadata account (PDA derived from the mint).
    #[account(mut)]
    pub metadata: UncheckedAccount<'info>,
    /// CHECK: The Metaplex Token Metadata program.
    #[account(address = METADATA_PROGRAM_ID)]
    pub metadata_program: AccountInfo<'info>,
    /// The SPL Token program.
    pub token_program: Program<'info, Token>,
    /// The Solana System program.
    pub system_program: Program<'info, System>,
    /// The SPL Token program.
    pub associated_token_program: Program<'info, AssociatedToken>,
    /// The Rent sysvar.
    pub rent: Sysvar<'info, Rent>,

    #[account(
        mut,
        seeds = [b"inventory", payer.key().as_ref()],
        bump,
    )]
    pub player_inventory: Account<'info, Inventory>,
}
