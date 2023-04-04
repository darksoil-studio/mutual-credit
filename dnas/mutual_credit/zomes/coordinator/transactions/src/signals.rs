use hdk::prelude::*;

use transaction_types::Transaction;

#[derive(Serialize, Deserialize, Debug)]
pub enum SignalType {
    NewTransactionCreated {
        transaction_hash: ActionHash,
        transaction: Transaction,
    },
}
