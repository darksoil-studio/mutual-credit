use hc_zome_mutual_credit_transactions_types::*;

pub use entry::*;

use hdi::prelude::*;

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
#[hdk_entry_defs]
#[unit_enum(UnitEntryTypes)]
pub enum EntryTypes {
    Transaction(Transaction),
}
