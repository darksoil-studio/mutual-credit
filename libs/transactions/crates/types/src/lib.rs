use hdi::prelude::*;
use hdk::prelude::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct AttemptCreateTransactionInput {
    pub transaction: Transaction,
    pub counterparty_chain_top: ActionHash,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TransactionParty {
    pub agent_pub_key: AgentPubKey,
    pub previous_transaction_hash: Option<ActionHash>,
    pub resulting_balance: f64,
}

#[hdk_entry_helper]
#[derive(Clone)]
pub struct Transaction {
    pub spender: TransactionParty,
    pub recipient: TransactionParty,
    pub amount: f64,
    pub info: SerializedBytes,
}

impl Transaction {
    pub fn try_from_entry(entry: Entry) -> ExternResult<Transaction> {
        match entry {
            Entry::CounterSign(_session_data, entry_bytes) => {
                let transaction = Transaction::try_from(entry_bytes.into_sb()).map_err(|e| {
                    wasm_error!(WasmErrorInner::Guest(format!(
                        "Failed to deserialize Transaction: {}",
                        e
                    )))
                })?;
                Ok(transaction)
            }
            _ => Err(wasm_error!(WasmErrorInner::Guest(String::from(
                "Malformed entry"
            )))),
        }
    }

    pub fn from_previous_transactions(
        spender: AgentPubKey,
        recipient: AgentPubKey,
        previous_spender_transaction: Option<(ActionHash, Transaction)>,
        previous_recipient_transaction: Option<(ActionHash, Transaction)>,
        amount: f64,
        info: SerializedBytes,
    ) -> ExternResult<Transaction> {
        let previous_spender_balance = balance_from_previous_transaction(
            spender.clone(),
            previous_spender_transaction.clone().map(|(_, t)| t),
        )?;
        let previous_recipient_balance = balance_from_previous_transaction(
            recipient.clone(),
            previous_recipient_transaction.clone().map(|(_, t)| t),
        )?;

        let resulting_spender_balance = previous_spender_balance - amount;
        let resulting_recipient_balance = previous_recipient_balance + amount;

        let spender = TransactionParty {
            agent_pub_key: spender.into(),
            previous_transaction_hash: previous_spender_transaction.map(|(h, _)| h),
            resulting_balance: resulting_spender_balance,
        };
        let recipient = TransactionParty {
            agent_pub_key: recipient.into(),
            previous_transaction_hash: previous_recipient_transaction.map(|(h, _)| h),
            resulting_balance: resulting_recipient_balance,
        };

        let transaction = Transaction {
            spender,
            recipient,
            amount,
            info,
        };

        Ok(transaction)
    }

    pub fn get_party(&self, agent_pub_key: &AgentPubKey) -> ExternResult<TransactionParty> {
        if AgentPubKey::from(self.spender.agent_pub_key.clone()).eq(agent_pub_key) {
            Ok(self.spender.clone())
        } else if AgentPubKey::from(self.recipient.agent_pub_key.clone()).eq(agent_pub_key) {
            Ok(self.recipient.clone())
        } else {
            Err(wasm_error!(WasmErrorInner::Guest(String::from(
                "This agent did not participate in the transaction",
            ))))
        }
    }

    pub fn get_counterparty(&self) -> ExternResult<TransactionParty> {
        let my_pub_key: AgentPubKey = agent_info()?.agent_initial_pubkey;

        if my_pub_key.eq(&self.spender.agent_pub_key) {
            Ok(self.spender.clone())
        } else if my_pub_key.eq(&self.recipient.agent_pub_key) {
            Ok(self.spender.clone())
        } else {
            Err(wasm_error!(WasmErrorInner::Guest(String::from(
                "I don't participate in this Transaction",
            ))))
        }
    }
}

fn balance_from_previous_transaction(
    for_agent: AgentPubKey,
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
