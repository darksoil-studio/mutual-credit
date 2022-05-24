use hdk::prelude::*;

use super::entry::Transaction;

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

pub fn get_transactions_activity(agent_pub_key: AgentPubKey) -> ExternResult<AgentActivity> {
  let filter = ChainQueryFilter::new().entry_type(Transaction::entry_type()?);

  let activity = get_agent_activity(agent_pub_key, filter, ActivityRequest::Full)?;

  Ok(activity)
}
