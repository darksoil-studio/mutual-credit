use crate::{signals::SignalType, transaction, utils};
use hdk::prelude::holo_hash::*;
use hdk::prelude::*;

#[hdk_entry(id = "transaction_intent")]
#[derive(Clone)]
#[serde(rename_all = "camelCase")]
pub struct TransactionIntent {
    pub spender_pub_key: AgentPubKeyB64,
    pub recipient_pub_key: AgentPubKeyB64,
    pub amount: f64,
}

impl TransactionIntent {
    pub fn get_counterparty(&self) -> ExternResult<AgentPubKeyB64> {
        let my_pub_key = agent_info()?.agent_initial_pubkey;

        if AgentPubKey::from(self.spender_pub_key).eq(&my_pub_key) {
            Ok(self.recipient_pub_key.clone())
        } else if AgentPubKey::from(self.recipient_pub_key).eq(&my_pub_key) {
            Ok(self.spender_pub_key.clone())
        } else {
            Err(WasmError::Guest(String::from(
                "I don't participate in this offer",
            )))
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum TransactionIntentType {
    Offer,
    Request,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CreateTransactionIntentInput {
    intent_type: TransactionIntentType,
    counterparty_pub_key: AgentPubKeyB64,
    amount: f64,
}
#[hdk_extern]
pub fn create_transaction_intent(
    input: CreateTransactionIntentInput,
) -> ExternResult<HeaderHashB64> {
    let my_pub_key = agent_info()?.agent_latest_pubkey;

    if AgentPubKey::from(input.counterparty_pub_key.clone()).eq(&my_pub_key) {
        return Err(crate::err("An agent cannot create an offer to themselves"));
    }

    let intent = match input.intent_type {
        TransactionIntentType::Offer => TransactionIntent {
            spender_pub_key: AgentPubKeyB64::from(my_pub_key),
            recipient_pub_key: input.counterparty_pub_key.clone(),
            amount: input.amount,
        },
        TransactionIntentType::Request => TransactionIntent {
            spender_pub_key: input.counterparty_pub_key.clone(),
            recipient_pub_key: AgentPubKeyB64::from(my_pub_key),
            amount: input.amount,
        },
    };

    let header_hash = create_entry(&intent)?;

    create_link(
        EntryHash::from(AgentPubKey::from(offer.get_counterparty()?)),
        header_hash.retype(hash_type::Entry),
        HdkLinkType::Any,
        (),
    )?;

    Ok(header_hash.into())
}

#[hdk_extern]
pub fn accept_transaction_intent(transaction_intent_header_hash: HeaderHashB64) -> ExternResult<(HeaderHashB64, Transaction)> {
    transaction::attempt_create_transaction_for_intent(transaction_intent_header_hash)
}

#[hdk_extern]
pub fn query_my_pending_offers(_: ()) -> ExternResult<Vec<Hashed<Offer>>> {
    let offers_elements = query_all_offers()?;

    let offers: Vec<Hashed<Offer>> = offers_elements
        .into_iter()
        .map(|element| {
            let offer = utils::try_from_element(element.clone())?;
            Ok(Hashed {
                hash: EntryHashB64::from(element.header().entry_hash().unwrap().clone()),
                content: offer,
            })
        })
        .collect::<ExternResult<Vec<Hashed<Offer>>>>()?;

    Ok(offers)
}

/** Helper functions */
fn internal_create_offer(offer: &Offer) -> ExternResult<EntryHashB64> {
    create_entry(offer)?;

    let offer_hash = hash_entry(offer)?;
    Ok(EntryHashB64::from(offer_hash))
}

fn query_all_offers() -> ExternResult<Vec<Element>> {
    let offer_entry_type = EntryType::App(AppEntryType::new(
        entry_def_index!(Offer)?,
        zome_info()?.id,
        EntryVisibility::Private,
    ));
    let filter = ChainQueryFilter::new()
        .entry_type(offer_entry_type)
        .include_entries(true);
    let query_result = query(filter)?;

    Ok(query_result)
}

fn internal_query_offer(offer_hash: EntryHash) -> ExternResult<Option<Offer>> {
    let all_offers = query_all_offers()?;

    let maybe_offer_element = all_offers.into_iter().find(|offer_element| {
        let maybe_entry_hash = offer_element.header().entry_hash();
        maybe_entry_hash.is_some() && maybe_entry_hash.unwrap().eq(&offer_hash)
    });

    match maybe_offer_element {
        None => Ok(None),
        Some(offer_element) => utils::try_from_element(offer_element).map(|o| Some(o)),
    }
}
