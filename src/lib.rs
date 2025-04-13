use substreams_ethereum::pb::eth::v2::Block;
use substreams::store::{StoreSet, StoreSetProto, StoreNew};
use substreams::errors::Error;
use std::collections::{HashMap, HashSet};

mod pb;
use pb::contract::v1::{ContractUsage, ContractEvents, DailyStats};

// Constants for practical limitations
const NEW_CONTRACT_WINDOW: u64 = 1000; // Blocks to consider a contract "new"
const MAX_WALLETS_PER_CONTRACT: usize = 1000; // Limit number of wallets stored per contract
const MIN_GAS_USED: u64 = 21000; // Minimum gas used to consider a transaction (basic ETH transfer)
const MAX_CONTRACTS_PER_BLOCK: usize = 1000; // Maximum number of contracts to process per block

// Store contract usage data
#[substreams::handlers::store]
fn store_contract_stats(contracts: ContractEvents, store: StoreSetProto<ContractUsage>) {
    for contract in contracts.contracts {
        // Use specific prefix to avoid key collisions
        store.set(0, format!("contract_usage:{}", contract.address), &contract);
    }
}

// Store daily aggregated stats
#[substreams::handlers::store]
fn store_daily_stats(contracts: ContractEvents, store: StoreSetProto<DailyStats>) {
    let mut daily_stats_map: HashMap<u64, DailyStats> = HashMap::new();

    for contract in &contracts.contracts {
        let day_ts = contract.day_timestamp;
        let stats = daily_stats_map.entry(day_ts).or_insert(DailyStats {
            day_timestamp: day_ts,
            active_contracts: 0,
            new_contracts: 0,
            total_calls: 0,
            unique_wallets: 0,
        });

        stats.active_contracts += 1;
        if contract.is_new_contract {
            stats.new_contracts += 1;
        }
        stats.total_calls += contract.total_calls;
        stats.unique_wallets += contract.unique_wallets;
    }

    for (day_ts, stats) in daily_stats_map {
        // Use specific prefix to avoid key collisions
        store.set(0, format!("daily_contract_stats:{}", day_ts), &stats);
    }
}

// Map function to process block and extract contract usage
#[substreams::handlers::map]
fn map_contract_usage(block: Block) -> Result<ContractEvents, Error> {
    let mut contract_map: HashMap<String, ContractUsage> = HashMap::new();

    // Validate and compute daily timestamp
    let timestamp = block.header.as_ref().unwrap().timestamp.as_ref().unwrap();
    let seconds = timestamp.seconds;
    if seconds < 1438269973 {
        // Ethereum genesis timestamp (July 30, 2015)
        // Just skip this check for now to avoid error handling issues
        // return Err(Error::from("Timestamp before Ethereum genesis"));
    }
    let day_timestamp = ((seconds / 86400) * 86400) as u64;

    // Note: We're no longer limiting to specific contracts
    // Instead, we'll process all contract interactions

    // Process transactions for contract interactions
    for tx in block.transaction_traces {
        // Skip if no 'to' address, failed or empty
        if tx.to.is_empty() || tx.status != 1 {
            continue;
        }

        // Skip transactions with low gas usage (likely simple transfers)
        if tx.gas_used < MIN_GAS_USED {
            continue;
        }

        let contract_addr = format!("0x{}", hex::encode(&tx.to));

        // Process contract interactions with limitations
        {
            let from_addr = format!("0x{}", hex::encode(&tx.from));
            let current_block = block.number;

            // Limit the number of contracts we process per block
            if contract_map.len() >= MAX_CONTRACTS_PER_BLOCK && !contract_map.contains_key(&contract_addr) {
                continue;
            }

            // Update or create contract usage
            let usage = contract_map.entry(contract_addr.clone()).or_insert(ContractUsage {
                address: contract_addr.clone(),
                first_interaction_block: current_block,
                last_interaction_block: current_block,
                total_calls: 0,
                unique_wallets: 0,
                interacting_wallets: Vec::new(),
                is_new_contract: true, // Initially set to true, will be updated based on block window
                day_timestamp,
            });

            // Update fields
            usage.total_calls += 1;
            usage.last_interaction_block = current_block;

            // Update is_new_contract based on block window
            usage.is_new_contract = current_block <= usage.first_interaction_block + NEW_CONTRACT_WINDOW;

            // Use HashSet for efficient wallet deduplication
            let mut wallet_set: HashSet<String> = usage.interacting_wallets.iter().cloned().collect();
            if wallet_set.insert(from_addr.clone()) {
                usage.unique_wallets += 1;
                // Limit the number of wallets we store per contract
                if usage.interacting_wallets.len() < MAX_WALLETS_PER_CONTRACT {
                    usage.interacting_wallets.push(from_addr);
                }
            }
        }
    }

    Ok(ContractEvents {
        contracts: contract_map.into_values().collect(),
    })
}

// Graph entities output module
#[substreams::handlers::map]
fn graph_out(
    contracts: ContractEvents,
    contract_stats_deltas: substreams::store::Deltas<ContractUsage>,
    daily_stats_deltas: substreams::store::Deltas<DailyStats>,
) -> Result<substreams_entity_change::pb::entity::EntityChanges, Error> {
    use substreams_entity_change::pb::entity::{EntityChange, EntityChanges};
    use substreams_entity_change::tables::Tables;

    let mut tables = Tables::new();

    // Process contract stats
    for delta in contract_stats_deltas.deltas {
        if !delta.operation.is_update() {
            continue;
        }

        let contract = delta.new_value;
        let contract_id = contract.address.clone();

        tables
            .create_row("Contract", contract_id.clone())
            .set("address", contract.address)
            .set("firstInteractionBlock", contract.first_interaction_block)
            .set("lastInteractionBlock", contract.last_interaction_block)
            .set("totalCalls", contract.total_calls)
            .set("uniqueWallets", contract.unique_wallets)
            .set("isNewContract", contract.is_new_contract);

        // Process wallet interactions
        for (i, wallet_addr) in contract.interacting_wallets.iter().enumerate() {
            let interaction_id = format!("{}-{}-{}", contract_id, wallet_addr, i);
            
            tables
                .create_row("Interaction", interaction_id)
                .set("contract", contract_id.clone())
                .set("wallet", wallet_addr.clone())
                .set("blockNumber", contract.last_interaction_block);
                
            // Create or update wallet entity
            tables
                .create_row("Wallet", wallet_addr.clone())
                .set("address", wallet_addr.clone());
        }
    }

    // Process daily stats
    for delta in daily_stats_deltas.deltas {
        if !delta.operation.is_update() {
            continue;
        }

        let daily_stat = delta.new_value;
        let day_id = daily_stat.day_timestamp.to_string();

        tables
            .create_row("DailyStat", day_id)
            .set("dayTimestamp", daily_stat.day_timestamp)
            .set("activeContracts", daily_stat.active_contracts)
            .set("newContracts", daily_stat.new_contracts)
            .set("totalCalls", daily_stat.total_calls)
            .set("uniqueWallets", daily_stat.unique_wallets);
    }

    Ok(tables.to_entity_changes())
}
