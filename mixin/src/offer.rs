use hdk::prelude::holo_hash::{EntryHashB64, AgentPubKeyB64};
use hdk::prelude::*;
use transaction::Transaction;

use crate::{
    signals::SignalType,
    transaction,
    utils::{self, Hashed},
};

#[hdk_entry(id = "offer", visibility = "private")]
#[derive(Clone)]
pub struct Offer {
    pub spender_pub_key: AgentPubKeyB64,
    pub recipient_pub_key: AgentPubKeyB64,
    pub amount: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateOfferInput {
    recipient_pub_key: AgentPubKeyB64,
    amount: f64,
}
#[hdk_extern]
pub fn create_offer(input: CreateOfferInput) -> ExternResult<EntryHashB64> {
    let my_pub_key = agent_info()?.agent_latest_pubkey;

    if AgentPubKey::from(input.recipient_pub_key.clone()).eq(&my_pub_key) {
        return Err(crate::err("An agent cannot create an offer to themselves"));
    }

    let offer = Offer {
        spender_pub_key: AgentPubKeyB64::from(my_pub_key),
        amount: input.amount,
        recipient_pub_key: input.recipient_pub_key.clone(),
    };

    let my_offer_hash = internal_create_offer(&offer)?;

    // TODO: handle the result
    call_remote(
        input.recipient_pub_key.into(),
        zome_info()?.name,
        "receive_offer".into(),
        None,
        &Hashed {
            hash: my_offer_hash.clone(),
            content: offer,
        },
    )?;

    Ok(my_offer_hash)
}

#[hdk_extern]
pub fn receive_offer(offer: Hashed<Offer>) -> ExternResult<EntryHashB64> {
    let offer_hash = internal_create_offer(&offer.content)?;

    emit_signal(SignalType::OfferReceived(offer))?;

    Ok(offer_hash)
}

#[hdk_extern]
pub fn accept_offer(offer_hash: EntryHashB64) -> ExternResult<EntryHashB64> {
    let maybe_offer = internal_query_offer(offer_hash.into())?;

    let offer = maybe_offer.ok_or(crate::err("Offer not found"))?;

    let hashed_transaction = transaction::create_transaction_for_offer(offer.clone())?;

    // Notify new transaction to counterparty
    call_remote(
        offer.spender_pub_key.into(),
        zome_info()?.name,
        "notify_accepted_offer".into(),
        None,
        &hashed_transaction,
    )?;

    Ok(hashed_transaction.hash)
}

#[hdk_extern]
pub fn notify_accepted_offer(transaction: Hashed<Transaction>) -> ExternResult<()> {
    emit_signal(SignalType::OfferAccepted(transaction))?;

    Ok(())
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
