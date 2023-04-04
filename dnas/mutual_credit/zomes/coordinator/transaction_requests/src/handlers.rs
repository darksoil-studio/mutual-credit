use std::collections::BTreeMap;

use transaction_requests_integrity::{call_transactions, CreateTransactionRequestInput};
use transactions_integrity::AttemptCreateTransactionInput;
use transaction_types::Transaction;
use hdk::prelude::holo_hash::*;
use hdk::prelude::*;
use transaction_requests_integrity::{LinkTypes, EntryTypes};

use crate::{utils::build_transaction, TransactionRequest};
use transaction_types::TransactionRequestType;

#[hdk_extern]
pub fn create_transaction_request(
    input: CreateTransactionRequestInput,
) -> ExternResult<(ActionHash, TransactionRequest)> {
    let my_pub_key = agent_info()?.agent_latest_pubkey;

    if input.counterparty_pub_key.clone().eq(&my_pub_key) {
        return Err(wasm_error!(WasmErrorInner::Guest(String::from(
            "An agent cannot create an offer to themselves",
        ))));
    }

    let transaction_request = match input.transaction_request_type {
        TransactionRequestType::Send => TransactionRequest {
            spender_pub_key: my_pub_key.clone(),
            recipient_pub_key: input.counterparty_pub_key.clone(),
            amount: input.amount,
        },
        TransactionRequestType::Receive => TransactionRequest {
            spender_pub_key: input.counterparty_pub_key.clone(),
            recipient_pub_key: my_pub_key.clone(),
            amount: input.amount,
        },
    };

    let action_hash = create_entry(EntryTypes::TransactionRequest(transaction_request.clone()))?;

    create_link(
        my_pub_key,
        action_hash.clone().retype(hash_type::Entry),
        LinkTypes::AgentToTransactionRequest,
        (),
    )?;
    create_link(
        AgentPubKey::from(transaction_request.get_counterparty()?),
        action_hash.clone().retype(hash_type::Entry),
        LinkTypes::CounterpartyToTransactionRequest,
        (),
    )?;

    Ok((action_hash.into(), transaction_request))
}

#[hdk_extern]
pub fn cancel_transaction_request(_transaction_request_hash: ActionHash) -> ExternResult<()> {
    Ok(())
}

#[hdk_extern]
pub fn reject_transaction_request(_transaction_request_hash: ActionHash) -> ExternResult<()> {
    Ok(())
}

#[hdk_extern]
pub fn clear_transaction_request(_transaction_request_hash: ActionHash) -> ExternResult<()> {
    Ok(())
}

#[hdk_extern]
pub fn accept_transaction_request(
    transaction_request_hash: ActionHash,
) -> ExternResult<(ActionHash, Transaction)> {
    let transaction_request_record = get(
        transaction_request_hash.clone(),
        GetOptions::default(),
    )?
    .ok_or(wasm_error!(WasmErrorInner::Guest(String::from(
        "Couldn't get transaction request",
    ))))?;

    let transaction_request: TransactionRequest = transaction_request_record
        .entry()
        .to_app_option()
        .map_err(|e| wasm_error!(WasmErrorInner::Guest(format!("Failed to convert entry to app option: {}", e))))?
        .ok_or(wasm_error!(WasmErrorInner::Guest(String::from(
            "Malformed transaction request",
        ))))?;
    let counterparty = transaction_request.get_counterparty()?;

    let counterparty_chain_top = get_chain_top(counterparty.into())?;

    let transaction = build_transaction(transaction_request_record)?;

    let result: (ActionHash, Transaction) = call_transactions(
        "attempt_create_transaction".into(),
        AttemptCreateTransactionInput {
            transaction,
            counterparty_chain_top: counterparty_chain_top.into(),
        },
    )?;

    Ok(result)
}

#[hdk_extern(infallible)]
fn post_commit(actions: Vec<SignedActionHashed>) {
    let my_transactions = query_my_transactions().unwrap();

    let my_new_transactions: Vec<SignedActionHashed> = actions
        .into_iter()
        .filter(|h| my_transactions.contains_key(&h.action_address().clone()))
        .collect();

    if my_new_transactions.len() > 0 {
        let get_inputs = my_new_transactions
            .into_iter()
            .map(|h| GetInput::new(h.action_address().clone().into(), Default::default()))
            .collect();

        let elements = HDK.with(|hdk| hdk.borrow().get(get_inputs)).unwrap();

        let transactions_i_created: Vec<_> = elements
            .into_iter()
            .filter_map(|el| el)
            .filter_map(|el| el.entry().as_option().map(|e| e.clone()))
            .filter(|entry| match entry {
                Entry::CounterSign(session_data, _entry_bytes) => {
                    let state = session_data
                        .agent_state_for_agent(&agent_info().unwrap().agent_initial_pubkey)
                        .unwrap();
                    state.agent_index().to_owned() == 0
                }
                _ => false,
            })
            .collect();

        if transactions_i_created.len() > 0 {
            let result = call_remote(
                agent_info().unwrap().agent_initial_pubkey,
                zome_info().unwrap().name,
                "clean_transaction_requests".into(),
                None,
                (),
            );

            match result.clone() {
                Ok(ZomeCallResponse::Ok(_)) => {}
                _ => error!(
                    "Error trying to clean the transaction requests {:?}",
                    result,
                ),
            };
        }
    }
}

