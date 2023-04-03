use hdk::prelude::holo_hash::*;
use hdk::prelude::*;

use crate::TransactionRequestType;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CreateTransactionRequestInput {
    pub transaction_request_type: TransactionRequestType,
    pub counterparty_pub_key: AgentPubKey,
    pub amount: f64,
}
