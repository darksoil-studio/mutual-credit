mod entry;
mod types;

pub use entry::*;
pub use types::*;

use hdi::prelude::*;

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
#[hdk_entry_defs]
#[unit_enum(UnitEntryTypes)]
pub enum EntryTypes {
    Transaction(Transaction),
}