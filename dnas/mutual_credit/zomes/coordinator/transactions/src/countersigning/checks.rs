use transactions_integrity::Transaction;
use hdk::prelude::holo_hash::*;
use hdk::prelude::*;

use crate::get_transactions_activity;

pub fn _check_preflight_response(preflight_response: PreflightResponse) -> ExternResult<()> {
    let preflight_request = preflight_response.request();
    let bytes = SerializedBytes::from(UnsafeBytes::from(
        preflight_request.preflight_bytes.0.clone(),
    ));

    let transaction = Transaction::try_from(bytes)
        .map_err(|e| wasm_error!(WasmErrorInner::Guest(format!("Failed to deserialize Transaction: {}", e))))?;

    let author = _get_author(&preflight_response)?;

    let party = transaction.get_party(&author)?;

    let chain_top = preflight_response.agent_state().chain_top();

    _check_transaction_is_the_latest(
        author,
        party.previous_transaction_hash.map(|h| ActionHash::from(h)),
        chain_top.clone(),
    )?;

    Ok(())
}

pub fn _check_transaction_is_the_latest(
    agent_pub_key: AgentPubKey,
    transaction_hash: Option<ActionHash>,
    highest_observed: ActionHash,
) -> ExternResult<()> {
    let activity: AgentActivity = get_transactions_activity(agent_pub_key.into())?;

    let actual_highest = activity
        .clone()
        .highest_observed
        .ok_or(wasm_error!(WasmErrorInner::Guest(String::from("Highest observed is None"))))?;

    if actual_highest.hash.len() != 1 {
        return Err(wasm_error!(WasmErrorInner::Guest(String::from(
            "More than one header is in the highest observed",
        ))));
    }

    if !actual_highest.hash[0].eq(&highest_observed) {
        return Err(wasm_error!(WasmErrorInner::Guest(String::from("Bad highest observed"))));
    }

    let valid = match (activity.valid_activity.last(), transaction_hash) {
        (None, None) => true,
        (Some((_, latest_observed_transaction)), Some(transaction_to_validate)) => {
            transaction_to_validate.eq(latest_observed_transaction)
        }
        _ => false,
    };

    if !valid {
        return Err(wasm_error!(WasmErrorInner::Guest(String::from(
            "Transaction is not the latest",
        ))));
    }

    Ok(())
}

pub fn _get_author(preflight_response: &PreflightResponse) -> ExternResult<AgentPubKey> {
    let author_index = preflight_response.agent_state().agent_index().clone() as usize;
    let author = preflight_response
        .request()
        .signing_agents
        .get(author_index)
        .ok_or(wasm_error!(WasmErrorInner::Guest(String::from(
            "Malformed preflight response",
        ))))?
        .0
        .clone();

    Ok(author)
}
