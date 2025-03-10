use jsonrpsee::{core::RpcResult, proc_macros::rpc};
use solana_account_decoder::UiAccount;
use solana_rpc_client_api::{
    config::{
        RpcAccountInfoConfig, RpcBlocksConfigWrapper, RpcContextConfig, RpcEncodingConfigWrapper,
        RpcGetVoteAccountsConfig, RpcLeaderScheduleConfig, RpcProgramAccountsConfig,
        RpcSendTransactionConfig, RpcSignatureStatusConfig, RpcSignaturesForAddressConfig,
        RpcSimulateTransactionConfig,
    },
    response::{
        OptionalContext, Response as RpcResponse, RpcBlockhash,
        RpcConfirmedTransactionStatusWithSignature, RpcContactInfo, RpcKeyedAccount, RpcPerfSample,
        RpcPrioritizationFee, RpcVoteAccountStatus,
    },
};
use solana_sdk::{
    commitment_config::CommitmentConfig, epoch_info::EpochInfo, pubkey::Pubkey, slot_history::Slot,
};
use solana_transaction_status::{TransactionStatus, UiConfirmedBlock};
use std::collections::HashMap;

#[rpc(server)]
pub trait SolanaRpc {
    // --------------------------
    // Send / Simulate
    // --------------------------

    #[method(name = "sendTransaction")]
    async fn send_transaction(
        &self,
        transaction: String,
        config: Option<RpcSendTransactionConfig>,
    ) -> RpcResult<String>;

    #[method(name = "simulateTransaction")]
    async fn simulate_transaction(
        &self,
        transaction: String,
        config: Option<RpcSimulateTransactionConfig>,
    ) -> RpcResult<String>;

    // // --------------------------
    // // Archival
    // // --------------------------
    //
    // #[method(name = "getBlock")]
    // async fn get_block(&self, slot: u64) -> RpcResult<Option<UiConfirmedBlock>>;
    //
    // #[method(name = "getBlocks")]
    // async fn get_blocks(
    //     &self,
    //     start_slot: Slot,
    //     config: Option<RpcBlocksConfigWrapper>,
    //     commitment: Option<CommitmentConfig>,
    // ) -> RpcResult<Vec<Slot>>;
    //
    // #[method(name = "getSignaturesForAddress")]
    // async fn get_signatures_for_address(
    //     &self,
    //     address: String,
    //     config: Option<RpcSignaturesForAddressConfig>,
    // ) -> RpcResult<Vec<RpcConfirmedTransactionStatusWithSignature>>;
    //
    // #[method(name = "getTransaction")]
    // async fn get_transaction(
    //     &self,
    //     signature_str: String,
    //     config: Option<RpcEncodingConfigWrapper<RpcTransactionConfig>>,
    // ) -> Result<Option<EncodedConfirmedTransactionWithStatusMeta>>;
    //
    // // --------------------------
    // // Cluster
    // // --------------------------
    //
    // #[method(name = "getClusterNodes")]
    // async fn get_cluster_nodes(&self) -> RpcResult<Vec<RpcContactInfo>>;
    //
    // // --------------------------
    // // Validator
    // // --------------------------
    //
    // #[method(name = "getSlot")]
    // async fn get_slot(&self, config: Option<RpcContextConfig>) -> RpcResult<Slot>;
    //
    // #[method(name = "getBlockHeight")]
    // async fn get_block_height(&self, config: Option<RpcContextConfig>) -> RpcResult<u64>;
    //
    // #[method(name = "getBlockTime")]
    // async fn get_block_time(&self, block: u64) -> RpcResult<u64>;
    //
    // #[method(name = "getFirstAvailableBlock")]
    // async fn get_first_available_block(&self) -> RpcResult<u64>;
    //
    // #[method(name = "getLatestBlockhash")]
    // async fn get_latest_blockhash(
    //     &self,
    //     config: Option<RpcContextConfig>,
    // ) -> RpcResult<RpcResponse<RpcBlockhash>>;
    //
    // #[method(name = "isBlockhashValid")]
    // async fn is_blockhash_valid(
    //     &self,
    //     blockhash: String,
    //     config: Option<IsBlockHashValidConfig>,
    // ) -> RpcResult<RpcResponse<bool>>;
    //
    // #[method(name = "getBlockCommitment")]
    // async fn get_block_commitment(
    //     &self,
    //     block: u64,
    // ) -> Result<RpcBlockCommitment<BlockCommitmentArray>>;
    //
    // #[method(name = "getRecentPerformanceSamples")]
    // async fn get_recent_performance_samples(
    //     &self,
    //     limit: Option<usize>,
    // ) -> RpcResult<Vec<RpcPerfSample>>;
    //
    // #[method(name = "getSignatureStatuses")]
    // async fn get_signature_statuses(
    //     &self,
    //     signature_strs: Vec<String>,
    //     config: Option<RpcSignatureStatusConfig>,
    // ) -> RpcResult<RpcResponse<Vec<Option<TransactionStatus>>>>;
    //
    // #[method(name = "getRecentPrioritizationFees")]
    // async fn get_recent_prioritization_fees(
    //     &self,
    //     pubkey_strs: Vec<String>,
    // ) -> RpcResult<Vec<RpcPrioritizationFee>>;
    //
    // #[method(name = "getEpochInfo")]
    // async fn get_epoch_info(&self, config: Option<RpcContextConfig>) -> RpcResult<EpochInfo>;
    //
    // #[method(name = "getLeaderSchedule")]
    // async fn get_leader_schedule(
    //     &self,
    //     slot: Option<u64>,
    //     config: Option<RpcLeaderScheduleConfig>,
    // ) -> RpcResult<Option<HashMap<String, Vec<usize>>>>;
    //
    // #[method(name = "getSlotLeaders")]
    // async fn get_slot_leaders(&self, start_slot: u64, limit: u64) -> RpcResult<Vec<Pubkey>>;
    //
    // #[method(name = "getVoteAccounts")]
    // async fn get_vote_accounts(
    //     &self,
    //     config: Option<RpcGetVoteAccountsConfig>,
    // ) -> RpcResult<RpcVoteAccountStatus>;
    //
    // // --------------------------
    // // Accounts
    // // --------------------------
    //
    // #[method(name = "getAccountInfo")]
    // async fn get_account_info(
    //     &self,
    //     pubkey_str: String,
    //     config: Option<RpcAccountInfoConfig>,
    // ) -> RpcResult<RpcResponse<Option<UiAccount>>>;
    //
    // #[method(name = "getMultipleAccounts")]
    // async fn get_multiple_accounts(
    //     &self,
    //     pubkey_strs: Vec<String>,
    //     config: Option<RpcAccountInfoConfig>,
    // ) -> RpcResult<RpcResponse<Vec<Option<UiAccount>>>>;
    //
    // #[method(name = "getProgramAccounts")]
    // async fn get_program_accounts(
    //     &self,
    //     program_id_str: String,
    //     config: Option<RpcProgramAccountsConfig>,
    // ) -> RpcResult<OptionalContext<Vec<RpcKeyedAccount>>>;
    //
    // #[method(name = "getBalance")]
    // async fn get_balance(
    //     &self,
    //     pubkey_str: String,
    //     config: Option<RpcContextConfig>,
    // ) -> RpcResult<RpcResponse<u64>>;
}
