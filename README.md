# Contract Interaction Analytics Substreams

This Substreams package tracks smart contract interactions, events, and creations on Ethereum from **January 2023 onward** (starting at block `16,308,000`). It provides deep analytics into how contracts are used and how wallets interact with them.

---

## Overview

The **Contract Reviewer Substreams** package analyzes Ethereum data to extract and structure:

✅ Contract usage patterns  
✅ Wallet interaction summaries  
✅ Daily on-chain activity trends  


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

Contract
Represents a smart contract that has received calls. Tracks usage metrics:

Total number of calls

Unique wallet count

First and last block interactions

Wallet
Represents a wallet address that has interacted with one or more contracts.

Tracks total interaction count

DailyStat
System-wide daily activity snapshot:

Active contracts

New contracts

Total calls

Unique wallets


---

## 🔍 Sample Queries

### 1. 📅 Daily Overview of Network Activity (Last 5 Days)
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

```

### 2. 🧠 Top 10 most-active contracts
```graphql
{
  contracts(first: 10, orderBy: totalCalls, orderDirection: desc) {
    address
    totalCalls
    uniqueWallets
    firstInteractionBlock
    lastInteractionBlock
  }
}

```
### 3. 👛 Stats for a Specific Wallet
```graphql
{
  wallet(id: "0xabc123...") {
    id
    totalInteractions
  }
}
```

### 4. 🆕 Recently Active Contracts
```graphql
{
  contracts(first: 10, orderBy: lastInteractionBlock, orderDirection: desc) {
    address
    lastInteractionBlock
    totalCalls
  }
}

```

### 5. 🔁 Total Number of Unique Wallets Seen Per Day
```graphql
{
  dailyStats(first: 10, orderBy: dayTimestamp, orderDirection: desc) {
    dayTimestamp
    uniqueWallets
  }
}

```
