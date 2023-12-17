use tonic::transport::Channel;

use super::two_phase_commit::two_phase_commit_service_client::TwoPhaseCommitServiceClient;
use super::two_phase_commit::{PreparePhaseReq, PreparePhaseResp};

use std::cell::RefCell;
use std::sync::Arc;


// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//     // creating a channel ie connection to server
//     let channel = tonic::transport::Channel::from_static("http://[::1]:50051")
//     .connect()
//     .await?;

//     // creating gRPC client from channel
//     let mut client= PreparePhaseClient::new(channel);
//     // creating a new Request
//     let request = tonic::Request::new(
//         PreparePhaseReq {
//             version: "1".to_owned(),
//             command: "vote?".to_owned(),
//         }
//     );

//     // sending request and waiting for response
//     let response = client.prepare(request).await?.into_inner();
//     println!("RESPONSE={:?}", response);
//     Ok(())
// }
#[tokio::main]
pub async fn init_rpc_client2() -> Result<(), Box<dyn std::error::Error>> {
    // creating a channel ie connection to server
    let channel = tonic::transport::Channel::from_static("http://[::1]:50051")
    .connect()
    .await?;

    // creating gRPC client from channel
    let mut client= TwoPhaseCommitServiceClient::new(channel);
    // creating a new Request
    let request = tonic::Request::new(
        PreparePhaseReq {
            version: "1".to_owned(),
            command: "vote?".to_owned(),
        }
    );

    // sending request and waiting for response
    let response = client.prepare(request).await?.into_inner();
    println!("RESPONSE={:?}", response);
    Ok(())
}

#[tokio::main]
pub async fn init_rpc_client(endpoint: &'static str) -> Result<TwoPhaseCommitServiceClient<Channel>, Box<dyn std::error::Error>> {
    let channel = tonic::transport::Channel::from_static(endpoint).connect().await?;
    let client= TwoPhaseCommitServiceClient::new(channel);
    Ok(client)
}

#[tokio::main]
pub async fn send_client(client: RefCell<TwoPhaseCommitServiceClient<Channel>>) -> Result<PreparePhaseResp, Box<dyn std::error::Error>> {
    let request = tonic::Request::new(
        PreparePhaseReq {
            version: "1".to_owned(),
            command: "vote?".to_owned(),
        }
    );

    // sending request and waiting for response
    let response: PreparePhaseResp = client.borrow_mut().prepare(request).await?.into_inner();
    println!("RESPONSE={:?}", response);
    Ok(response)
}