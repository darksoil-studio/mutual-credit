use hc_lib_transactions::Transaction;
use hdk::prelude::*;
use hdk::prelude::holo_hash::*;

use crate::TransactionRequest;

#[derive(Serialize, Deserialize, Debug)]
pub enum SignalType {
    TransactionRequestReceived{
        transaction_request_hash: HeaderHashB64,
        transaction_request: TransactionRequest
    },
    TransactionRequestAccepted{
        transaction_request_hash: HeaderHashB64,
        transaction: (HeaderHashB64, Transaction)
    },
    TransactionRequestCancelled{
        transaction_request_hash: HeaderHashB64,
    },
    TransactionRequestRejected{
        transaction_request_hash: HeaderHashB64,
    },
}
