use transaction_requests_integrity::call_transactions;
use transaction_types::{Transaction,TransactionRequest};
use hdk::prelude::holo_hash::*;
use hdk::prelude::*;

pub fn build_transaction(transaction_request_record: Record) -> ExternResult<Transaction> {
    let transaction_request: TransactionRequest = transaction_request_record
        .entry()
        .to_app_option()
        .map_err(|e| wasm_error!(WasmErrorInner::Guest(format!("Failed to convert entry to app option: {}", e))))?
        .ok_or(wasm_error!(WasmErrorInner::Guest(String::from(
            "Malformed transaction_request",
        ))))?;

    let spender = transaction_request.spender_pub_key.clone();
    let recipient = transaction_request.recipient_pub_key.clone();

    let spender_latest_transaction = get_latest_transaction_for_agent(spender.into())?;
    let recipient_latest_transaction = get_latest_transaction_for_agent(recipient.into())?;

    let transaction = Transaction::from_previous_transactions(
        transaction_request.spender_pub_key.into(),
        transaction_request.recipient_pub_key.into(),
        spender_latest_transaction,
        recipient_latest_transaction,
        transaction_request.amount,
        SerializedBytes::try_from(transaction_request_record.action_address())
            .map_err(|e| wasm_error!(format!("Failed to serialize transaction request record: {}", e)))?,
    )?;
    Ok(transaction)
}

fn get_latest_transaction_for_agent(
    agent_pub_key: AgentPubKey,
) -> ExternResult<Option<(ActionHash, Transaction)>> {
    call_transactions("get_latest_transaction_for_agent".into(), agent_pub_key)
}
