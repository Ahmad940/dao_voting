use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Voter {
    pub has_voted: bool,
    pub vote: bool,
}
