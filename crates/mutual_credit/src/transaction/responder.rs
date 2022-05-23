use hdk::prelude::holo_hash::*;
use hdk::prelude::*;

use super::entry::Transaction;

#[hdk_extern]
pub fn request_lock_chain(preflight_request: PreflightRequest) -> ExternResult<PreflightResponse> {
  let bytes = SerializedBytes::from(UnsafeBytes::from(preflight_request.preflight_bytes().0));

  let transaction = Transaction::try_from(bytes)?;

  

  match accept_countersigning_preflight_request(preflight_request.clone())? {
        PreflightRequestAcceptance::Accepted(response) => Ok(response),
        _ => Err(WasmError::Guest(
            "There was an error accepting the preflight request for the transaction".into(),
        )),
    }
}