#[hdk_extern]
pub fn clean_transaction_requests(_: ()) -> ExternResult<()> {
    let my_pub_key = agent_info()?.agent_initial_pubkey;
    let links = get_links(my_pub_key, LinkTypes::AgentToTransactionRequest, None)?;

    let my_transactions = query_my_transactions()?;

    for (transaction_hash, transaction) in my_transactions {
        let info = transaction.info.clone();

        let transaction_request_hash = ActionHash::try_from(info)
            .map_err(|e| wasm_error!(WasmErrorInner::Guest(format!("Failed to deserialize transaction request hash: {}", e))))?;

        if let Some(link) = links.iter().find(|link| {
            link.target
                .clone()
                .retype(hash_type::Action)
                .eq(&transaction_request_hash)
        }) {
            delete_link(link.create_link_hash.clone())?;

            create_link(
                transaction_request_hash.clone().retype(hash_type::Entry),
                ActionHash::from(transaction_hash).retype(hash_type::Entry),
                LinkTypes::RequestToTransactionAction,
                (),
            )?;
        }
    }

    Ok(())
}

fn query_my_transactions() -> ExternResult<BTreeMap<ActionHash, Transaction>> {
    let response = call_remote(
        agent_info().unwrap().agent_initial_pubkey,
        ZomeName::from("transactions"),
        "query_my_transactions".into(),
        None,
        (),
    )?;

    let result = match response {
        ZomeCallResponse::Ok(result) => Ok(result),
        _ => Err(wasm_error!(WasmErrorInner::Guest(format!(
            "Error querying my transactions: {:?}",
            response
        )))),
    }?;

    let my_transactions: BTreeMap<ActionHash, Transaction> = result.decode()
        .map_err(|e| wasm_error!(WasmErrorInner::Guest(format!("Failed to deserialize transactions: {}", e))))?;

    Ok(my_transactions)
}

#[hdk_extern]
pub fn get_my_transaction_requests(
    _: (),
) -> ExternResult<BTreeMap<ActionHash, TransactionRequest>> {
    let my_pub_key = agent_info()?.agent_initial_pubkey;
    let links = get_links(my_pub_key, LinkTypes::AgentToTransactionRequest, None)?;

    let get_inputs = links
        .into_iter()
        .map(|link| {
            GetInput::new(
                link.target.retype(hash_type::Action).into(),
                GetOptions::default(),
            )
        })
        .collect();

    let records = HDK.with(|hdk| hdk.borrow().get(get_inputs))?;

    let transaction_requests = records
        .into_iter()
        .filter_map(|el| el)
        .map(|r| {
            let action_hash = ActionHash::from(r.action_address().clone());

            let transaction_request: TransactionRequest =
                r.entry()
                    .to_app_option()
                    .map_err(|e| wasm_error!(WasmErrorInner::Guest(format!("Failed to convert entry to app option: {}", e))))?
                    .ok_or(wasm_error!(WasmErrorInner::Guest(String::from(
                        "Malformed transaction request",
                    ))))?;

            Ok((action_hash, transaction_request))
        })
        .collect::<ExternResult<BTreeMap<ActionHash, TransactionRequest>>>()?;

    Ok(transaction_requests)
}

fn get_chain_top(agent_pub_key: AgentPubKey) -> ExternResult<ActionHash> {
    let activity = get_agent_activity(
        agent_pub_key,
        ChainQueryFilter::new(),
        ActivityRequest::Full,
    )?;

    let highest_observed = activity
        .highest_observed
        .ok_or(wasm_error!(WasmErrorInner::Guest(String::from(
            "Counterparty highest observed was empty",
        ))))?;

    if highest_observed.hash.len() != 1 {
        return Err(wasm_error!(WasmErrorInner::Guest(String::from(
            "Counterparty highest observed was more than one",
        ))));
    }

    Ok(highest_observed.hash[0].clone())
}
