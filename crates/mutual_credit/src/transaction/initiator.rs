use hdk::prelude::holo_hash::*;
use hdk::prelude::*;

use crate::intent::Intent;

use super::{
    common::create_transaction, get_latest_transaction_for, responder::IsIntentStillValid,
    validation::validate_preflight_response, Transaction,
};

pub fn attempt_create_transaction(
    intent_element: Element,
    counterparty_chain_top: HeaderHashB64,
) -> ExternResult<(HeaderHashB64, Transaction)> {
    let counterparty = intent_element.header().author().clone();

    let response = call_remote(
        counterparty.clone(),
        zome_info()?.name,
        "is_intent_is_still_valid".into(),
        None,
        IsIntentStillValid {
            intent_hash: intent_element.header_address().clone().into(),
            chain_top: counterparty_chain_top.clone(),
        },
    )?;

    let _is_valid_result: () = match response {
        ZomeCallResponse::Ok(result) => result.decode()?,
        _ => Err(WasmError::Guest(String::from(
            "Error with fn is_intent_still_valid",
        ))),
    }?;

    let transaction = build_transaction(intent_element)?;
    let preflight_request = build_preflight_request(transaction.clone())?;

    let my_response = match accept_countersigning_preflight_request(preflight_request)? {
        PreflightRequestAcceptance::Accepted(response) => Ok(response),
        _ => Err(WasmError::Guest(String::from(
            "Couldn't lock our own chain",
        ))),
    }?;

    let response = call_remote(
        counterparty.clone(),
        zome_info()?.name,
        "request_create_transaction".into(),
        None,
        my_response.clone(),
    )?;

    let counterparty_response: PreflightResponse = match response {
        ZomeCallResponse::Ok(result) => result.decode()?,
        _ => Err(WasmError::Guest(String::from(
            "Error with fn request_create_transaction",
        ))),
    }?;

    let chain_top = counterparty_response.agent_state().chain_top();

    if !HeaderHash::from(counterparty_chain_top).eq(chain_top) {
        return Err(WasmError::Guest(String::from(
            "Counterparty chain moved in the process of finalizing the transaction",
        )));
    }

    validate_preflight_response(counterparty_response.clone())?;

    let header_hash = create_transaction(
        transaction.clone(),
        vec![my_response, counterparty_response],
    )?;

    Ok((header_hash.into(), transaction))
}

fn build_transaction(intent_element: Element) -> ExternResult<Transaction> {
    let transaction_intent: Intent =
        intent_element
            .entry()
            .to_app_option()?
            .ok_or(WasmError::Guest(String::from(
                "Malformed transaction_intent",
            )))?;

    let spender = transaction_intent.spender_pub_key.clone();
    let recipient = transaction_intent.recipient_pub_key.clone();

    let spender_latest_transaction = get_latest_transaction_for(spender.into())?;
    let recipient_latest_transaction = get_latest_transaction_for(recipient.into())?;

    let transaction = Transaction::from_previous_transactions(
        transaction_intent.spender_pub_key.into(),
        transaction_intent.recipient_pub_key.into(),
        spender_latest_transaction,
        recipient_latest_transaction,
        transaction_intent.amount,
        intent_element.header_address().clone().into(),
    )?;
    Ok(transaction)
}

fn build_preflight_request(transaction: Transaction) -> ExternResult<PreflightRequest> {
    let transaction_hash = hash_entry(&transaction)?;

    let times = session_times_from_millis(5_000)?;

    let header_base = HeaderBase::Create(CreateBase::new(Transaction::entry_type()?));

    let countersigning_agents = vec![
        (
            AgentPubKey::from(transaction.spender.agent_pub_key.clone()),
            vec![],
        ),
        (
            AgentPubKey::from(transaction.recipient.agent_pub_key.clone()),
            vec![],
        ),
    ];

    let bytes = SerializedBytes::try_from(transaction.clone())?;

    let preflight_bytes = PreflightBytes(bytes.bytes().to_vec());

    let preflight_request = PreflightRequest::try_new(
        transaction_hash,
        countersigning_agents,
        Some(0),
        times,
        header_base,
        preflight_bytes,
    )
    .map_err(|err| WasmError::Guest(format!("Could not create preflight request: {:?}", err)))?;

    Ok(preflight_request)
}
