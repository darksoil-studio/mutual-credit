pub mod signals;
pub mod types;
pub mod utils;

pub use entry::*;
pub use types::*;
pub use utils::*;

use hdi::prelude::*;
use transaction_types::TransactionRequest;


#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
#[hdk_entry_defs]
#[unit_enum(UnitEntryTypes)]
pub enum EntryTypes {
    TransactionRequest(TransactionRequest),
}

#[derive(Serialize, Deserialize)]
#[hdk_link_types]
pub enum LinkTypes {
    AgentToTransactionRequest,
    CounterpartyToTransactionRequest,
    RequestToTransactionAction,
}