use hdk::prelude::*;

mod entry;
mod handlers;
mod checks;

pub use entry::*;
pub use handlers::*;
pub use checks::*;

#[hdk_extern]
pub fn init(_: ()) -> ExternResult<InitCallbackResult> {

    Ok(InitCallbackResult::Pass)
}

#[hdk_extern]
fn recv_remote_signal(signal: SerializedBytes) -> ExternResult<()> {
    emit_signal(&signal)?;
    Ok(())
}
