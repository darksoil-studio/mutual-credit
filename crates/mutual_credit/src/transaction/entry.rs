use hdk::prelude::holo_hash::*;
use hdk::prelude::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TransactionParty {
    pub agent_pub_key: AgentPubKeyB64,
    pub previous_transaction_hash: Option<HeaderHashB64>,
    pub resulting_balance: f64,
}

#[hdk_entry(id = "transaction", visibility = "public")]
#[derive(Clone)]
#[serde(rename_all = "camelCase")]
pub struct Transaction {
    pub spender: TransactionParty,
    pub recipient: TransactionParty,
    pub transaction_intent_header_hash: HeaderHashB64,
    pub amount: f64,
}

impl Transaction {
    pub fn entry_type() -> ExternResult<EntryType> {
        Ok(EntryType::App(AppEntryType::new(
            entry_def_index!(Transaction)?,
            zome_info()?.id,
            EntryVisibility::Public,
        )))
    }

    pub fn from_previous_transactions(
        spender: AgentPubKeyB64,
        recipient: AgentPubKeyB64,
        previous_spender_transaction: Option<(HeaderHashB64, Transaction)>,
        previous_recipient_transaction: Option<(HeaderHashB64, Transaction)>,
        amount: f64,
        transaction_intent_header_hash: HeaderHashB64,
    ) -> ExternResult<Transaction> {
        let previous_spender_balance = balance_from_previous_transaction(
            spender,
            previous_spender_transaction.map(|(_, t)| t),
        )?;
        let previous_recipient_balance = balance_from_previous_transaction(
            recipient,
            previous_recipient_transaction.map(|(_, t)| t),
        )?;

        let resulting_spender_balance = previous_spender_balance - amount;
        let resulting_recipient_balance = previous_recipient_balance - amount;

        let spender = TransactionParty {
            agent_pub_key: spender,
            previous_transaction_hash: previous_spender_transaction.map(|(h, _)| h),
            resulting_balance: resulting_spender_balance,
        };
        let recipient = TransactionParty {
            agent_pub_key: recipient,
            previous_transaction_hash: previous_recipient_transaction.map(|(h, _)| h),
            resulting_balance: resulting_recipient_balance,
        };

        let transaction = Transaction {
            spender,
            recipient,
            amount,
            transaction_intent_header_hash,
        };

        Ok(transaction)
    }

    fn get_party(&self, agent_pub_key: &AgentPubKeyB64) -> ExternResult<TransactionParty> {
        if self.spender.agent_pub_key.eq(agent_pub_key) {
            Ok(self.spender)
        } else if self.recipient.agent_pub_key.eq(agent_pub_key) {
            Ok(self.recipient)
        } else {
            Err(WasmError::Guest(String::from(
                "This agent did not participate in the transaction",
            )))
        }
    }
}

fn balance_from_previous_transaction(
    for_agent: AgentPubKeyB64,
    previous_transaction: Option<Transaction>,
) -> ExternResult<f64> {
    match previous_transaction {
        None => Ok(0.0),
        Some(txn) => {
            let party = txn.get_party(&for_agent)?;
            Ok(party.resulting_balance)
        }
    }
}
