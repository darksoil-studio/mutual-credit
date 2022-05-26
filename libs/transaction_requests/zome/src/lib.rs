use hdk::prelude::*;

pub mod countersigning;
mod entry;
mod handlers;
pub mod signals;

pub use entry::*;
pub use handlers::*;

pub fn init() -> ExternResult<()> {
    let mut functions = GrantedFunctions::new();
    functions.insert((zome_info()?.name, "recv_remote_signal".into()));

    let grant = ZomeCallCapGrant {
        access: CapAccess::Unrestricted,
        functions,
        tag: "".into(),
    };
    create_cap_grant(grant)?;

    let mut functions = GrantedFunctions::new();
    functions.insert((zome_info()?.name, "pre_transaction_check".into()));

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

    Ok(())
}
