use crate::transaction;
use crate::transaction::entry::Transaction;
use hdk::prelude::holo_hash::*;
use hdk::prelude::*;

#[hdk_entry(id = "intent")]
#[derive(Clone)]
#[serde(rename_all = "camelCase")]
pub struct Intent {
    pub spender_pub_key: AgentPubKeyB64,
    pub recipient_pub_key: AgentPubKeyB64,
    pub amount: f64,
}

impl Intent {
    pub fn get_counterparty(&self) -> ExternResult<AgentPubKeyB64> {
        let my_pub_key: AgentPubKeyB64 = agent_info()?.agent_initial_pubkey.into();

        if my_pub_key.eq(&self.spender_pub_key) {
            Ok(self.recipient_pub_key.clone())
        } else if my_pub_key.eq(&self.recipient_pub_key) {
            Ok(self.spender_pub_key.clone())
        } else {
            Err(WasmError::Guest(String::from(
                "I don't participate in this offer",
            )))
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum IntentType {
    Offer,
    Request,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CreateIntentInput {
    intent_type: IntentType,
    counterparty_pub_key: AgentPubKeyB64,
    amount: f64,
}
#[hdk_extern]
pub fn create_intent(input: CreateIntentInput) -> ExternResult<HeaderHashB64> {
    let my_pub_key = agent_info()?.agent_latest_pubkey;

    if AgentPubKey::from(input.counterparty_pub_key.clone()).eq(&my_pub_key) {
        return Err(crate::err("An agent cannot create an offer to themselves"));
    }

    let intent = match input.intent_type {
        IntentType::Offer => Intent {
            spender_pub_key: AgentPubKeyB64::from(my_pub_key.clone()),
            recipient_pub_key: input.counterparty_pub_key.clone(),
            amount: input.amount,
        },
        IntentType::Request => Intent {
            spender_pub_key: input.counterparty_pub_key.clone(),
            recipient_pub_key: AgentPubKeyB64::from(my_pub_key.clone()),
            amount: input.amount,
        },
    };

    let header_hash = create_entry(&intent)?;

    create_link(
        EntryHash::from(my_pub_key),
        header_hash.clone().retype(hash_type::Entry),
        HdkLinkType::Any,
        (),
    )?;
    create_link(
        EntryHash::from(AgentPubKey::from(intent.get_counterparty()?)),
        header_hash.clone().retype(hash_type::Entry),
        HdkLinkType::Any,
        (),
    )?;

    Ok(header_hash.into())
}

// TODO: pass the chain top from the UI?
#[derive(Serialize, Deserialize, Debug)]
pub struct AcceptIntentInput {
    intent_hash: HeaderHashB64,
    counterparty_chain_top: HeaderHashB64,
}
#[hdk_extern]
pub fn accept_intent(intent_hash: HeaderHashB64) -> ExternResult<(HeaderHashB64, Transaction)> {
    let intent_element = get(
        HeaderHash::from(intent_hash.clone()),
        GetOptions::default(),
    )?
    .ok_or(WasmError::Guest(String::from(
        "Couldn't get intent",
    )))?;

    let counterparty_chain_top = get_chain_top(intent_element.header().author().clone())?;

    let result = transaction::attempt_create_transaction_for_intent(
        intent_element.clone(),
        counterparty_chain_top.into(),
    )?;

    create_link(
        intent_element.header_address().clone().retype(hash_type::Entry),
        HeaderHash::from(result.0.clone()).retype(hash_type::Entry),
        HdkLinkType::Any,
        (),
    )?;

    Ok(result)
}

fn get_chain_top(agent_pub_key: AgentPubKey) -> ExternResult<HeaderHash> {
    let filter = ChainQueryFilter::new().entry_type(Transaction::entry_type()?);

    let activity = get_agent_activity(agent_pub_key, filter, ActivityRequest::Full)?;

    let highest_observed = activity.highest_observed.ok_or(WasmError::Guest(String::from("Counterparty highest observed was empty")))?;

    if highest_observed.hash.len() != 1 {
        return Err(WasmError::Guest(String::from("Counterparty highest observed was more than one")));
    }

    Ok(highest_observed.hash[0].clone())
}