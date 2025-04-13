# Contract Subgraph

A subgraph for Ethereum contract usage analytics, powered by substreams.

## Overview

This subgraph indexes Ethereum contract interactions and provides analytics on:

- Contract usage metrics
- Wallet interactions
- Daily statistics

The data is sourced from a substreams package that processes Ethereum blocks and extracts contract usage data.

### Performance Considerations

To handle the large volume of data on Ethereum, the substreams implementation includes several practical limitations:

- Maximum of 1,000 wallets stored per contract (while still tracking the total count)
- Minimum gas usage threshold to filter out simple ETH transfers
- Maximum of 1,000 contracts processed per block
- Contract interactions are tracked within a 1,000 block window for "new contract" designation

## Schema

The subgraph defines the following entities:

- `Contract`: Information about a contract, including usage metrics
- `Wallet`: Information about wallets that interact with contracts
- `Interaction`: Records of interactions between wallets and contracts
- `DailyContractStat`: Daily statistics for each contract
- `DailyStat`: Aggregate daily statistics across all contracts

## Setup

This project has been simplified to make it easier to get started without requiring all the dependencies for building the Rust code.

1. Install Node.js dependencies:

```bash
npm install
```

2. Create a placeholder substreams package:

```bash
./scripts/package-substreams.sh
```

3. Generate code from the GraphQL schema:

```bash
npm run codegen
```

4. Build the subgraph:

```bash
npm run build
```

## Deployment

### Local Deployment

```bash
# Create a local subgraph
npm run create-local

# Deploy to local Graph Node
npm run deploy-local
```

### Studio Deployment

```bash
# Deploy to The Graph Studio
npm run deploy
```

## Advanced: Building the Substreams from Source

If you want to build the actual substreams package from source (not required for initial development):

1. Install Rust and the wasm32 target:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup target add wasm32-unknown-unknown
```

2. Install the Substreams CLI:
```bash
curl -L https://github.com/streamingfast/substreams/releases/download/v1.1.0/substreams_1.1.0_darwin_x86_64.tar.gz | tar zxf -
sudo mv substreams /usr/local/bin
```

3. Build the Rust code:
```bash
cargo build --release --target wasm32-unknown-unknown
```

4. Package the substreams:
```bash
npm run package-substreams
```

This will create a `contract_reviewer-v0.1.0.spkg` file that is referenced in the subgraph manifest.

## Querying

Once deployed, you can query the subgraph using GraphQL. Here's an example query:

```graphql
{
  contracts(first: 10, orderBy: totalCalls, orderDirection: desc) {
    id
    address
    totalCalls
    uniqueWallets
    avgCallsPerWallet
    dailyStats(first: 7, orderBy: dayTimestamp, orderDirection: desc) {
      dayTimestamp
      calls
      uniqueWallets
    }
  }
}
```

## License

MIT
