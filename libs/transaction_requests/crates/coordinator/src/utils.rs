use hc_zome_mutual_credit_transaction_requests_integrity::TransactionRequest;
use hc_zome_mutual_credit_transactions_types::Transaction;
use hdk::prelude::*;
use serde::de::DeserializeOwned;

pub fn call_transactions<I, R>(fn_name: String, payload: I) -> ExternResult<R>
where
    I: serde::Serialize + std::fmt::Debug,
    R: serde::Serialize + std::fmt::Debug + DeserializeOwned,
{
    let response = call(
        CallTargetCell::Local,
        ZomeName::from("transactions"),
        fn_name.into(),
        None,
        payload,
    )?;

    let result = match response {
        ZomeCallResponse::Ok(result) => Ok(result),
        _ => Err(wasm_error!(WasmErrorInner::Guest(format!(
            "Error creating the transaction: {:?}",
            response
        )))),
    }?;

    let transaction_hash: R = result.decode().map_err(|e| {
        wasm_error!(WasmErrorInner::Guest(format!(
            "Failed to decode transaction hash: {}",
            e
        )))
    })?;

    Ok(transaction_hash)
}

pub fn get_counterparty(transaction_request: &TransactionRequest) -> ExternResult<AgentPubKey> {
    let my_pub_key = agent_info()?.agent_initial_pubkey;

    if my_pub_key.eq(&transaction_request.spender_pub_key) {
        Ok(transaction_request.recipient_pub_key.clone())
    } else if my_pub_key.eq(&transaction_request.recipient_pub_key) {
        Ok(transaction_request.spender_pub_key.clone())
    } else {
        Err(wasm_error!(WasmErrorInner::Guest(String::from(
            "I don't participate in this TransactionRequest",
        ))))
    }
}

pub fn build_transaction(transaction_request_record: Record) -> ExternResult<Transaction> {
    let transaction_request: TransactionRequest = transaction_request_record
        .entry()
        .to_app_option()
        .map_err(|e| {
            wasm_error!(WasmErrorInner::Guest(format!(
                "Failed to convert entry to app option: {}",
                e
            )))
        })?
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
        SerializedBytes::try_from(transaction_request_record.action_address()).map_err(|e| {
            wasm_error!(format!(
                "Failed to serialize transaction request record: {}",
                e
            ))
        })?,
    )?;
    Ok(transaction)
}

fn get_latest_transaction_for_agent(
    agent_pub_key: AgentPubKey,
) -> ExternResult<Option<(ActionHash, Transaction)>> {
    let maybe_record: Option<Record> =
        call_transactions("get_latest_transaction_for_agent".into(), agent_pub_key)?;

    match maybe_record {
        Some(record) => {
            let entry = record
                .entry()
                .as_option()
                .ok_or(wasm_error!(WasmErrorInner::Guest(
                    "Transaction record must have an entry present".to_string()
                )))?
                .clone();
            let transaction = Transaction::try_from(entry)?;
            Ok(Some((record.action_address().clone(), transaction)))
        }
        None => Ok(None),
    }
}
