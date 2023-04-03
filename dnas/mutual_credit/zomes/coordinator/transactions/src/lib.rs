use hc_zome_transactions_integrity::Transaction;
use hdk::prelude::*;

mod handlers;
mod countersigning;
mod signals;
mod utils;

pub use handlers::*;
pub use utils::*;

entry_defs![Transaction::entry_def()];

#[hdk_extern]
pub fn init(_: ()) -> ExternResult<InitCallbackResult> {
    let mut functions = GrantedFunctions::new();
    functions.insert((zome_info()?.name, "recv_remote_signal".into()));

    let grant = ZomeCallCapGrant {
        access: CapAccess::Unrestricted,
        functions,
        tag: "".into(),
    };
    create_cap_grant(grant)?;

    let mut functions = GrantedFunctions::new();
    functions.insert((zome_info()?.name, "transaction_preflight".into()));

    let grant = ZomeCallCapGrant {
        access: CapAccess::Unrestricted,
        functions,
        tag: "".into(),
    };
    create_cap_grant(grant)?;

    let mut functions = GrantedFunctions::new();
    functions.insert((zome_info()?.name, "request_create_transaction".into()));

    let grant = ZomeCallCapGrant {
        access: CapAccess::Unrestricted,
        functions,
        tag: "".into(),
    };
    create_cap_grant(grant)?;

    Ok(InitCallbackResult::Pass)
}
