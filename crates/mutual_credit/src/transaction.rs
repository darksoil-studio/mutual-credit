use std::collections::BTreeMap;

use hdk::prelude::holo_hash::*;
use hdk::prelude::*;

use self::common::get_transactions_activity;
use self::entry::Transaction;

mod initiator;
mod responder;
mod validation;
mod common;
pub mod entry;


pub fn attempt_create_transaction_for_intent(
    intent_element: Element,
    counterparty_chain_top: HeaderHashB64,
) -> ExternResult<(HeaderHashB64, Transaction)> {
    initiator::attempt_create_transaction(intent_element, counterparty_chain_top)
}

#[hdk_extern]
pub fn get_transactions_for_agent(
    agent_pub_key: AgentPubKeyB64,
) -> ExternResult<BTreeMap<HeaderHashB64, Transaction>> {
    let activity = get_transactions_activity(agent_pub_key.into())?;

    let transactions = activity
        .valid_activity
        .into_iter()
        .map(|(_, header_hash)| {
            let element = get(header_hash.clone(), GetOptions::default())?
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
