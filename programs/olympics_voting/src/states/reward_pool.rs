use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct RewardPool {
    pub owner: Pubkey,
    pub points: u64,
}
