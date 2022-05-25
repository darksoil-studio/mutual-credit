use hdk::prelude::*;

use crate::get_transactions_activity;

pub fn check_transaction_is_the_latest(
  agent_pub_key: AgentPubKey,
  transaction_hash: Option<HeaderHash>,
  highest_observed: HeaderHash,
) -> ExternResult<()> {
  let activity = get_transactions_activity(agent_pub_key)?;

  let actual_highest = activity
      .highest_observed
      .ok_or(WasmError::Guest(String::from("Highest observed is None")))?;

  if actual_highest.hash.len() != 1 {
      return Err(WasmError::Guest(String::from(
          "More than one header is in the highest observed",
      )));
  }

  error!("{:?} {}", actual_highest.hash, highest_observed);
  if !actual_highest.hash[0].eq(&highest_observed) {
      return Err(WasmError::Guest(String::from(
          "Bad highest observed",
      )));
  }

  let valid = match (activity.valid_activity.last(), transaction_hash) {
      (None, None) => true,
      (Some((_, latest_observed_transaction)), Some(transaction_to_validate)) => {
          transaction_to_validate.eq(latest_observed_transaction)
      }
      _ => false,
  };

  if !valid {
      return Err(WasmError::Guest(String::from(
          "Transaction is not the latest",
      )));
  }

  Ok(())
}
