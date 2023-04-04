use transactions_integrity::UnitEntryTypes;
use transaction_types::Transaction;
use hdk::prelude::holo_hash::*;
use hdk::prelude::*;

pub fn create_transaction(
    transaction: Transaction,
    preflight_responses: Vec<PreflightResponse>,
) -> ExternResult<ActionHash> {
    let entry = Entry::CounterSign(
        Box::new(
            CounterSigningSessionData::try_from_responses(preflight_responses, vec![]).map_err(
                |countersigning_error| wasm_error!(WasmErrorInner::Guest(countersigning_error.to_string())
            ))?,
        ),
        transaction.clone().try_into()?,
    );

    let transaction_header_hash = HDK.with(|h| {
        h.borrow().create(CreateInput::new(
            ScopedEntryDefIndex::try_from(UnitEntryTypes::Transaction)?,
            EntryVisibility::Public,
            entry,
            // Countersigned entries MUST have strict ordering.
            ChainTopOrdering::Strict,
        ))
    })?;

    Ok(transaction_header_hash.into())
}
