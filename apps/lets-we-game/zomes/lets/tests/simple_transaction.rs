use std::collections::BTreeMap;

use hc_lib_transaction_requests::*;
use hc_lib_transactions::*;
use hdk::prelude::holo_hash::*;

use holochain::test_utils::consistency_10s;
use holochain::{conductor::config::ConductorConfig, sweettest::*};

#[tokio::test(flavor = "multi_thread")]
async fn simple_transaction() {
    // Use prebuilt DNA file
    let dna_path = std::env::current_dir()
        .unwrap()
        .join("../../workdir/lets.dna");
    let dna = SweetDnaFile::from_bundle(&dna_path).await.unwrap();

    // Set up conductors
    let mut conductors = SweetConductorBatch::from_config(2, ConductorConfig::default()).await;
    let apps = conductors.setup_app("lets", &[dna]).await.unwrap();
    conductors.exchange_peer_info().await;

    let ((alice,), (bobbo,)) = apps.into_tuples();

    let alice_zome = alice.zome("lets");
    let bob_zome = bobbo.zome("lets");

    println!("Alice {}", alice.agent_pubkey());
    println!("Bob {}", bobbo.agent_pubkey());

    consistency_10s(&[&alice, &bobbo]).await;

    let map: BTreeMap<HeaderHashB64, Transaction> = conductors[1]
        .call(&bob_zome, "query_my_transactions", ())
        .await;

    assert_eq!(map.len(), 0);

    let transaction_request_input = CreateTransactionRequestInput {
        transaction_request_type: TransactionRequestType::Send,
        counterparty_pub_key: AgentPubKeyB64::from(bobbo.agent_pubkey().clone()),
        amount: 10.0,
    };

    let (transaction_request_hash, _): (HeaderHashB64, TransactionRequest) = conductors[0]
        .call(
            &alice_zome,
            "create_transaction_request",
            transaction_request_input,
        )
        .await;

    consistency_10s(&[&alice, &bobbo]).await;

    let transaction_requests: BTreeMap<HeaderHashB64, TransactionRequest> = conductors[0]
        .call(&alice_zome, "get_my_transaction_requests", ())
        .await;

    assert_eq!(transaction_requests.len(), 1);

    let transaction_requests: BTreeMap<HeaderHashB64, TransactionRequest> = conductors[1]
        .call(&bob_zome, "get_my_transaction_requests", ())
        .await;

    assert_eq!(transaction_requests.len(), 1);

    let _txn: (HeaderHashB64, Transaction) = conductors[1]
        .call(
            &bob_zome,
            "accept_transaction_request",
            transaction_request_hash,
        )
        .await;

    consistency_10s(&[&alice, &bobbo]).await;

    let transactions: BTreeMap<HeaderHashB64, Transaction> = conductors[1]
        .call(&bob_zome, "query_my_transactions", ())
        .await;

    assert_eq!(transactions.len(), 1);
    assert_eq!(transactions.into_iter().next().unwrap().1.amount, 10.0);

    let transaction_requests: BTreeMap<HeaderHashB64, TransactionRequest> = conductors[0]
        .call(&alice_zome, "get_my_transaction_requests", ())
        .await;

    assert_eq!(transaction_requests.len(), 1);

    let transaction_requests: BTreeMap<HeaderHashB64, TransactionRequest> = conductors[1]
        .call(&bob_zome, "get_my_transaction_requests", ())
        .await;

    assert_eq!(transaction_requests.len(), 0);
}
