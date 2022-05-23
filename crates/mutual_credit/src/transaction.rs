use std::collections::BTreeMap;

use hdk::prelude::holo_hash::*;
use hdk::prelude::*;

use self::entry::Transaction;

mod initiator;
mod responder;
pub mod entry;


pub fn attempt_create_transaction_for_intent(
    transaction_intent_header_hash: HeaderHashB64,
) -> ExternResult<(HeaderHashB64, Transaction)> {
    initiator::attempt_create_transaction(transaction_intent_header_hash)

    // Create link from offer to transaction
}

#[hdk_extern]
pub fn get_transactions_for_agent(
    agent_pub_key: AgentPubKeyB64,
) -> ExternResult<BTreeMap<HeaderHashB64, Transaction>> {
    let activity = get_transactions_activity(agent_pub_key)?;

    let transactions = activity
        .valid_activity
        .into_iter()
        .map(|(seq, header_hash)| {
            let element = get(header_hash, GetOptions::default())?
                .ok_or(WasmError::Guest(String::from("Couldn't get transaction")))?;

            let entry = element
                .entry()
                .as_option()
                .ok_or(WasmError::Guest(String::from("Malformed transaction")))?;

            let transaction = entry_to_transaction(entry.clone())?;

            let hash_b64 = HeaderHashB64::from(header_hash.clone());

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
    agent_pub_key: AgentPubKeyB64,
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

fn get_transactions_activity(agent_pub_key: AgentPubKeyB64) -> ExternResult<AgentActivity> {
    let filter = ChainQueryFilter::new().entry_type(Transaction::entry_type()?);

    let activity = get_agent_activity(agent_pub_key.into(), filter, ActivityRequest::Full)?;

    Ok(activity)
}
