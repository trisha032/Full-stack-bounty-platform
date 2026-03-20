# Bounty Platform - Soroban Smart Contract

A decentralized bounty platform built on the Stellar network using Soroban smart contracts. This smart contract enables trustless bounty creation, acceptance, work submission, and completion with automatic reward distribution.

## 📋 Project Description

The **Bounty Platform** is a decentralized application that allows anyone to create bounties with rewards, accept them as workers, submit completed work, and have the bounty creator verify and complete the work to release rewards.

Built with Soroban SDK v25, this contract leverages Stellar's blockchain for secure, transparent, and automated bounty management without requiring intermediaries.

## 🎯 What It Does

The Bounty Platform smart contract facilitates a complete bounty lifecycle:

1. **Create Bounties** - Anyone can create a bounty with a title, description, and reward amount
2. **Accept Bounties** - Workers can accept open bounties to work on them
3. **Submit Work** - Workers submit their completed work with a link/description
4. **Complete Bounties** - Bounty creators verify work and mark as completed
5. **Cancel Bounties** - Creators can cancel open or in-progress bounties

## ✨ Features

### Core Features
- **Trustless Transactions** - No intermediary needed for reward distribution
- **Multiple Bounty States** - Open → InProgress → Submitted → Completed (or Cancelled)
- **Full Audit Trail** - All bounty actions are recorded on-chain
- **Worker Verification** - Only the assigned worker can submit work
- **Creator Authorization** - Only the creator can complete or cancel bounties

### Bounty Lifecycle
```
┌─────────┐     accept      ┌─────────────┐   submit   ┌───────────┐   complete   ┌───────────┐
│  Open   │ ─────────────▶ │ InProgress  │ ─────────▶ │ Submitted │ ───────────▶ │ Completed │
└─────────┘                └─────────────┘            └───────────┘              └───────────┘
     │                           │
     │         cancel            │
     └───────────────────────────┴──────────────────▶ ┌───────────┐
                                                       │ Cancelled │
                                                       └───────────┘
```

### Smart Contract Functions
| Function | Description | Auth Required |
|----------|-------------|---------------|
| `create_bounty` | Create a new bounty with title, description, and reward | Creator |
| `accept_bounty` | Worker accepts an open bounty | Worker |
| `submit_work` | Worker submits completed work | Worker |
| `complete_bounty` | Creator approves work and completes bounty | Creator |
| `cancel_bounty` | Creator cancels an open/in-progress bounty | Creator |
| `get_bounty` | Retrieve bounty details by ID | None |
| `get_all_bounties` | Retrieve all bounties | None |

## 🔧 Contract Details

### Contract Data Types

**BountyStatus**
- `Open` - Bounty available for acceptance
- `InProgress` - Worker has accepted and is working
- `Submitted` - Worker has submitted their work
- `Completed` - Creator approved and bounty is complete
- `Cancelled` - Bounty was cancelled by creator

**Bounty**
- `id` - Unique bounty identifier (u32)
- `title` - Bounty title (String)
- `description` - Detailed description (String)
- `reward` - Reward amount in XLM or tokens (i128)
- `creator` - Bounty creator address
- `worker` - Assigned worker address
- `status` - Current bounty status
- `submission` - Optional work submission link/description

### Build Information
- **WASM Size**: 6,195 bytes (optimized)
- **WASM Hash**: `6c2075c313c1ee13aa2eccbbb1a5dd0f0b2015e729b528b2db1b9d111989ecf5`
- **Exported Functions**: 7

## 📦 Deployment

### Build the Contract
```bash
cd ~/connector/project/contract
stellar contract build
```

### Deploy to Testnet
```bash
# Generate keys
stellar keys generate dev --network testnet --fund

# Deploy contract
stellar contract deploy \
  --wasm target/wasm32v1-none/release/contract.wasm \
  --source-account dev \
  --network testnet
```

### Deploy to Mainnet
```bash
# Generate keys
stellar keys generate prod --network mainnet --fund

# Deploy contract
stellar contract deploy \
  --wasm target/wasm32v1-none/release/contract.wasm \
  --source-account prod \
  --network mainnet
```

## 🧪 Testing

Run the test suite:
```bash
cd ~/connector/project/contract
cargo test
```

**Test Coverage:**
- Create bounty
- Accept bounty
- Submit and complete bounty workflow
- Cancel bounty
- Get all bounties
- Prevent duplicate acceptance
- Handle non-existent bounties

## 🌐 Deployed Smart Contract

| Network | Contract Address |
|---------|------------------|
| **Testnet** | Deploy your contract using the commands above |
| **Mainnet** | Deploy your contract using the commands above |

To get your deployed contract address, run:
```bash
stellar contract deploy \
  --wasm target/wasm32v1-none/release/contract.wasm \
  --source-account <your-key> \
  --network <testnet|mainnet> \
  --save-as deployed.json
```

## 🔗 Resources

- [Soroban Documentation](https://developers.stellar.org/docs/build/smart-contracts/overview)
- [Stellar Developers](https://developers.stellar.org)
- [Stellar SDK](https://stellar-sdk.readthedocs.io/)
- [Freighter Wallet](https://www.freighter.app/)

## 📄 License

This project is open source and available under the MIT License.
