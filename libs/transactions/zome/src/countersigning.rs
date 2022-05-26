use hdk::prelude::*;

use crate::Transaction;

pub fn build_preflight_request(
    transaction: Transaction,
    countersigning_agents: CounterSigningAgents,
) -> ExternResult<PreflightRequest> {
    let transaction_hash = hash_entry(&transaction)?;

    let times = session_times_from_millis(5_000)?;

    let header_base = HeaderBase::Create(CreateBase::new(Transaction::entry_type()?));

    let bytes = SerializedBytes::try_from(transaction.clone())?;

    let preflight_bytes = PreflightBytes(bytes.bytes().to_vec());

    let preflight_request = PreflightRequest::try_new(
        transaction_hash,
        countersigning_agents,
        Some(0),
        times,
        header_base,
        preflight_bytes,
    )
    .map_err(|err| WasmError::Guest(format!("Could not create preflight request: {:?}", err)))?;

    Ok(preflight_request)
}

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
