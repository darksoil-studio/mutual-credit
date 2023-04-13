use std::collections::BTreeMap;

use transactions_integrity::UnitEntryTypes;
use transaction_types::Transaction;
use hdk::prelude::holo_hash::*;
use hdk::prelude::*;

pub fn records_to_transactions(
    records: Vec<Record>,
) -> ExternResult<BTreeMap<ActionHash, Transaction>> {
    let transactions = records
        .into_iter()
        .map(|element| {
            let entry = element
                .entry()
                .as_option()
                .ok_or(wasm_error!(WasmErrorInner::Guest(String::from("Malformed transaction"))))?;

            let transaction = Transaction::try_from_entry(entry.clone())?;

            let hash_b64 = ActionHash::from(element.action_address().clone());

            Ok((hash_b64, transaction))
        })
        .collect::<ExternResult<BTreeMap<ActionHash, Transaction>>>()?;

    Ok(transactions)
}

pub fn get_transactions_activity(agent_pub_key: AgentPubKeyB64) -> ExternResult<AgentActivity> {
    let transaction_entry_type: EntryType = UnitEntryTypes::Transaction.try_into()?;

    let filter = ChainQueryFilter::new().entry_type(transaction_entry_type);

    let activity = get_agent_activity(agent_pub_key.into(), filter, ActivityRequest::Full)?;

    Ok(activity)
}
