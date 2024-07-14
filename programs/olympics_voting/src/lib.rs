use anchor_lang::prelude::*;

mod error;
mod instructions;
mod states;

use instructions::*;

use error::*;

declare_id!("2KdyZ7UN3PMWB5GLUSFc1V3dpmF4JTRciRJpERnB6B6x");

#[program]
pub mod olympics_voting {

    use super::*;

    pub fn initialize(ctx: Context<Initialize>, name: String) -> Result<()> {
        ctx.accounts
            .initialize(name, ctx.accounts.user.key.clone())?;
        Ok(())
    }

    pub fn create_proposal(
        ctx: Context<CreateProposal>,
        title: String,
        description: String,
        voting_period: i64,
    ) -> Result<()> {
        ctx.accounts
            .create_proposal(title, description, voting_period)?;

        Ok(())
    }

    pub fn cast_vote(ctx: Context<Vote>, vote: bool) -> Result<()> {
        ctx.accounts
            .cast_vote(vote, ctx.accounts.user.key.clone())?;

        Ok(())
    }
}
