use substreams_ethereum::pb::eth::v2::Block;
use substreams::store::{StoreSet, StoreSetProto, StoreNew};
use substreams::errors::Error;
use std::collections::{HashMap, HashSet};

mod pb;
use pb::contract::v1::{ContractUsage, ContractEvents, DailyStats};

// Constants for practical limitations
const NEW_CONTRACT_WINDOW: u64 = 1000; // Blocks to consider a contract "new"
const MIN_GAS_USED: u64 = 21000; // Minimum gas used to consider a transaction (basic ETH transfer)
const MAX_CONTRACTS_PER_BLOCK: usize = 1000; // Maximum number of contracts to process per block
const MAX_WALLETS_PER_CONTRACT: usize = 100; // Limit number of wallets stored per contract

// Store contract usage data with batched writes
#[substreams::handlers::store]
fn store_contract_stats(contracts: ContractEvents, store: StoreSetProto<ContractUsage>) {
    let mut updates = Vec::new();
    for contract in &contracts.contracts {
        updates.push((format!("contract_usage:{}", contract.address), contract.clone()));
    }
    for (key, value) in updates {
        store.set(0, key, &value);
    }
}

// Store daily aggregated stats using pre-aggregated data
#[substreams::handlers::store]
fn store_daily_stats(contracts: ContractEvents, store: StoreSetProto<DailyStats>) {
    // Create daily stats from contract data
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
    
    // Store the aggregated stats
    let mut updates = Vec::new();
    for (day_ts, stats) in daily_stats_map {
        updates.push((format!("daily_contract_stats:{}", day_ts), stats));
    }
    for (key, value) in updates {
        store.set(0, key, &value);
    }
}

// Map function to index blocks
#[substreams::handlers::map]
fn map_block_index(block: Block) -> Result<Block, Error> {
    // Simply pass through the block for use by other modules
    Ok(block)
}

// Map function to process block and extract contract usage with optimizations
#[substreams::handlers::map]
fn map_contract_usage(block: Block) -> Result<ContractEvents, Error> {
    let mut contract_map: HashMap<String, ContractUsage> = HashMap::new();

    // Validate and compute daily timestamp with robust error handling
    let timestamp = match block.header.as_ref().and_then(|h| h.timestamp.as_ref()) {
        Some(ts) => ts,
        None => {
            substreams::log::info!("Missing block timestamp");
            return Ok(ContractEvents { 
                contracts: Vec::new(),
            });
        }
    };
    
    let seconds = timestamp.seconds;
    // Skip validation to avoid errors, but log a warning
    if seconds < 1438269973 {
        // This is before Ethereum genesis (July 30, 2015)
        // Just log a warning and continue
        substreams::log::info!("Warning: Block timestamp before Ethereum genesis");
    }
    
    let day_timestamp = ((seconds / 86400) * 86400) as u64;
    let current_block = block.number;

    // Process transactions for contract interactions with early filtering
    for tx in block.transaction_traces {
        // Skip invalid transactions early
        if tx.to.is_empty() || tx.status != 1 || tx.gas_used < MIN_GAS_USED {
            continue;
        }

        let contract_addr = format!("0x{}", hex::encode(&tx.to));
        let from_addr = format!("0x{}", hex::encode(&tx.from));

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

        // Track unique wallets efficiently
        let mut wallet_set: HashSet<String> = usage.interacting_wallets.iter().cloned().collect();
        if wallet_set.insert(from_addr.clone()) {
            usage.unique_wallets += 1;
            
            // Limit the number of wallets we store per contract
            if usage.interacting_wallets.len() < MAX_WALLETS_PER_CONTRACT {
                usage.interacting_wallets.push(from_addr);
            }
        }
    }

    Ok(ContractEvents {
        contracts: contract_map.into_values().collect(),
    })
}

// Graph entities output module with optimizations
#[substreams::handlers::map]
fn graph_out(contracts: ContractEvents) -> Result<substreams_entity_change::pb::entity::EntityChanges, Error> {
    use substreams_entity_change::tables::Tables;

    let mut tables = Tables::new();
    
    // Get the current block number for use in IDs
    let block_number = if !contracts.contracts.is_empty() {
        contracts.contracts[0].last_interaction_block
    } else {
        0 // Fallback if no contracts
    };

    // Process contracts
    for contract in &contracts.contracts {
        let contract_id = contract.address.clone();
        
        tables
            .create_row("Contract", contract_id.clone())
            .set("address", contract.address.clone())
            .set("firstInteractionBlock", contract.first_interaction_block)
            .set("lastInteractionBlock", contract.last_interaction_block)
            .set("totalCalls", contract.total_calls)
            .set("uniqueWallets", contract.unique_wallets)
            .set("isNewContract", contract.is_new_contract);
            
        // Process wallet interactions - ensure each interaction has a truly unique ID
        for (i, wallet_addr) in contract.interacting_wallets.iter().enumerate().take(10) { // Limit to 10 interactions per contract
            // Create a truly unique ID for each interaction by including block number and index
            let interaction_id = format!("{}-{}-{}-{}", contract_id, wallet_addr, block_number, i);
            
            tables
                .create_row("Interaction", interaction_id)
                .set("contract", contract_id.clone())
                .set("wallet", wallet_addr.clone())
                .set("blockNumber", contract.last_interaction_block)
                .set("timestamp", contract.day_timestamp);
                
            // Create or update wallet entity
            tables
                .create_row("Wallet", wallet_addr.clone())
                .set("address", wallet_addr.clone())
                .set("contractsInteracted", 1)
                .set("totalInteractions", 1);
        }
        
        // Create DailyContractStat entry with block number for uniqueness
        let daily_contract_stat_id = format!("{}-{}-{}", 
            contract_id, 
            contract.day_timestamp, 
            contract.last_interaction_block
        );
        
        tables
            .create_row("DailyContractStat", daily_contract_stat_id)
            .set("contract", contract_id.clone())
            .set("dayTimestamp", contract.day_timestamp)
            .set("calls", contract.total_calls)
            .set("uniqueWallets", contract.unique_wallets);
    }

    // Aggregate daily stats
    let mut daily_stats_map: HashMap<u64, (u64, u64, u64, u64)> = HashMap::new(); // (active, new, calls, wallets)
    
    for contract in &contracts.contracts {
        let day_ts = contract.day_timestamp;
        let stats = daily_stats_map.entry(day_ts).or_insert((0, 0, 0, 0));
        
        stats.0 += 1; // active_contracts
        if contract.is_new_contract {
            stats.1 += 1; // new_contracts
        }
        stats.2 += contract.total_calls; // total_calls
        stats.3 += contract.unique_wallets; // unique_wallets
    }
    
    // Create DailyStat entries
    for (day_timestamp, (active, new, calls, wallets)) in daily_stats_map {
        // Include block number in the ID to ensure uniqueness
        let day_id = format!("{}-{}", day_timestamp, block_number);
        
        tables
            .create_row("DailyStat", day_id)
            .set("dayTimestamp", day_timestamp)
            .set("activeContracts", active)
            .set("newContracts", new)
            .set("totalCalls", calls)
            .set("uniqueWallets", wallets);
    }

    Ok(tables.to_entity_changes())
}
