# Contract Interaction Analytics Substreams

This package tracks contract interactions, events, and creations on Ethereum from January 2023 onwards (starting at block 16308000). It provides detailed analytics about contract usage and wallet interactions.

## Overview

The Contract Reviewer Substreams package analyzes Ethereum blockchain data to extract and process information about smart contract interactions. It tracks:

- Contract usage patterns
- Contract events
- Contract creations
- Wallet interactions

## Data Limitations

For performance and efficiency reasons, this subgraph implements several practical limitations:

- Only processes transactions with gas used above 21,000 (basic ETH transfer)
- Limits processing to a maximum of 1,000 contracts per block
- Stores a maximum of 100 wallet addresses per contract
- Captures only up to 10 interactions per contract in the graph output
- Considers contracts "new" for a window of 1,000 blocks after first interaction
- Indexes data starting from January 2023 (block 16,308,000) for relevance and performance

These limitations ensure efficient indexing and query performance while capturing the most relevant contract activity data.

## Modules

### map_block_index

Indexes Ethereum blocks for use by other modules. This module simply passes through the block for use by other modules, particularly the contract creation module.

### map_contract_events

Extracts contract events from transaction logs. This module processes all transaction logs in a block to identify and extract contract events. Each event includes the contract address, wallet address, transaction hash, log index, and event signature.

### map_contract_creation

Detects contract creations from transactions. This module identifies contract creation transactions and extracts information about newly created contracts, including the creator address, contract address, and contract bytecode.

### map_contract_usage

Analyzes contract usage patterns from transactions. This module tracks contract interactions, unique wallets, and call counts. It also incorporates data from contract events and creations to provide a comprehensive view of contract usage.

### store_contract_stats

Stores contract usage statistics. This module maintains a store of contract usage data, including interaction counts, unique wallet counts, and first/last interaction blocks.

### store_daily_stats

Aggregates daily statistics for contracts. This module maintains daily aggregated statistics across all contracts, including active contracts, new contracts, total calls, and unique wallets.

### graph_out

Generates entity changes for The Graph. This module transforms contract data into entity changes that can be consumed by a subgraph. It creates entities for contracts, wallets, interactions, events, and statistics.

## Entities

- **Contract**: Information about a smart contract, including usage statistics
- **Wallet**: Information about a wallet that interacts with contracts
- **Interaction**: Record of a wallet interacting with a contract
- **ContractEvent**: Detailed information about events emitted by contracts
- **ContractCreation**: Information about contract creation transactions
- **DailyContractStat**: Daily statistics for a specific contract
- **DailyStat**: Aggregated daily statistics across all contracts

## Usage

This Substreams package can be used with The Graph to create a subgraph that indexes contract interactions, events, and creations.


