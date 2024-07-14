use anchor_lang::prelude::*;

use crate::states::DaoAccount;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + DaoAccount::INIT_SPACE)]
    pub dao_account: Account<'info, DaoAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> Initialize<'info> {
    pub fn initialize(&mut self, name: String, owner: Pubkey) -> Result<()> {
        let dao_account = &mut self.dao_account;
        dao_account.name = name;
        dao_account.owner = owner;

        Ok(())
    }
}
