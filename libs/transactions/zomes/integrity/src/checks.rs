use hdk::prelude::{holo_hash::HeaderHashB64, *};

use crate::Transaction;

pub fn get_transactions_activity(agent_pub_key: AgentPubKey) -> ExternResult<AgentActivity> {
    let filter = ChainQueryFilter::new().entry_type(Transaction::entry_type()?);

    let activity = get_agent_activity(agent_pub_key, filter, ActivityRequest::Full)?;

    Ok(activity)
}

pub fn get_latest_transaction_for_agent(
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

pub fn entry_to_transaction(entry: Entry) -> ExternResult<Transaction> {
    match entry {
        Entry::CounterSign(_session_data, entry_bytes) => {
            let transaction = Transaction::try_from(entry_bytes.into_sb())?;
            Ok(transaction)
        }
        _ => Err(WasmError::Guest(String::from("Malformed entry"))),
    }
}
