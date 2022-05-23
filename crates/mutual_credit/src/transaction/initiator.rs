use hdk::prelude::holo_hash::*;
use hdk::prelude::*;

use crate::transaction_intent::{TransactionIntent};

use super::{get_latest_transaction_for, Transaction};

pub fn attempt_create_transaction(
    transaction_intent_header_hash: HeaderHashB64,
) -> ExternResult<(HeaderHashB64, Transaction)> {
    let transaction = build_transaction(transaction_intent_header_hash)?;

    let preflight_request = build_preflight_request(transaction)?;
}

fn build_transaction(transaction_intent_header_hash: HeaderHashB64) -> ExternResult<Transaction> {
    let transaction_intent_element = get(
        HeaderHash::from(transaction_intent_header_hash.clone()),
        GetOptions::default(),
    )?
    .ok_or(WasmError::Guest(String::from(
        "Couldn't get transaction_intent",
    )))?;

    let transaction_intent: TransactionIntent = transaction_intent_element
        .entry()
        .to_app_option()?
        .ok_or(WasmError::Guest(String::from(
            "Malformed transaction_intent",
        )))?;

    let spender = transaction_intent.spender_pub_key;
    let recipient = transaction_intent.recipient_pub_key;

    let spender_latest_transaction = get_latest_transaction_for(spender)?;
    let recipient_latest_transaction = get_latest_transaction_for(recipient)?;

    let transaction = Transaction::from_previous_transactions(
        transaction_intent.spender_pub_key,
        transaction_intent.recipient_pub_key,
        spender_latest_transaction,
        recipient_latest_transaction,
        transaction_intent.amount,
        transaction_intent_header_hash,
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
