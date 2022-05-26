extern crate hc_lib_transaction_requests;
use hc_lib_transaction_requests::TransactionRequest;
use hc_lib_transactions::Transaction;
use hdk::prelude::*;

entry_defs![Transaction::entry_def(), TransactionRequest::entry_def()];

#[hdk_extern]
pub fn init(_: ()) -> ExternResult<InitCallbackResult> {
    hc_lib_transaction_requests::init()?;

    Ok(InitCallbackResult::Pass)
}

#[hdk_extern]
fn recv_remote_signal(signal: SerializedBytes) -> ExternResult<()> {
    emit_signal(&signal)?;
    Ok(())
}
