# DAO Voting Program

A decentralize autonomous organization voting program for proposals using rust's [anchor-lang](https://www.anchor-lang.com) to facilitate the creation and voting on proposals and rewarding users for voting.

# Technologies Used

- [Rust 🦀](https://www.rust-lang.org)
- [Anchor-Lang ⚓︎](https://www.anchor-lang.com)
- [Yarn](https://yarnpkg.com)
- [Typescript](https://www.typescriptlang.org)
- [Node](http://nodejs.org)

# Deliverables

- [x] Create DAO Account
- [x] Create proposal
- [X] Vote on proposal
- [x] Reward pool for participation
- [ ] ZKP 

# Setting Up and installations

> [!NOTE]  
> Make sure you have rust, [solana-cli](https://docs.solanalabs.com/cli/install), - [anchor-cli ⚓︎](https://www.anchor-lang.com/docs/installation) and nodejs's yarn (run `npm i -g yarn` if missing) package manager installed.

### Clone repository
```bash
git clone https://github.com/ahmad940/dao_voting
cd dao_voting
```

### Install node package
```bash
yarn
```

### Build The program [optional]
```bash
anchor test # this will download crates, compile and build the rust anchor program and run the tests
```

### Test The contract
```bash
# this will download crates, compile and build the rust anchor program if not build and run the tests
anchor test 
```

### Deploying The contract
```bash
# deploy the contact
anchor deploy 
```

# Porgram stucture
The anchor program is broken into modules for cleaner code and easy readability approach
```md
Anchor program
├── src
│   ├── instructions
│   └── states
├── lib.rs

└── // ...etc
```
### instruction module

This abstract away the contract accounts instructions

### state module

This abstract away the contract accounts states

# Usage
The methods and how to use this contract on client can be found in the `./tests/olympics_voting.ts`

Additionally these are the methods and their payload of this contract as in the lib.rs
```ts
# initialize and create a dao account
initialize(name: string)
```

```ts
# create a proposal
createProposal(proposal_title: string, proposal_description: string, proposal_period: anchor.BN #time in unix converted to BN)
```
```ts
# cast vote (true or false)
castVote(vote: boolean #true or false)
```

# Youtube Video
https://www.youtube.com/watch?v=w634Wh15vEo