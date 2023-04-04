use hdk::prelude::*;

use transaction_types::Transaction;

#[derive(Serialize, Deserialize, Debug)]
pub struct AttemptCreateTransactionInput {
    pub transaction: Transaction,
    pub counterparty_chain_top: ActionHash,
}
