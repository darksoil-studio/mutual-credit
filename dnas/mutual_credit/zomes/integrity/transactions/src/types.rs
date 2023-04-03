use hdk::prelude::*;

use crate::Transaction;

#[derive(Serialize, Deserialize, Debug)]
pub struct AttemptCreateTransactionInput {
    pub transaction: Transaction,
    pub counterparty_chain_top: ActionHash,
}
