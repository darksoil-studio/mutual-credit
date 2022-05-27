use hc_zome_transactions_integrity::{CreateTransactionInput, Transaction};
use hdk::prelude::holo_hash::*;
use hdk::prelude::*;

use crate::utils::call_transactions;

pub fn create_transaction(
    transaction: Transaction,
    preflight_responses: Vec<PreflightResponse>,
) -> ExternResult<HeaderHashB64> {
    let transaction_hash: HeaderHashB64 = call_transactions(
        String::from("create_transaction"),
        CreateTransactionInput {
            transaction,
            preflight_responses,
        },
    )?;

    Ok(transaction_hash)
}
