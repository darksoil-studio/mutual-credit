use hdk::prelude::{holo_hash::HeaderHashB64, *};

use crate::Transaction;

pub fn check_transaction_is_the_latest(
    agent_pub_key: AgentPubKey,
    transaction_hash: Option<HeaderHash>,
    highest_observed: HeaderHash,
) -> ExternResult<()> {
    let activity = get_transactions_activity(agent_pub_key.clone())?;

    let actual_highest = activity
        .clone()
        .highest_observed
        .ok_or(WasmError::Guest(String::from("Highest observed is None")))?;

    if actual_highest.hash.len() != 1 {
        return Err(WasmError::Guest(String::from(
            "More than one header is in the highest observed",
        )));
    }

    if !actual_highest.hash[0].eq(&highest_observed) {
        return Err(WasmError::Guest(String::from("Bad highest observed")));
    }

    let valid = match (activity.valid_activity.last(), transaction_hash) {
        (None, None) => true,
        (Some((_, latest_observed_transaction)), Some(transaction_to_validate)) => {
            transaction_to_validate.eq(latest_observed_transaction)
        }
        _ => false,
    };

    if !valid {
        return Err(WasmError::Guest(String::from(
            "Transaction is not the latest",
        )));
    }

    Ok(())
}

pub fn get_transactions_activity(agent_pub_key: AgentPubKey) -> ExternResult<AgentActivity> {
    let filter = ChainQueryFilter::new().entry_type(Transaction::entry_type()?);

    let activity = get_agent_activity(agent_pub_key, filter, ActivityRequest::Full)?;

    Ok(activity)
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

pub fn entry_to_transaction(entry: Entry) -> ExternResult<Transaction> {
    match entry {
        Entry::CounterSign(_session_data, entry_bytes) => {
            let transaction = Transaction::try_from(entry_bytes.into_sb())?;
            Ok(transaction)
        }
        _ => Err(WasmError::Guest(String::from("Malformed entry"))),
    }
}
