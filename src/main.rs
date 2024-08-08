#![allow(missing_docs)]
use subxt::{
    PolkadotConfig,
    utils::{AccountId32, MultiAddress},
    OnlineClient,
};
use subxt::backend::rpc::{RpcClient, rpc_params};
use anyhow::{Result, Context, anyhow};
use log::{info, error};
use serde_json;
use sp_core::crypto::Ss58Codec;
use codec::Encode;
use rand::Rng;
use base64::{encode};

// Metadata for the substrate Template node
#[subxt::subxt(runtime_metadata_path = "metadata.scale")]
pub mod statemint {}

// PolkadotConfig or SubstrateConfig will suffice for this example at the moment,
// but PolkadotConfig is a little more correct, having the right `Address` type.
type StatemintConfig = PolkadotConfig;
use subxt_signer::sr25519::dev::{self};


#[tokio::main]
async fn main() {
    env_logger::init();
    if let Err(err) = run().await {
        error!("{:?}", err);
    }
}

async fn run() -> Result<()> {

    let _api = OnlineClient::<StatemintConfig>::from_url("ws://127.0.0.1:9944").await?;
    println!("Connection with parachain established.");

    let alice: MultiAddress<AccountId32, ()> = dev::alice().public_key().into();
    println!("Alice: {:?}", alice);

    let account_id: AccountId32 = match alice {
        MultiAddress::Id(id) => id,
        _ => panic!("Expected MultiAddress::Id variant"),
    };

    // Convert AccountId32 to SS58 address
    let encoded: Vec<u8> = account_id.encode();
    let array: [u8; 32] = encoded.try_into().expect("AccountId32 should be 32 bytes");
    let ss58_address = Ss58Codec::to_ss58check(&sp_core::crypto::AccountId32::new(array));
    println!("SS58 Address: {}", ss58_address);


    let text = "Hey substrate";

     // Generate random data as bytes of vector with defined size
    let mut rng = rand::thread_rng();
    let random_data: Vec<u8> = (0..32).map(|_| rng.gen()).collect(); // 16 bytes of random data
    println!("Random Data: {:?}", random_data);
    
    let statement: String = text.to_string();
    let statement1 = encode(&random_data);
    println!("Statement1: {:?}", statement1);
    println!("Statement: {:?}", statement);

    // Create RPC client
    let rpc_client = RpcClient::from_url("ws://127.0.0.1:9944")
        .await
        .context("Failed to create RPC client")?;

    // Call the statement_submit RPC method
    let result: serde_json::Value = rpc_client
        .request("statement_submit", rpc_params![statement1, None::<subxt::utils::H256>, ss58_address])
        .await
        .context("Failed to submit statement via RPC")?;

    // Check the result
    match result {
        serde_json::Value::String(bytes) => {
            info!("Statement submitted via RPC successfully. Encoded extrinsic: {}", bytes);
        },
        _ => return Err(anyhow!("Unexpected RPC response: {:?}", result)),
    }

    Ok(())
}