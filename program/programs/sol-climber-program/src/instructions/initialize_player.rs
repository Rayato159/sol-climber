use crate::states::Player;
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
    pub system_program: Program<'info, System>,
}
