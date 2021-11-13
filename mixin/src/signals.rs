use crate::{offer::Offer, transaction::Transaction, utils::Hashed};
use hdk::prelude::*;

#[derive(Serialize, Deserialize, Debug)]
pub enum SignalType {
    OfferReceived(Hashed<Offer>),
    OfferAccepted(Hashed<Transaction>),
}
