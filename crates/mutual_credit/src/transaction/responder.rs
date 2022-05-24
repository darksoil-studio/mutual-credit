use hdk::prelude::holo_hash::*;
use hdk::prelude::*;

use super::common::create_transaction;
use super::entry::Transaction;
use super::validation::validate_preflight_response;

#[derive(Debug, Serialize, Deserialize)]
pub struct IsIntentStillValid {
    pub intent_hash: HeaderHashB64,
    pub chain_top: HeaderHashB64,
}
#[hdk_extern]
pub fn is_intent_still_valid(input: IsIntentStillValid) -> ExternResult<()> {

    validate_is_top_of_the_chain(input.chain_top.into())?;
    validate_intent_is_still_valid(input.intent_hash.into())?;

    Ok(())
}

#[hdk_extern]
pub fn request_create_transaction(
    preflight_response: PreflightResponse,
) -> ExternResult<PreflightResponse> {
    let preflight_request = preflight_response.request().clone();
    let bytes = SerializedBytes::from(UnsafeBytes::from(preflight_request.preflight_bytes().0.clone()));

    let transaction = Transaction::try_from(bytes)?;

    validate_intent_is_still_valid(transaction.intent_hash.clone().into())?;
    validate_preflight_response(preflight_response.clone())?;

    let my_response = match accept_countersigning_preflight_request(preflight_request.clone())? {
        PreflightRequestAcceptance::Accepted(response) => Ok(response),
        _ => Err(WasmError::Guest(
            "There was an error accepting the preflight request for the transaction".into(),
        )),
    }?;


    let _header_hash = create_transaction(
        transaction.clone(),
        vec![my_response.clone(), preflight_response],
    )?;

    Ok(my_response)
}

pub fn validate_intent_is_still_valid(_intent_hash: HeaderHash) -> ExternResult<()> {
    Ok(())
}

fn validate_is_top_of_the_chain(chain_top: HeaderHash) -> ExternResult<()> {
    let elements = query(ChainQueryFilter::new())?;

    let last_element = elements
        .last()
        .ok_or(WasmError::Guest(String::from("Chain is empty!")))?;

    if !HeaderHash::from(chain_top).eq(last_element.header_address()) {
        return Err(WasmError::Guest(String::from("Moved chain")));
    }

    Ok(())
}
