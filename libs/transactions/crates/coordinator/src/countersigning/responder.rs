use hc_zome_mutual_credit_transactions_types::Transaction;
use hdk::prelude::*;

use super::common::create_transaction;

#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionPreflight {
    pub chain_top: ActionHash,
    pub preflight_request: PreflightRequest,
}
#[hdk_extern]
pub fn transaction_preflight(input: TransactionPreflight) -> ExternResult<PreflightResponse> {
    check_is_top_of_the_chain(input.chain_top.into())?;
    // TODO: Add custom checks
    // check_transaction_request_is_still_valid(input.transaction_request_hash.into())?;

    let my_response =
        match accept_countersigning_preflight_request(input.preflight_request.clone())? {
            PreflightRequestAcceptance::Accepted(response) => Ok(response),
            _ => Err(wasm_error!(WasmErrorInner::Guest(
                "There was an error accepting the preflight request for the transaction".into(),
            ))),
        }?;

    Ok(my_response)
}

#[hdk_extern]
pub fn request_create_transaction(
    all_responses: Vec<PreflightResponse>,
) -> ExternResult<ActionHash> {
    let preflight_request = all_responses[0].request().clone();
    let bytes = SerializedBytes::from(UnsafeBytes::from(
        preflight_request.preflight_bytes.0.clone(),
    ));

    let transaction = Transaction::try_from(bytes).map_err(|e| {
        wasm_error!(WasmErrorInner::Guest(format!(
            "Failed to deserialize transaction: {}",
            e
        )))
    })?;

    let header_hash = create_transaction(transaction.clone(), all_responses)?;

    Ok(header_hash)
}

fn check_is_top_of_the_chain(chain_top: ActionHash) -> ExternResult<()> {
    let elements = query(ChainQueryFilter::new())?;

    let last_element = elements
        .last()
        .ok_or(wasm_error!(WasmErrorInner::Guest(String::from(
            "Chain is empty!"
        ))))?;

    if !ActionHash::from(chain_top).eq(last_element.action_address()) {
        return Err(wasm_error!(WasmErrorInner::Guest(String::from(
            "Moved chain"
        ))));
    }

    Ok(())
}
