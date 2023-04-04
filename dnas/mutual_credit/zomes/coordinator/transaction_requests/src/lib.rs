use hdk::prelude::*;

mod handlers;
mod utils;

pub use handlers::*;
use transaction_types::TransactionRequest;


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
fn recv_remote_signal(signal: SerializedBytes) -> ExternResult<()> {
    emit_signal(&signal)?;
    Ok(())
}
