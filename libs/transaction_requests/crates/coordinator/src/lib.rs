use hdk::prelude::*;

mod handlers;
mod utils;

pub use handlers::*;
use hc_zome_mutual_credit_transaction_requests_integrity::TransactionRequest;

#[hdk_extern]
pub fn init(_: ()) -> ExternResult<InitCallbackResult> {
    let mut functions = BTreeSet::new();
    functions.insert((zome_info()?.name, "recv_remote_signal".into()));

    let grant = ZomeCallCapGrant {
        access: CapAccess::Unrestricted,
        functions: GrantedFunctions::Listed(functions),
        tag: "".into(),
    };
    create_cap_grant(grant)?;

    Ok(InitCallbackResult::Pass)
}

#[hdk_extern]
fn recv_remote_signal(signal: Signal) -> ExternResult<()> {
    emit_signal(&signal)?;
    Ok(())
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum Signal {
    TransactionRequestCreated {
        transaction_request: Record,
    },
    TransactionRequestCancelled {
        transaction_request_hash: ActionHash,
    },
    TransactionRequestRejected {
        transaction_request_hash: ActionHash,
    },
    TransactionRequestCleared {
        transaction_request_hash: ActionHash,
    },
    // Transaction completed comes from the transactions zome
}
