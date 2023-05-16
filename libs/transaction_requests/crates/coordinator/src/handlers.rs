use hc_zome_mutual_credit_transaction_requests_integrity::{
    EntryTypes, LinkTypes, TransactionRequestType,
};
use hc_zome_mutual_credit_transactions_types::{AttemptCreateTransactionInput, Transaction};
use hdk::prelude::*;

use crate::{
    utils::{build_transaction, call_transactions, get_counterparty},
    Signal, TransactionRequest,
};

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateTransactionRequestInput {
    pub transaction_request_type: TransactionRequestType,
    pub counterparty_pub_key: AgentPubKey,
    pub amount: f64,
}

#[hdk_extern]
pub fn create_transaction_request(input: CreateTransactionRequestInput) -> ExternResult<Record> {
    let my_pub_key = agent_info()?.agent_latest_pubkey;

    if input.counterparty_pub_key.clone().eq(&my_pub_key) {
        return Err(wasm_error!(WasmErrorInner::Guest(String::from(
            "An agent cannot create an transaction request to themselves",
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
        action_hash.clone(),
        LinkTypes::AgentToTransactionRequest,
        (),
    )?;
    create_link(
        input.counterparty_pub_key.clone(),
        action_hash.clone(),
        LinkTypes::AgentToTransactionRequest,
        (),
    )?;

    let record = get(action_hash.clone(), GetOptions::default())?.ok_or(wasm_error!(
        WasmErrorInner::Guest(String::from("Could not find the newly created Transaction"))
    ))?;
    let signal = Signal::TransactionRequestCreated {
        transaction_request: record.clone(),
    };
    emit_signal(signal.clone())?;
    remote_signal(signal, vec![input.counterparty_pub_key])?;

    Ok(record)
}

#[hdk_extern]
pub fn clear_transaction_requests(
    transaction_requests_hashes: Vec<ActionHash>,
) -> ExternResult<()> {
    let my_pub_key = agent_info()?.agent_initial_pubkey;
    let links = get_links(my_pub_key, LinkTypes::AgentToTransactionRequest, None)?;

    for link in links {
        let transaction_request_hash = ActionHash::from(link.target.clone());
        if transaction_requests_hashes
            .iter()
            .find(|hash| transaction_request_hash.eq(hash))
            .is_some()
        {
            delete_link(link.create_link_hash)?;
            emit_signal(Signal::TransactionRequestCleared {
                transaction_request_hash,
            })?;
        }
    }

    Ok(())
}

#[hdk_extern]
pub fn cancel_transaction_request(transaction_request_hash: ActionHash) -> ExternResult<()> {
    delete_entry(transaction_request_hash.clone())?;
    clear_transaction_requests(vec![transaction_request_hash.clone()])?;

    let transaction_request_record = get_transaction_request(transaction_request_hash.clone())?
        .ok_or(wasm_error!(WasmErrorInner::Guest(String::from(
            "Couldn't get transaction request",
        ))))?;

    let transaction_request: TransactionRequest = transaction_request_record
        .record
        .entry()
        .to_app_option()
        .map_err(|e| {
            wasm_error!(WasmErrorInner::Guest(format!(
                "Failed to convert entry to app option: {}",
                e
            )))
        })?
        .ok_or(wasm_error!(WasmErrorInner::Guest(String::from(
            "Malformed transaction request",
        ))))?;
    let counterparty = get_counterparty(&transaction_request)?;

    let signal = Signal::TransactionRequestCancelled {
        transaction_request_hash,
    };
    emit_signal(signal.clone())?;

    remote_signal(signal, vec![counterparty])?;

    Ok(())
}

#[hdk_extern]
pub fn reject_transaction_request(transaction_request_hash: ActionHash) -> ExternResult<()> {
    delete_entry(transaction_request_hash.clone())?;
    clear_transaction_requests(vec![transaction_request_hash.clone()])?;
    let transaction_request_record = get_transaction_request(transaction_request_hash.clone())?
        .ok_or(wasm_error!(WasmErrorInner::Guest(String::from(
            "Couldn't get transaction request",
        ))))?;

    let transaction_request: TransactionRequest = transaction_request_record
        .record
        .entry()
        .to_app_option()
        .map_err(|e| {
            wasm_error!(WasmErrorInner::Guest(format!(
                "Failed to convert entry to app option: {}",
                e
            )))
        })?
        .ok_or(wasm_error!(WasmErrorInner::Guest(String::from(
            "Malformed transaction request",
        ))))?;
    let counterparty = get_counterparty(&transaction_request)?;

    let signal = Signal::TransactionRequestRejected {
        transaction_request_hash,
    };
    emit_signal(signal.clone())?;

    remote_signal(signal, vec![counterparty])?;

    Ok(())
}

#[hdk_extern]
pub fn accept_transaction_request(transaction_request_hash: ActionHash) -> ExternResult<Record> {
    let transaction_request_record = get(transaction_request_hash.clone(), GetOptions::default())?
        .ok_or(wasm_error!(WasmErrorInner::Guest(String::from(
            "Couldn't get transaction request",
        ))))?;

    let transaction_request: TransactionRequest = transaction_request_record
        .entry()
        .to_app_option()
        .map_err(|e| {
            wasm_error!(WasmErrorInner::Guest(format!(
                "Failed to convert entry to app option: {}",
                e
            )))
        })?
        .ok_or(wasm_error!(WasmErrorInner::Guest(String::from(
            "Malformed transaction request",
        ))))?;
    let counterparty = get_counterparty(&transaction_request)?;

    let counterparty_chain_top = get_chain_top(counterparty.into())?;

    let transaction = build_transaction(transaction_request_record)?;

    let result: Record = call_transactions(
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
    if let Err(err) = inner_post_commit(actions) {
        error!("Error executing post commit: {:?}", err);
    }
}

fn inner_post_commit(actions: Vec<SignedActionHashed>) -> ExternResult<()> {
    let my_transactions = query_my_transactions()?;

    let my_new_transactions: Vec<SignedActionHashed> = actions
        .into_iter()
        .filter(|h| {
            my_transactions
                .iter()
                .find(|transaction| transaction.action_address().eq(&h.action_address()))
                .is_some()
        })
        .collect();

    if my_new_transactions.len() > 0 {
        let result = call_remote(
            agent_info()?.agent_initial_pubkey,
            zome_info()?.name,
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
    Ok(())
}

#[hdk_extern]
pub fn clean_transaction_requests(_: ()) -> ExternResult<()> {
    let my_pub_key = agent_info()?.agent_initial_pubkey;
    let links = get_links(my_pub_key, LinkTypes::AgentToTransactionRequest, None)?;

    let my_transactions = query_my_transactions()?;

    for transaction_record in my_transactions {
        let transaction = Transaction::try_from(transaction_record.clone())?;
        let info = transaction.info.clone();

        let transaction_request_hash = ActionHash::try_from(info).map_err(|e| {
            wasm_error!(WasmErrorInner::Guest(format!(
                "Failed to deserialize transaction request hash: {}",
                e
            )))
        })?;

        if let Some(link) = links
            .iter()
            .find(|link| ActionHash::from(link.target.clone()).eq(&transaction_request_hash))
        {
            delete_link(link.create_link_hash.clone())?;

            create_link(
                transaction_request_hash.clone(),
                transaction_record.action_address().clone(),
                LinkTypes::TransactionRequestToTransaction,
                (),
            )?;
        }
    }

    Ok(())
}

fn query_my_transactions() -> ExternResult<Vec<Record>> {
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

    let my_transactions: Vec<Record> = result.decode().map_err(|e| {
        wasm_error!(WasmErrorInner::Guest(format!(
            "Failed to deserialize transactions: {}",
            e
        )))
    })?;

    Ok(my_transactions)
}

#[hdk_extern]
pub fn get_transaction_requests_for_agent(agent: AgentPubKey) -> ExternResult<Vec<ActionHash>> {
    let links = get_links(agent, LinkTypes::AgentToTransactionRequest, None)?;

    let action_hashes = links
        .into_iter()
        .map(|link| ActionHash::from(link.target))
        .collect();

    Ok(action_hashes)
}

#[hdk_extern]
pub fn get_transaction_request(
    transaction_request_hash: ActionHash,
) -> ExternResult<Option<RecordDetails>> {
    let maybe_details = get_details(transaction_request_hash, GetOptions::default())?;

    let Some(details) = maybe_details else {  return Ok(None);};

    match details {
        Details::Record(d) => Ok(Some(d)),
        _ => Err(wasm_error!(WasmErrorInner::Guest(
            "Error fetching the details for the transaction request".to_string()
        ))),
    }
}

#[hdk_extern]
pub fn get_transaction_for_transaction_request(
    transaction_request_hash: ActionHash,
) -> ExternResult<Option<ActionHash>> {
    let links = get_links(
        transaction_request_hash,
        LinkTypes::TransactionRequestToTransaction,
        None,
    )?;

    Ok(links
        .first()
        .map(|link| ActionHash::from(link.target.clone())))
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
