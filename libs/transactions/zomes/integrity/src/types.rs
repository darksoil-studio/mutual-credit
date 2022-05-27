use hdk::prelude::*;

use crate::Transaction;

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateTransactionInput {
    pub transaction: Transaction,
    pub preflight_responses: Vec<PreflightResponse>,
}
