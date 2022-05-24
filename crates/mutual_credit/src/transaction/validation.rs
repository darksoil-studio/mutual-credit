use hdk::prelude::*;

use super::{entry::Transaction, common::get_transactions_activity};

pub fn validate_preflight_response(preflight_response: PreflightResponse) -> ExternResult<()> {
    let preflight_request = preflight_response.request();
    let bytes = SerializedBytes::from(UnsafeBytes::from(preflight_request.preflight_bytes().0.clone()));

    let transaction = Transaction::try_from(bytes)?;

    let author_index = preflight_response.agent_state().agent_index().clone() as usize;
    let author = preflight_request
        .signing_agents()
        .get(author_index)
        .ok_or(WasmError::Guest(String::from(
            "Malformed preflight response",
        )))?
        .0.clone();

    let party = transaction.get_party(&author)?;

    let chain_top = preflight_response.agent_state().chain_top();

    validate_transaction_is_the_latest(
        author,
        party.previous_transaction_hash.map(|h| HeaderHash::from(h)),
        chain_top.clone(),
    )?;

    Ok(())
}

fn validate_transaction_is_the_latest(
    agent_pub_key: AgentPubKey,
    transaction_hash: Option<HeaderHash>,
    highest_observed: HeaderHash,
) -> ExternResult<()> {
    let activity = get_transactions_activity(agent_pub_key)?;

    let actual_highest = activity
        .highest_observed
        .ok_or(WasmError::Guest(String::from("Highest observed is None")))?;

    if actual_highest.hash.len() != 1 {
        return Err(WasmError::Guest(String::from(
            "More than one header is in the highest observed",
        )));
    }

    if !actual_highest.hash[0].eq(&highest_observed) {
        return Err(WasmError::Guest(String::from(
            "More than one header is in the highest observed",
        )));
    }

    let valid = match (activity.valid_activity.last(), transaction_hash) {
        (None, None) => true,
        (Some((_, latest_observed_transaction)), Some(transaction_to_validate)) => {
            transaction_to_validate.eq(latest_observed_transaction)
        }
        _ => false,
    };

    if !valid {
        return Err(WasmError::Guest(String::from(
            "Transaction is not the latest",
        )));
    }

    Ok(())
}
