use hdk::prelude::holo_hash::*;
use hdk::prelude::*;

use hc_zome_transactions_integrity::Transaction;

#[derive(Serialize, Deserialize, Debug)]
pub enum SignalType {
    NewTransactionCreated {
        transaction_hash: HeaderHashB64,
        transaction: Transaction,
    },
}
