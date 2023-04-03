use hdk::prelude::holo_hash::*;
use hdk::prelude::*;

#[hdk_entry_helper]
#[derive(Clone)]
#[serde(rename_all = "camelCase")]
pub struct TransactionRequest {
    pub spender_pub_key: AgentPubKey,
    pub recipient_pub_key: AgentPubKey,
    pub amount: f64,
}

impl TransactionRequest {
    pub fn get_counterparty(&self) -> ExternResult<AgentPubKey> {
        let my_pub_key = agent_info()?.agent_initial_pubkey;

        if my_pub_key.eq(&self.spender_pub_key) {
            Ok(self.recipient_pub_key.clone())
        } else if my_pub_key.eq(&self.recipient_pub_key) {
            Ok(self.spender_pub_key.clone())
        } else {
            Err(wasm_error!(WasmErrorInner::Guest(String::from(
                "I don't participate in this TransactionRequest",
            ))))
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum TransactionRequestType {
    Send,
    Receive,
}
