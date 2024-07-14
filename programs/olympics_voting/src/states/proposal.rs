use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Proposal {
    #[max_len(20)]
    pub title: String,
    #[max_len(200)]
    pub description: String,
    pub voting_period: i64, // Unix timestamp
    pub yes_votes: u64,
    pub no_votes: u64,
}
