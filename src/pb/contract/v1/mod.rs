// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContractUsage {
    #[prost(string, tag="1")]
    pub address: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub first_interaction_block: u64,
    #[prost(uint64, tag="3")]
    pub last_interaction_block: u64,
    #[prost(uint64, tag="4")]
    pub total_calls: u64,
    #[prost(uint64, tag="5")]
    pub unique_wallets: u64,
    #[prost(string, repeated, tag="6")]
    pub interacting_wallets: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(bool, tag="7")]
    pub is_new_contract: bool,
    #[prost(uint64, tag="8")]
    pub day_timestamp: u64,
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WalletInteraction {
    #[prost(string, tag="1")]
    pub wallet_address: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub interaction_count: u64,
    #[prost(uint64, tag="3")]
    pub first_interaction_block: u64,
    #[prost(uint64, tag="4")]
    pub last_interaction_block: u64,
    #[prost(bool, tag="5")]
    pub is_repeat_user: bool,
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContractEvents {
    #[prost(message, repeated, tag="1")]
    pub contracts: ::prost::alloc::vec::Vec<ContractUsage>,
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DailyStats {
    #[prost(uint64, tag="1")]
    pub day_timestamp: u64,
    #[prost(uint64, tag="2")]
    pub active_contracts: u64,
    #[prost(uint64, tag="3")]
    pub new_contracts: u64,
    #[prost(uint64, tag="4")]
    pub total_calls: u64,
    #[prost(uint64, tag="5")]
    pub unique_wallets: u64,
}
