use hc_zome_transaction_requests_integrity::{call_transactions, TransactionRequest};
use hc_zome_transactions_integrity::Transaction;
use hdk::prelude::holo_hash::*;
use hdk::prelude::*;

pub fn build_transaction(transaction_request_element: Element) -> ExternResult<Transaction> {
    let transaction_request: TransactionRequest = transaction_request_element
        .entry()
        .to_app_option()?
        .ok_or(WasmError::Guest(String::from(
            "Malformed transaction_request",
        )))?;

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
        SerializedBytes::try_from(transaction_request_element.header_address())?,
    )?;
    Ok(transaction)
}

fn get_latest_transaction_for_agent(
    agent_pub_key: AgentPubKeyB64,
) -> ExternResult<Option<(HeaderHashB64, Transaction)>> {
    call_transactions("get_latest_transaction_for_agent".into(), agent_pub_key)
}
