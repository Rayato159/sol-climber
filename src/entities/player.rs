use bevy::prelude::*;
use solana_sdk::pubkey::Pubkey;

#[derive(Debug, Component)]
pub struct Player {
    pub address: String,
}

impl Player {
    pub fn get_address(&self) -> Pubkey {
        Pubkey::from_str_const(self.address.as_str())
    }
}
