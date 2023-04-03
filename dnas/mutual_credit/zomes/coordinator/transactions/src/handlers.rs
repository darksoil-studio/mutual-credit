use std::collections::BTreeMap;

use transactions_integrity::{Transaction, UnitEntryTypes};
use hdk::prelude::holo_hash::*;
use hdk::prelude::*;

use crate::{records_to_transactions, get_transactions_activity};

#[hdk_extern]
pub fn query_my_transactions(_: ()) -> ExternResult<BTreeMap<ActionHash, Transaction>> {
    let transaction_entry_type: EntryType = UnitEntryTypes::Transaction.try_into()?;

    let filter = ChainQueryFilter::new()
        .entry_type(transaction_entry_type)
        .include_entries(true);
    let elements = query(filter)?;

    records_to_transactions(elements)
}

#[hdk_extern]
pub fn get_transactions_for_agent(
    agent_pub_key: AgentPubKeyB64,
) -> ExternResult<BTreeMap<ActionHash, Transaction>> {
    let activity = get_transactions_activity(agent_pub_key.into())?;

    let get_inputs = activity
        .valid_activity
        .into_iter()
        .map(|(_, header_hash)| GetInput::new(header_hash.into(), GetOptions::default()))
        .collect();

    let maybe_elements = HDK.with(|hdk| hdk.borrow().get(get_inputs))?;

    let elements = maybe_elements.into_iter().filter_map(|el| el).collect();

    let transactions = records_to_transactions(elements)?;

    Ok(transactions)
}

#[hdk_extern]
pub fn get_latest_transaction_for_agent(
    agent_pub_key: AgentPubKeyB64,
) -> ExternResult<Option<(ActionHash, Transaction)>> {
    let activity = get_transactions_activity(agent_pub_key)?;

    match activity.valid_activity.last() {
        None => Ok(None),
        Some((_seq, hash)) => {
            let element = get(hash.clone(), GetOptions::default())?.ok_or(wasm_error!(WasmErrorInner::Guest(
                String::from("Couldn't get latest transaction"),
            )))?;

            let entry = element
                .entry()
                .as_option()
                .ok_or(wasm_error!(WasmErrorInner::Guest(String::from("Malformed transaction"))))?;

            let transaction = Transaction::try_from_entry(entry.clone())?;

            let hash_b64 = ActionHash::from(hash.clone());

            Ok(Some((hash_b64, transaction)))
        }
    }
}

