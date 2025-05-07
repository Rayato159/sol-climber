use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Equipment {
    pub mint: Pubkey,
    #[max_len(32)]
    pub name: String,
}
