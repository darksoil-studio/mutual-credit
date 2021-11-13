use hdk::prelude::*;
use holo_hash::{AgentPubKeyB64, EntryHashB64};

use crate::{
    offer::Offer,
    utils::{self, Hashed},
};

#[hdk_entry(id = "transaction", visibility = "public")]
#[derive(Clone)]
pub struct Transaction {
    spender_pub_key: AgentPubKeyB64,
    recipient_pub_key: AgentPubKeyB64,
    offer_hash: EntryHashB64,
    amount: f64,
    timestamp: Timestamp,
}

pub fn create_transaction_for_offer(offer: Offer) -> ExternResult<Hashed<Transaction>> {
    let time = sys_time()?;
    let transaction = Transaction {
        spender_pub_key: offer.spender_pub_key.clone(),
        recipient_pub_key: offer.recipient_pub_key.clone(),
        amount: offer.amount,
        offer_hash: EntryHashB64::from(hash_entry(&offer)?),
        timestamp: time
    };

    create_entry(&transaction)?;

    let transaction_hash = hash_entry(&transaction)?;

    create_link(
        AgentPubKey::from(offer.spender_pub_key).into(),
        transaction_hash.clone(),
        (),
    )?;
    create_link(
        AgentPubKey::from(offer.recipient_pub_key).into(),
        transaction_hash.clone(),
        (),
    )?;

    Ok(Hashed {
        hash: EntryHashB64::from(transaction_hash),
        content: transaction,
    })
}

#[hdk_extern]
pub fn get_transactions_for_agent(
    agent_pub_key: AgentPubKeyB64,
) -> ExternResult<Vec<Hashed<Transaction>>> {
    let links = get_links(AgentPubKey::from(agent_pub_key).into(), None)?;

    let transactions: Vec<Hashed<Transaction>> = links
        .iter()
        .map(|link| {
            let transaction: Transaction = utils::try_get_and_convert(link.target.clone())?;
            Ok(Hashed {
                hash: EntryHashB64::from(link.target.clone()),
                content: transaction,
            })
        })
        .collect::<ExternResult<Vec<Hashed<Transaction>>>>()?;

    Ok(transactions)
}
