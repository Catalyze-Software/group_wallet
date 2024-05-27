# Internet Computer Group Wallet Ledger Canister

This project hosts a group wallet ledger canister on the Internet Computer, designed to manage whitelist approvals, transactions, and airdrops through a voting mechanism. It allows multiple parties to participate in the decision-making process for approving critical operations within a decentralized environment.

## Features

- **Whitelist Management**: Handle whitelist requests and allow collective voting on these requests.
- **Transaction Processing**: Manage transaction requests with group_wallet approvals.
- **Airdrop Functionality**: Support airdrop requests and manage related errors and transactions.

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

## Usage Details

### Whitelist Operations

- **Request Whitelist**: Submit a request to be added to the whitelist by specifying the request type.
- **Get Whitelist Requests**: Retrieve all whitelist requests filtered by status.
- **Vote on Whitelist Request**: Vote on specific whitelist requests using the request ID and your vote type.
- **Retrieve Whitelist**: Get a list of all principals currently whitelisted.

### Transaction Management

- **Submit Transaction Request**: Request a transaction by specifying the canister ID and the transaction type. Await multisig approval.
- **Get Transaction Requests**: Fetch transaction requests based on their status.
- **Vote on Transaction Request**: Participate in voting for transaction approvals.

## Airdrop Features

- **Request Airdrop**: Initiate an airdrop request by specifying the canister ID and transfer arguments. Airdrops require multisig approval.
- **Get Airdrop Requests**: View all airdrop requests filtered by status.
- **Get Airdrop Errors**: Retrieve errors related to specific airdrop requests.
- **Vote on Airdrop Request**: Cast your vote on airdrop requests to decide their outcome.
- **Retrieve Airdrop Transaction Details**: Obtain transaction details for specific airdrop requests.

### Contributing

Contributions are welcome! Please submit pull requests with new features or bug fixes, or open issues for bugs or feature requests.

### Licence

GPL-2.0
