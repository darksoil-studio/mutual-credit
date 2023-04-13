use hdk::prelude::*;

mod handlers;
mod countersigning;
mod signals;
mod utils;

pub use handlers::*;
pub use utils::*;


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

    let mut functions = BTreeSet::new();
    functions.insert((zome_info()?.name, "transaction_preflight".into()));

    let grant = ZomeCallCapGrant {
        access: CapAccess::Unrestricted,
        functions: GrantedFunctions::Listed(functions),
        tag: "".into(),
    };
    create_cap_grant(grant)?;

    let mut functions = BTreeSet::new();
    functions.insert((zome_info()?.name, "request_create_transaction".into()));

    let grant = ZomeCallCapGrant {
        access: CapAccess::Unrestricted,
        functions: GrantedFunctions::Listed(functions),
        tag: "".into(),
    };
    create_cap_grant(grant)?;

    Ok(InitCallbackResult::Pass)
}
