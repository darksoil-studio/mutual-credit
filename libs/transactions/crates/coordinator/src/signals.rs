use hdk::prelude::*;

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum SignalType {
    NewTransactionCreated { transaction: Record },
}
