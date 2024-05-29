# Internet Computer Group Wallet Ledger Canister

This project hosts a group wallet ledger canister on the Internet Computer, designed to manage whitelist approvals, transactions, and airdrops through a voting mechanism. It allows multiple parties to participate in the decision-making process for approving critical operations within a decentralized environment.

## Requirements

- Rust
- DFX SDK
- Internet Computer network access

## Installation

Clone the repository and navigate to the directory:

```bash
git clone https://github.com/Catalyze-Software/group_wallet.git
cd group_wallet
```

## Overview

### Owner management

Owner is the principal that has the ability to add/remove/replace users in the whitelist. Owner is
set during the canister installation by the group wallet index canister. Owner can be changed by the
group wallet index canister only. Any authorized user can get the owner principal.

### Whitelisting

Whitelisting is the process of adding a user to the whitelist. Whitelisted users can vote on
proposals. Initially, whitelisted users are set by the group wallet index canister. Whitelisted users
can be added/removed by the owner. Minimum is 2 and maximum whitelisted users amount is 3, including
the owner.

### Voting

Voting is the process of approving or rejecting a proposal. Whitelisted users can vote on proposals.
A proposal is approved if majority (2/3) of the whitelisted users approve it. A proposal is rejected if
majority (2/3) of the whitelisted users reject it. If a proposal is not approved or rejected within
one day (24h), it is considered rejected. A proposal can be voted on only once. A proposal can be created
by owner. There is no need to vote a proposal by the all whitelisted users. Only the majority is enough.

### Executing a proposal

If a proposal is approved, it can be executed by the any whitelisted user. If a proposal is rejected,
it will throw error.

### What should be changed

- Min and max whitelisted users should be 3.
- Proposals could be created by any whitelisted user.

### Proposals

- Owner able to create a proposal by specifying the canister ID and proposal arguments (Airdrop or
  Transfer content)
- Any authorized user can get proposals with the votes and optionally filter by status
- Any authorized user can get votes of a proposal by specifying the proposal ID and optionally filter
  by option
- Any whitelist user can vote on a proposal by specifying the proposal ID and vote
- If a proposal is approved, any whitelist user can execute the proposal by specifying the proposal ID
- If a proposal is rejected, it will throw an error
- If it's an airdrop proposal, any authorized user can get airdrop details by specifying the proposal ID

### Airdrops

Airdrops are the process of transferring tokens to multiple users.

### Transfers

Transfers are the process of transferring tokens to a single user.

## Contributing

Contributions are welcome! Please submit pull requests with new features or bug fixes, or open issues for bugs or feature requests.

## Licence

GPL-2.0
