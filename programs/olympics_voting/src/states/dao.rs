use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct DaoAccount {
    #[max_len(20)]
    pub name: String,
    pub owner: Pubkey,
}