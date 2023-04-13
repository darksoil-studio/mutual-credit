use hdi::prelude::*;

#[hdk_entry_helper]
#[derive(Clone)]
pub struct TransactionRequest {
    pub spender_pub_key: AgentPubKey,
    pub recipient_pub_key: AgentPubKey,
    pub amount: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum TransactionRequestType {
    Send,
    Receive,
}

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
