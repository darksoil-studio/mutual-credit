use std::collections::BTreeMap;

use hdk::prelude::holo_hash::*;
use hdk::prelude::*;

use crate::Transaction;

#[hdk_extern]
pub fn query_my_transactions(_: ()) -> ExternResult<BTreeMap<HeaderHashB64, Transaction>> {
    let filter = ChainQueryFilter::new()
        .entry_type(Transaction::entry_type()?)
        .include_entries(true);
    let elements = query(filter)?;

    elements_to_transactions(elements)
}

#[hdk_extern]
pub fn get_transactions_for_agent(
    agent_pub_key: AgentPubKeyB64,
) -> ExternResult<BTreeMap<HeaderHashB64, Transaction>> {
    let activity = get_transactions_activity(agent_pub_key.into())?;

    let get_inputs = activity
        .valid_activity
        .into_iter()
        .map(|(_, header_hash)| GetInput::new(header_hash.into(), GetOptions::default()))
        .collect();

    let maybe_elements = HDK.with(|hdk| hdk.borrow().get(get_inputs))?;

    let elements = maybe_elements.into_iter().filter_map(|el| el).collect();

    let transactions = elements_to_transactions(elements)?;

    Ok(transactions)
}

pub fn elements_to_transactions(
    elements: Vec<Element>,
) -> ExternResult<BTreeMap<HeaderHashB64, Transaction>> {
    let transactions = elements
        .into_iter()
        .map(|element| {
            let entry = element
                .entry()
                .as_option()
                .ok_or(WasmError::Guest(String::from("Malformed transaction")))?;

            let transaction = entry_to_transaction(entry.clone())?;

            let hash_b64 = HeaderHashB64::from(element.header_address().clone());

            Ok((hash_b64, transaction))
        })
        .collect::<ExternResult<BTreeMap<HeaderHashB64, Transaction>>>()?;

    Ok(transactions)
}

pub fn entry_to_transaction(entry: Entry) -> ExternResult<Transaction> {
    match entry {
        Entry::CounterSign(_session_data, entry_bytes) => {
            let transaction = Transaction::try_from(entry_bytes.into_sb())?;
            Ok(transaction)
        }
        _ => Err(WasmError::Guest(String::from("Malformed entry"))),
    }
}

pub fn get_latest_transaction_for(
    agent_pub_key: AgentPubKey,
) -> ExternResult<Option<(HeaderHashB64, Transaction)>> {
    let activity = get_transactions_activity(agent_pub_key)?;

    match activity.valid_activity.last() {
        None => Ok(None),
        Some((_seq, hash)) => {
            let element = get(hash.clone(), GetOptions::default())?.ok_or(WasmError::Guest(
                String::from("Couldn't get latest transaction"),
            ))?;

            let entry = element
                .entry()
                .as_option()
                .ok_or(WasmError::Guest(String::from("Malformed transaction")))?;

            let transaction = entry_to_transaction(entry.clone())?;

            let hash_b64 = HeaderHashB64::from(hash.clone());

            Ok(Some((hash_b64, transaction)))
        }
    }
}

pub fn get_transactions_activity(agent_pub_key: AgentPubKey) -> ExternResult<AgentActivity> {
    let filter = ChainQueryFilter::new().entry_type(Transaction::entry_type()?);

    let activity = get_agent_activity(agent_pub_key, filter, ActivityRequest::Full)?;

    Ok(activity)
}
