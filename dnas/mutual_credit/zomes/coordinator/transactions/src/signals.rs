use hdk::prelude::*;

use transactions_integrity::Transaction;

#[derive(Serialize, Deserialize, Debug)]
pub enum SignalType {
    NewTransactionCreated {
        transaction_hash: ActionHash,
        transaction: Transaction,
    },
}
