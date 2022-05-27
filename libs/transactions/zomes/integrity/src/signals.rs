use hc_zome_transactions::Transaction;
use hdk::prelude::holo_hash::*;
use hdk::prelude::*;

use crate::TransactionRequest;

#[derive(Serialize, Deserialize, Debug)]
pub enum SignalType {
    NewTransactionCreated {
        transaction_hash: HeaderHashB64,
        transaction: Transaction,
    },
}
