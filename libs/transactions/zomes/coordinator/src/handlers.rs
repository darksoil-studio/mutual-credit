use std::collections::BTreeMap;

use hc_zome_transactions_integrity::{entry_to_transaction, CreateTransactionInput, Transaction};
use hdk::prelude::holo_hash::*;
use hdk::prelude::*;

#[hdk_extern]
pub fn create_transaction(input: CreateTransactionInput) -> ExternResult<HeaderHashB64> {
    let entry = Entry::CounterSign(
        Box::new(
            CounterSigningSessionData::try_from_responses(input.preflight_responses).map_err(
                |countersigning_error| WasmError::Guest(countersigning_error.to_string()),
            )?,
        ),
        input.transaction.clone().try_into()?,
    );

    let transaction_header_hash = HDK.with(|h| {
        h.borrow().create(CreateInput::new(
            Transaction::entry_def_id(),
            entry,
            // Countersigned entries MUST have strict ordering.
            ChainTopOrdering::Strict,
        ))
    })?;

    Ok(transaction_header_hash.into())
}

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

#[hdk_extern]
pub fn get_transactions_activity(agent_pub_key: AgentPubKeyB64) -> ExternResult<AgentActivity> {
    hc_zome_transactions_integrity::get_transactions_activity(agent_pub_key.into())
}

#[hdk_extern]
pub fn get_latest_transaction_for_agent(
    agent_pub_key: AgentPubKeyB64,
) -> ExternResult<Option<(HeaderHashB64, Transaction)>> {
    hc_zome_transactions_integrity::get_latest_transaction_for_agent(agent_pub_key.into())
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
