use anchor_lang::prelude::*;

use crate::states::{DaoAccount, Proposal};

#[derive(Accounts)]
#[instruction(title: String)]
pub struct CreateProposal<'info> {
    #[account(
        init, 
        payer = user, 
        space = 8 + Proposal::INIT_SPACE, 
        seeds = [b"proposal", dao_account.key().as_ref(), title.as_bytes()], 
        bump,
    )]
    pub proposal: Account<'info, Proposal>,
    pub dao_account: Account<'info, DaoAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

impl <'info> CreateProposal<'info> {
    pub  fn create_proposal(&mut self, title: String, description: String, voting_period: i64,) -> Result<()>{
         let proposal = &mut self.proposal;
         
        proposal.title = title;
        proposal.description = description;
        proposal.yes_votes = 0;
        proposal.no_votes = 0;
        proposal.voting_period = voting_period;

        Ok(())
    }
}