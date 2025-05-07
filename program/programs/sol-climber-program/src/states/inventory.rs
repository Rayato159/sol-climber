use anchor_lang::prelude::*;

use super::Equipment;

#[account]
#[derive(InitSpace)]
pub struct Inventory {
    #[max_len(9)]
    pub equipments: Vec<Equipment>,
}

impl Inventory {
    pub fn initialize(&mut self) {
        self.equipments = Vec::new();
    }
}
