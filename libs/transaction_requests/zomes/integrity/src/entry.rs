use hdk::prelude::holo_hash::*;
use hdk::prelude::*;

#[hdk_entry(id = "transaction_request")]
#[derive(Clone)]
#[serde(rename_all = "camelCase")]
pub struct TransactionRequest {
    pub spender_pub_key: AgentPubKeyB64,
    pub recipient_pub_key: AgentPubKeyB64,
    pub amount: f64,
}

impl TransactionRequest {
    pub fn get_counterparty(&self) -> ExternResult<AgentPubKeyB64> {
        let my_pub_key: AgentPubKeyB64 = agent_info()?.agent_initial_pubkey.into();

        if my_pub_key.eq(&self.spender_pub_key) {
            Ok(self.recipient_pub_key.clone())
        } else if my_pub_key.eq(&self.recipient_pub_key) {
            Ok(self.spender_pub_key.clone())
        } else {
            Err(WasmError::Guest(String::from(
                "I don't participate in this offer",
            )))
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum TransactionRequestType {
    Send,
    Receive,
}
