use std::collections::BTreeMap;

use hc_zome_transactions_integrity::Transaction;
use hdk::prelude::holo_hash::*;
use hdk::prelude::*;

use crate::transaction_entry_type;

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

            let transaction = Transaction::try_from_entry(entry.clone())?;

            let hash_b64 = HeaderHashB64::from(element.header_address().clone());

            Ok((hash_b64, transaction))
        })
        .collect::<ExternResult<BTreeMap<HeaderHashB64, Transaction>>>()?;

    Ok(transactions)
}

pub fn get_transactions_activity(agent_pub_key: AgentPubKeyB64) -> ExternResult<AgentActivity> {
    let filter = ChainQueryFilter::new().entry_type(transaction_entry_type()?);

    let activity = get_agent_activity(agent_pub_key.into(), filter, ActivityRequest::Full)?;

    Ok(activity)
}
