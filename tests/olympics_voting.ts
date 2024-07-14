import * as anchor from '@coral-xyz/anchor'
import { Program } from '@coral-xyz/anchor'
import { expect } from 'chai'
import { OlympicsVoting } from '../target/types/olympics_voting'

describe('olympics_voting', async () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env()
  anchor.setProvider(provider)

  const program = anchor.workspace.OlympicsVoting as Program<OlympicsVoting>
  const dao_account = anchor.web3.Keypair.generate()
  console.log('dao_account', dao_account.publicKey)

  const dao_name = 'Superteam'
  const proposal_title = 'Proposal title here'
  const proposal_description =
    'Hello world, this is a proposal, I am proposing for the implementation of bla and bla and bla, it will be a good bla for this dao community bla bla'
  const current_date = new Date()
  current_date.setDate(current_date.getDate() + 7)
  const unixTimestamp = new anchor.BN(Math.floor(current_date.getTime() / 1000)) // Convert to seconds

  const [proposalPDA, proposalDump] =
    await anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from('proposal'),
        dao_account.publicKey.toBuffer(),
        Buffer.from(proposal_title),
      ],
      program.programId
    )
  console.log('pda', proposalPDA)

  const [voterPda, voterBump] =
    await anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from('voter'),
        proposalPDA.toBuffer(),
        provider.wallet.publicKey.toBuffer(),
      ],
      program.programId
    )

  const [rewardPda, rewardBump] =
    await anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from('reward_account'),
        dao_account.publicKey.toBuffer(),
        provider.wallet.publicKey.toBuffer(),
      ],
      program.programId
    )

  it('Is initialized!', async () => {
    // Add your test here.
    const tx = await program.methods
      .initialize(dao_name)
      .accounts({
        daoAccount: dao_account.publicKey,
        user: provider.wallet.publicKey,
      })
      .signers([dao_account])
      .rpc()

    console.log('Your transaction signature', tx)

    const account = await program.account.daoAccount.fetch(
      dao_account.publicKey
    )

    expect((await account).name).equal(dao_name)
  })

  it('Proposal Created', async () => {
    // Add your test here.
    const tx = await program.methods
      .createProposal(proposal_title, proposal_description, unixTimestamp)
      .accounts({
        daoAccount: dao_account.publicKey,
        user: provider.wallet.publicKey,
      })
      .rpc()

    const account = await program.account.proposal.fetch(proposalPDA)
    console.log('account', account)

    expect(account.title).equal(proposal_title)
    expect(account.description).equal(proposal_description)
    expect(account.votingPeriod.toNumber()).equal(unixTimestamp.toNumber())
    expect(account.noVotes.toNumber()).equal(0)
    expect(account.yesVotes.toNumber()).equal(0)
  })

  it('Cast Vote', async () => {
    const tx = await program.methods
      .castVote(true)
      .accounts({
        proposal: proposalPDA,
        daoAccount: dao_account.publicKey,
        user: provider.wallet.publicKey,
      })
      .rpc()

    const proposalAccount = await program.account.proposal.fetch(proposalPDA)
    const voterAccount = await program.account.voter.fetch(voterPda)
    const rewardPool = await program.account.rewardPool.fetch(rewardPda)

    console.log('proposal account', proposalAccount)
    console.log('voter account', voterAccount)
    console.log('reward pool', rewardPool)

    expect(proposalAccount.yesVotes.toNumber()).equal(1)
    expect(proposalAccount.noVotes.toNumber()).equal(0)
    expect(voterAccount.hasVoted).equal(true)
    expect(voterAccount.vote).equal(true)
    expect(rewardPool.points.toNumber()).equal(1)
  })

  it('Cannot vote twice', async () => {
    try {
      const tx = await program.methods
        .castVote(true)
        .accounts({
          proposal: proposalPDA,
          daoAccount: dao_account.publicKey,
          user: provider.wallet.publicKey,
        })
        .rpc()

      const proposalAccount = await program.account.proposal.fetch(proposalPDA)
      const voterAccount = await program.account.voter.fetch(voterPda)
      const rewardPool = await program.account.rewardPool.fetch(rewardPda)

      console.log('proposal account', proposalAccount)
      console.log('voter account', voterAccount)
      console.log('reward pool', rewardPool)

      expect(proposalAccount.yesVotes.toNumber()).equal(1)
      expect(proposalAccount.noVotes.toNumber()).equal(0)
      expect(voterAccount.hasVoted).equal(true)
      expect(voterAccount.vote).equal(true)
      expect(rewardPool.points.toNumber()).equal(1)
    } catch (error) {
      console.log('You have already voted')
    }
  })
})
