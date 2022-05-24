use hdk::prelude::*;
use hdk::prelude::holo_hash::*;

use crate::intent::Intent;
use crate::transaction::entry::Transaction;

#[derive(Serialize, Deserialize, Debug)]
pub enum SignalType {
    IntentReceived{
        intent_hash: HeaderHashB64,
        intent: Intent
    },
    IntentAccepted{
        intent_hash: HeaderHashB64,
        transaction: (HeaderHashB64, Transaction)
    },
    IntentCancelled{
        intent_hash: HeaderHashB64,
    },
    IntentRejected{
        intent_hash: HeaderHashB64,
    },
}
