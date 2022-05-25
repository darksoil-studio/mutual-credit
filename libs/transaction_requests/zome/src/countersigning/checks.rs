use hc_lib_transactions::{check_transaction_is_the_latest, Transaction};
use hdk::prelude::*;


pub fn check_preflight_response(preflight_response: PreflightResponse) -> ExternResult<()> {
    let preflight_request = preflight_response.request();
    let bytes = SerializedBytes::from(UnsafeBytes::from(
        preflight_request.preflight_bytes().0.clone(),
    ));

    let transaction = Transaction::try_from(bytes)?;

    let author_index = preflight_response.agent_state().agent_index().clone() as usize;
    let author = preflight_request
        .signing_agents()
        .get(author_index)
        .ok_or(WasmError::Guest(String::from(
            "Malformed preflight response",
        )))?
        .0
        .clone();

    let party = transaction.get_party(&author)?;

    let chain_top = preflight_response.agent_state().chain_top();

    check_transaction_is_the_latest(
        author,
        party.previous_transaction_hash.map(|h| HeaderHash::from(h)),
        chain_top.clone(),
    )?;

    Ok(())
}
