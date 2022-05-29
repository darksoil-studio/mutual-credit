use hdk::prelude::*;
use hdk::prelude::holo_hash::*;

use crate::Transaction;

#[derive(Serialize, Deserialize, Debug)]
pub struct AttemptCreateTransactionInput {
    pub transaction: Transaction,
    pub counterparty_chain_top: HeaderHashB64,
}
