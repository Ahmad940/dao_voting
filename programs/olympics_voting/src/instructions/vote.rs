use anchor_lang::prelude::*;

use crate::{
    states::{DaoAccount, Proposal, RewardPool, Voter},
    CustomError,
};

#[derive(Accounts)]
pub struct Vote<'info> {
    #[account(mut)]
    pub proposal: Account<'info, Proposal>,

    #[account(mut)]
    pub dao_account: Account<'info, DaoAccount>,

    #[account(
        init,
        seeds = [b"voter", proposal.key().as_ref(), user.key().as_ref()],
        bump,
        payer = user,
        space = 8 + Voter::INIT_SPACE,
    )]
    pub voter: Account<'info, Voter>,

    #[account(
        init,
        seeds = [b"reward_account", dao_account.key().as_ref(), user.key().as_ref()],
        bump,
        payer = user,
        space = 8 + RewardPool::INIT_SPACE,
    )]
    pub reward_account: Account<'info, RewardPool>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,
}

impl<'info> Vote<'info> {
    pub fn cast_vote(&mut self, vote: bool, owner: Pubkey) -> Result<()> {
        let proposal = &mut self.proposal;
        let voter = &mut self.voter;
        let reward_pool = &mut self.reward_account;

        if voter.has_voted {
            return Err(CustomError::HasVoted.into());
        }

        match vote {
            true => proposal.yes_votes += 1,
            false => proposal.no_votes += 1,
        }

        voter.has_voted = true;
        voter.vote = vote;
        reward_pool.points += 1;
        reward_pool.owner = owner;

        Ok(())
    }
}
