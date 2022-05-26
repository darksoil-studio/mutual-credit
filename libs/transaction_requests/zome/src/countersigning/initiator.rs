use hc_lib_transactions::{
    build_preflight_request, create_transaction, get_latest_transaction_for, Transaction,
};
use hdk::prelude::holo_hash::*;
use hdk::prelude::*;

use crate::TransactionRequest;

use super::checks::check_preflight_response;
use super::responder::PreTransactionCheckInput;

pub fn attempt_create_transaction(
    transaction_request_element: Element,
    counterparty_chain_top: HeaderHashB64,
) -> ExternResult<(HeaderHashB64, Transaction)> {
    let transaction_request: TransactionRequest = transaction_request_element
        .entry()
        .to_app_option()?
        .ok_or(WasmError::Guest(String::from(
            "Malformed transaction request",
        )))?;
    let counterparty = transaction_request.get_counterparty()?;

    let response = call_remote(
        counterparty.clone().into(),
        zome_info()?.name,
        "pre_transaction_check".into(),
        None,
        PreTransactionCheckInput {
            transaction_request_hash: transaction_request_element.header_address().clone().into(),
            chain_top: counterparty_chain_top.clone(),
        },
    )?;

    match response.clone() {
        ZomeCallResponse::Ok(_) => Ok(()),
        _ => Err(WasmError::Guest(format!(
            "Error with fn pre_transaction_check: {:?}",
            response
        ))),
    }?;

    let transaction = build_transaction(transaction_request_element)?;
    let my_pub_key = agent_info()?.agent_initial_pubkey;

    let countersigning_agents = vec![
        (AgentPubKey::from(my_pub_key.clone()), vec![]),
        (AgentPubKey::from(counterparty.clone()), vec![]),
    ];
    let preflight_request = build_preflight_request(transaction.clone(), countersigning_agents)?;

    let my_response = match accept_countersigning_preflight_request(preflight_request)? {
        PreflightRequestAcceptance::Accepted(response) => Ok(response),
        _ => Err(WasmError::Guest(String::from(
            "Couldn't lock our own chain",
        ))),
    }?;

    let response = call_remote(
        counterparty.clone().into(),
        zome_info()?.name,
        "request_create_transaction".into(),
        None,
        my_response.clone(),
    )?;

    let result = match response {
        ZomeCallResponse::Ok(result) => Ok(result),
        _ => Err(WasmError::Guest(format!(
            "Error with fn request_create_transaction {:?}",
            response
        ))),
    }?;

    let counterparty_response: PreflightResponse = result.decode()?;

    let chain_top = counterparty_response.agent_state().chain_top();

    if !HeaderHash::from(counterparty_chain_top).eq(chain_top) {
        return Err(WasmError::Guest(String::from(
            "Counterparty chain moved in the process of finalizing the transaction",
        )));
    }

    check_preflight_response(counterparty_response.clone())?;

    let header_hash = create_transaction(
        transaction.clone(),
        vec![my_response, counterparty_response],
    )?;

    Ok((header_hash.into(), transaction))
}

fn build_transaction(transaction_request_element: Element) -> ExternResult<Transaction> {
    let transaction_request: TransactionRequest = transaction_request_element
        .entry()
        .to_app_option()?
        .ok_or(WasmError::Guest(String::from(
            "Malformed transaction_request",
        )))?;

    let spender = transaction_request.spender_pub_key.clone();
    let recipient = transaction_request.recipient_pub_key.clone();

    let spender_latest_transaction = get_latest_transaction_for(spender.into())?;
    let recipient_latest_transaction = get_latest_transaction_for(recipient.into())?;

    let transaction = Transaction::from_previous_transactions(
        transaction_request.spender_pub_key.into(),
        transaction_request.recipient_pub_key.into(),
        spender_latest_transaction,
        recipient_latest_transaction,
        transaction_request.amount,
        SerializedBytes::try_from(transaction_request_element.header_address())?,
    )?;
    Ok(transaction)
}
