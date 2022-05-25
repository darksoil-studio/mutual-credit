use std::collections::BTreeMap;

use hdk::prelude::holo_hash::*;
use hdk::prelude::*;

use crate::Transaction;

pub fn create_transaction(
    transaction: Transaction,
    responses: Vec<PreflightResponse>,
) -> ExternResult<HeaderHash> {
    let entry = Entry::CounterSign(
        Box::new(
            CounterSigningSessionData::try_from_responses(responses).map_err(
                |countersigning_error| WasmError::Guest(countersigning_error.to_string()),
            )?,
        ),
        transaction.clone().try_into()?,
    );

    let transaction_header_hash = HDK.with(|h| {
        h.borrow().create(CreateInput::new(
            Transaction::entry_def_id(),
            entry,
            // Countersigned entries MUST have strict ordering.
            ChainTopOrdering::Strict,
        ))
    })?;

    Ok(transaction_header_hash)
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

pub fn get_transactions_activity(agent_pub_key: AgentPubKey) -> ExternResult<AgentActivity> {
    let filter = ChainQueryFilter::new().entry_type(Transaction::entry_type()?);

    let activity = get_agent_activity(agent_pub_key, filter, ActivityRequest::Full)?;

    Ok(activity)
}
