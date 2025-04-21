# Contract Interaction Analytics Substreams

This Substreams package tracks smart contract interactions, events, and creations on Ethereum from **January 2023 onward** (starting at block `16,308,000`). It provides deep analytics into how contracts are used and how wallets interact with them.

---

## Overview

The **Contract Reviewer Substreams** package analyzes Ethereum data to extract and structure:

- ✅ Contract usage patterns  
- ✅ Smart contract events  
- ✅ Contract creation transactions  
- ✅ Wallet-to-contract interactions  

All data is output in a format consumable by The Graph.

---

## ⚠️ Data Limitations

To ensure high performance and efficient indexing, this package includes practical constraints:

- Ignores transactions with gas used ≤ `21,000` (simple ETH transfers)
- Limits to **1,000 contracts per block**
- Stores up to **100 wallet addresses per contract**
- Outputs a max of **10 interactions per contract**
- Marks contracts as “new” for **1,000 blocks after their first interaction**
- Indexes from **January 2023 (block 16,308,000)** onward

---

## 🧱 Modules

| Module Name           | Description |
|-----------------------|-------------|
| `map_block_index`     | Passes block data downstream for reuse |
| `map_contract_events` | Extracts logs and event metadata per contract |
| `map_contract_creation` | Detects new contract creations + metadata |
| `map_contract_usage`  | Analyzes interaction and call frequency |
| `store_contract_stats`| Stores contract-level aggregated data |
| `store_daily_stats`   | Tracks system-wide daily usage trends |
| `graph_out`           | Outputs all changes to be consumed by The Graph |

---

## 🧬 Entities

### `Contract`
Stores contract metadata and usage stats.  
Tracks events, interactions, and creation history.

### `Wallet`
Tracks wallet-level interactions across contracts, creation activity, and event triggers.

### `Interaction`
Represents an individual interaction (call) between a wallet and a contract.

### `ContractEvent`
Captures emitted events (log entries) triggered by wallets interacting with contracts.

### `ContractCreation`
Logs contract deployment metadata including creator, bytecode, and block info.

### `DailyContractStat`
Tracks per-contract call volume and unique users per day.

### `DailyStat`
System-wide aggregate for active contracts, calls, and unique wallets per day.

---

## 🔍 Sample Queries

### 1. 📅 Get daily stats summary for the last 5 days
```graphql
{
  dailyStats(first: 5, orderBy: dayTimestamp, orderDirection: desc) {
    dayTimestamp
    activeContracts
    newContracts
    totalCalls
    uniqueWallets
  }
}

# 2. 🧠 Top 10 most-called contracts
{
  contracts(first: 10, orderBy: totalCalls, orderDirection: desc) {
    id
    address
    totalCalls
    uniqueWallets
  }
}

# 3. 👛 Wallet interaction history
{
  wallet(id: "0xabc123...") {
    address
    totalInteractions
    contractsInteracted
    interactions {
      contract {
        id
        address
      }
      blockNumber
      timestamp
    }
  }
}

# 4. 🆕 Recently created contracts
{
  contractCreations(first: 5, orderBy: blockNumber, orderDirection: desc) {
    contract {
      id
      address
    }
    creator {
      id
    }
    blockNumber
    timestamp
    transactionHash
  }
}

# 5. 🔁 Recent contract events
{
  contractEvents(first: 10, orderBy: timestamp, orderDirection: desc) {
    contract {
      id
    }
    wallet {
      id
    }
    eventType
    blockNumber
    transactionHash
    logIndex
  }
}
