use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Player {
    pub summit_count: u32,
    pub death_count: u32,
}

impl Player {
    pub fn initialize(&mut self) {
        self.summit_count = 0;
        self.death_count = 0;
    }

    pub fn dead_increment(&mut self) {
        self.death_count += 1;
    }

    pub fn reach_summit_increment(&mut self) {
        self.summit_count += 1;
    }
}
