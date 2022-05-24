use hdk::prelude::holo_hash::*;
use hdk::prelude::*;
use holochain::test_utils::consistency_10s;
use holochain::{conductor::config::ConductorConfig, sweettest::*};

#[tokio::test(flavor = "multi_thread")]
async fn simple_transaction() {
    // Use prebuilt DNA file
    let dna_path = std::env::current_dir()
        .unwrap()
        .join("../../example/workdir/mutual_credit.dna");
    let dna = SweetDnaFile::from_bundle(&dna_path).await.unwrap();

    // Set up conductors
    let mut conductors = SweetConductorBatch::from_config(2, ConductorConfig::default()).await;
    let apps = conductors.setup_app("mutual_credit", &[dna]).await.unwrap();
    conductors.exchange_peer_info().await;

    let ((alice,), (bobbo,)) = apps.into_tuples();

    let alice_zome = alice.zome("mutual_credit");
    let bob_zome = bobbo.zome("mutual_credit");

    let create_intent_input =  CreateIntentInput {
        intent_type: IntentType::Offer,
        counterparty_pub_key: bobbo.agent_key(),
        amount: 10.0,
    };

    let intent_hash: HeaderHashB64 = conductors[0]
        .call(&alice_zome, "create_intent", create_intent_input)
        .await;

    consistency_10s(&[&alice, &bobbo]).await;

    let txn: (HeaderHashB64, Transaction) =
        conductors[1].call(&bob_zome, "accept_intent", intent_hash).await;

}