use hc_zome_transactions_integrity::{CreateTransactionInput, Transaction};
use hdk::prelude::holo_hash::*;
use hdk::prelude::*;

pub fn create_transaction(
    transaction: Transaction,
    preflight_responses: Vec<PreflightResponse>,
) -> ExternResult<HeaderHashB64> {
    let response = call(
        CallTargetCell::Local,
        "transactions".into(),
        "create_transaction".into(),
        None,
        CreateTransactionInput {
            transaction,
            preflight_responses,
        },
    )?;

    let result = match response {
        ZomeCallResponse::Ok(result) => Ok(result),
        _ => Err(WasmError::Guest(format!(
            "Error creating the transaction: {:?}",
            response
        ))),
    }?;

    let transaction_hash: HeaderHashB64 = result.decode()?;

    Ok(transaction_hash)
}
