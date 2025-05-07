use crate::states::{Inventory, Player};
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct InitializePlayer<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        init,
        payer = signer,
        seeds = [b"player", signer.key().as_ref()],
        bump,
        space = 8 + Player::INIT_SPACE,
    )]
    pub player: Account<'info, Player>,

    #[account(
        init,
        payer = signer,
        seeds = [b"inventory", signer.key().as_ref()],
        bump,
        space = 8 + Inventory::INIT_SPACE,
    )]
    pub inventory: Account<'info, Inventory>,
    pub system_program: Program<'info, System>,
}
