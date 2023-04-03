use transactions_integrity::Transaction;
use hdk::prelude::*;

use crate::TransactionRequest;

#[derive(Serialize, Deserialize, Debug)]
pub enum SignalType {
    TransactionRequestReceived{
        transaction_request_hash: ActionHash,
        transaction_request: TransactionRequest
    },
    TransactionRequestAccepted{
        transaction_request_hash: ActionHash,
        transaction: (ActionHash, Transaction)
    },
    TransactionRequestCancelled{
        transaction_request_hash: ActionHash,
    },
    TransactionRequestRejected{
        transaction_request_hash: ActionHash,
    },
}
