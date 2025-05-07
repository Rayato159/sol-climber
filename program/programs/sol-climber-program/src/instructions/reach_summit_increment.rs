use anchor_lang::prelude::*;

use crate::states::Player;

#[derive(Accounts)]
pub struct ReachSummitIncrement<'info> {
    #[account(
        mut,
        seeds = [b"player", wallet.key().as_ref()],
        bump
    )]
    pub player: Account<'info, Player>,
    /// CHECK: This is the player's wallet, used only as a seed reference
    pub wallet: UncheckedAccount<'info>,
}
