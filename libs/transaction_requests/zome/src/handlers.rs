use hc_lib_transactions::Transaction;
use hdk::prelude::holo_hash::*;
use hdk::prelude::*;

use crate::{TransactionRequest, TransactionRequestType, countersigning::initiator::attempt_create_transaction};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CreateTransactionRequestInput {
    pub transaction_request_type: TransactionRequestType,
    pub counterparty_pub_key: AgentPubKeyB64,
    pub amount: f64,
}
#[hdk_extern]
pub fn create_transaction_request(
    input: CreateTransactionRequestInput,
) -> ExternResult<(HeaderHashB64, TransactionRequest)> {
    let my_pub_key = agent_info()?.agent_latest_pubkey;

    if AgentPubKey::from(input.counterparty_pub_key.clone()).eq(&my_pub_key) {
        return Err(WasmError::Guest(String::from(
            "An agent cannot create an offer to themselves",
        )));
    }

    let transaction_request = match input.transaction_request_type {
        TransactionRequestType::Send => TransactionRequest {
            spender_pub_key: AgentPubKeyB64::from(my_pub_key.clone()),
            recipient_pub_key: input.counterparty_pub_key.clone(),
            amount: input.amount,
        },
        TransactionRequestType::Receive => TransactionRequest {
            spender_pub_key: input.counterparty_pub_key.clone(),
            recipient_pub_key: AgentPubKeyB64::from(my_pub_key.clone()),
            amount: input.amount,
        },
    };

    let header_hash = create_entry(&transaction_request)?;

    create_link(
        EntryHash::from(my_pub_key),
        header_hash.clone().retype(hash_type::Entry),
        HdkLinkType::Any,
        (),
    )?;
    create_link(
        EntryHash::from(AgentPubKey::from(transaction_request.get_counterparty()?)),
        header_hash.clone().retype(hash_type::Entry),
        HdkLinkType::Any,
        (),
    )?;

    Ok((header_hash.into(), transaction_request))
}

#[hdk_extern]
pub fn accept_transaction_request(
    transaction_request_hash: HeaderHashB64,
) -> ExternResult<(HeaderHashB64, Transaction)> {
    let transaction_request_element = get(
        HeaderHash::from(transaction_request_hash.clone()),
        GetOptions::default(),
    )?
    .ok_or(WasmError::Guest(String::from("Couldn't get intent")))?;

    let counterparty_chain_top =
        get_chain_top(transaction_request_element.header().author().clone())?;

    let result = attempt_create_transaction(
        transaction_request_element.clone(),
        counterparty_chain_top.into(),
    )?;

    create_link(
        transaction_request_element
            .header_address()
            .clone()
            .retype(hash_type::Entry),
        HeaderHash::from(result.0.clone()).retype(hash_type::Entry),
        HdkLinkType::Any,
        (),
    )?;

    Ok(result)
}

fn get_chain_top(agent_pub_key: AgentPubKey) -> ExternResult<HeaderHash> {
    let filter = ChainQueryFilter::new().entry_type(Transaction::entry_type()?);

    let activity = get_agent_activity(agent_pub_key, filter, ActivityRequest::Full)?;

    let highest_observed = activity
        .highest_observed
        .ok_or(WasmError::Guest(String::from(
            "Counterparty highest observed was empty",
        )))?;

    if highest_observed.hash.len() != 1 {
        return Err(WasmError::Guest(String::from(
            "Counterparty highest observed was more than one",
        )));
    }

    Ok(highest_observed.hash[0].clone())
}
