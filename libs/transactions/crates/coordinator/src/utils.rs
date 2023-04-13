use hc_zome_mutual_credit_transactions_integrity::UnitEntryTypes;
use hdk::prelude::*;

pub fn get_transactions_activity(agent_pub_key: AgentPubKey) -> ExternResult<AgentActivity> {
    let transaction_entry_type: EntryType = UnitEntryTypes::Transaction.try_into()?;

    let filter = ChainQueryFilter::new().entry_type(transaction_entry_type);

    let activity = get_agent_activity(agent_pub_key, filter, ActivityRequest::Full)?;

    Ok(activity)
}
