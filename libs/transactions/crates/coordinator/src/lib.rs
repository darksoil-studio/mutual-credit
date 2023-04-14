use hc_zome_mutual_credit_transactions_types::Transaction;
use hdk::prelude::*;

mod countersigning;
mod handlers;
mod utils;

pub use handlers::*;
pub use utils::*;

#[hdk_extern]
pub fn init(_: ()) -> ExternResult<InitCallbackResult> {
    let mut functions = BTreeSet::new();

    functions.insert((zome_info()?.name, "recv_remote_signal".into()));

    let grant = ZomeCallCapGrant {
        access: CapAccess::Unrestricted,
        functions: GrantedFunctions::Listed(functions),
        tag: "".into(),
    };
    create_cap_grant(grant)?;

    let mut functions = BTreeSet::new();
    functions.insert((zome_info()?.name, "transaction_preflight".into()));

    let grant = ZomeCallCapGrant {
        access: CapAccess::Unrestricted,
        functions: GrantedFunctions::Listed(functions),
        tag: "".into(),
    };
    create_cap_grant(grant)?;

    let mut functions = BTreeSet::new();
    functions.insert((zome_info()?.name, "request_create_transaction".into()));

    let grant = ZomeCallCapGrant {
        access: CapAccess::Unrestricted,
        functions: GrantedFunctions::Listed(functions),
        tag: "".into(),
    };
    create_cap_grant(grant)?;

    Ok(InitCallbackResult::Pass)
}

#[hdk_extern]
fn recv_remote_signal(signal: Signal) -> ExternResult<()> {
    emit_signal(&signal)?;
    Ok(())
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum Signal {
    NewTransactionCreated { transaction: Record },
}

#[hdk_extern(infallible)]
fn post_commit(actions: Vec<SignedActionHashed>) {
    if let Err(err) = inner_post_commit(actions) {
        error!("Error executing post commit: {:?}", err);
    }
}

fn inner_post_commit(actions: Vec<SignedActionHashed>) -> ExternResult<()> {
    let my_transactions = query_my_transactions(())?;

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
        let get_inputs = my_new_transactions
            .into_iter()
            .map(|h| GetInput::new(h.action_address().clone().into(), Default::default()))
            .collect();

        let records: Vec<Record> = HDK
            .with(|hdk| hdk.borrow().get(get_inputs))?
            .into_iter()
            .filter_map(|r| r)
            .collect();

        for transaction_record in records.clone() {
            let transaction = Transaction::try_from(transaction_record.clone())?;

            let signal = Signal::NewTransactionCreated {
                transaction: transaction_record,
            };
            error!("Emitting very important signal");
            emit_signal(signal.clone())?;

            remote_signal(signal, vec![transaction.get_counterparty()?.agent_pub_key])?;
        }
    }
    Ok(())
}
