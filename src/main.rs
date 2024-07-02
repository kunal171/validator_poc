#![allow(missing_docs)]
use subxt::{ OnlineClient, PolkadotConfig
};
use subxt_signer::sr25519::dev::{self};
use subxt::backend::{legacy::LegacyRpcMethods, rpc::{RpcClient, rpc_params}};



//metadata for the substrate Template node
#[subxt::subxt(runtime_metadata_path = "metadata.scale")]
pub mod statemint {}

// PolkadotConfig or SubstrateConfig will suffice for this example at the moment,
// but PolkadotConfig is a little more correct, having the right `Address` type.
type StatemintConfig = PolkadotConfig;

#[tokio::main]
pub async fn main() {
    if let Err(err) = run().await {
        eprintln!("{err}");
    }
}

async fn run() -> Result<(), Box<dyn std::error::Error>> {

    // Sending Message using Transaction 
    let api = OnlineClient::<StatemintConfig>::from_url("ws://127.0.0.1:9944").await?;
    println!("Connection with parachain established.");

    // let alice: MultiAddress<AccountId32, ()> = dev::alice().public_key().into();
    let alice_pair_signer = dev::alice();

    let remark_message = "Hello World".as_bytes();
    println!("{:?}", remark_message);

    let remark_extrinsic = statemint::tx().system().remark_with_event(remark_message.to_vec());

    // Submit the extrinsic
    let _result = api.
        tx()
        .sign_and_submit_then_watch_default(&remark_extrinsic, &alice_pair_signer)
        .await
        .map(|e| {
            println!("Collection creation submitted, waiting for transaction to be finalized...");
            e
        })?
        .wait_for_finalized_success()
        .await?;

        
    // Sending Message using RPC
    let rpc_client = RpcClient::from_url("ws://127.0.0.1:9944").await?;    

    // Use this to RPC methods:
    // let rpc = LegacyRpcMethods::<PolkadotConfig>::new(rpc_client.clone());

     // Encode the message
    let encoded_message = hex::encode(remark_message);
    println!("{:?}", encoded_message);


    let result = rpc_client.request("statement_submit", rpc_params![encoded_message]).await?;  
     
    Ok(())

}
