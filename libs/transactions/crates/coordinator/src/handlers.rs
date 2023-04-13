use hc_zome_mutual_credit_transactions_integrity::UnitEntryTypes;
use hdk::prelude::*;

use crate::get_transactions_activity;

#[hdk_extern]
pub fn query_my_transactions(_: ()) -> ExternResult<Vec<Record>> {
    let transaction_entry_type: EntryType = UnitEntryTypes::Transaction.try_into()?;

    let filter = ChainQueryFilter::new()
        .entry_type(transaction_entry_type)
        .include_entries(true);
    let records = query(filter)?;

    Ok(records)
}

#[hdk_extern]
pub fn get_transactions_for_agent(agent_pub_key: AgentPubKey) -> ExternResult<Vec<Record>> {
    let activity = get_transactions_activity(agent_pub_key)?;

    let get_inputs = activity
        .valid_activity
        .into_iter()
        .map(|(_, header_hash)| GetInput::new(header_hash.into(), GetOptions::default()))
        .collect();

    let maybe_elements = HDK.with(|hdk| hdk.borrow().get(get_inputs))?;

    let records = maybe_elements.into_iter().filter_map(|el| el).collect();

    Ok(records)
}

#[hdk_extern]
pub fn get_latest_transaction_for_agent(
    agent_pub_key: AgentPubKey,
) -> ExternResult<Option<Record>> {
    let activity = get_transactions_activity(agent_pub_key)?;

    match activity.valid_activity.last() {
        None => Ok(None),
        Some((_seq, hash)) => {
            let record = get(hash.clone(), GetOptions::default())?.ok_or(wasm_error!(
                WasmErrorInner::Guest(String::from("Couldn't get latest transaction"),)
            ))?;

            Ok(Some(record))
        }
    }
}
