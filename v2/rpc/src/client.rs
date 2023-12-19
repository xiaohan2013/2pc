use tower::timeout::Timeout;
use tonic::transport::Channel;

use super::two_phase_commit::two_phase_commit_service_client::TwoPhaseCommitServiceClient;
use super::two_phase_commit::{PreparePhaseReq, PreparePhaseResp};

use std::cell::RefCell;
use std::sync::Arc;
use std::time::Duration;
use tokio::runtime::Builder;

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
pub async fn init_rpc_client1(endpoint: &'static str) -> Result<TwoPhaseCommitServiceClient<tower::timeout::Timeout<Channel>>, Box<dyn std::error::Error>> {
    let channel = tonic::transport::Channel::from_static(endpoint).connect().await?;
    let time_channel = Timeout::new(channel, Duration::from_millis(1000));
    let client= TwoPhaseCommitServiceClient::new(time_channel);
    Ok(client)
}

pub async fn init_rpc_client(endpoint: &'static str) -> Result<TwoPhaseCommitServiceClient<Channel>, Box<dyn std::error::Error>>{
    let channel = tonic::transport::Channel::from_static(endpoint)
            .connect()
            .await?;
    
    let client= TwoPhaseCommitServiceClient::new(channel);
    Ok(client)
}

pub fn init_rpc_client_sync() -> Result<TwoPhaseCommitServiceClient<Channel>, tonic::transport::Error> {
    let rt = Builder::new_multi_thread().enable_all().build().unwrap();
    let client = rt.block_on(TwoPhaseCommitServiceClient::connect("http://[::1]:50051"))?;
    Ok(client)
}

// #[tokio::main]
// pub async fn send_client(client: RefCell<TwoPhaseCommitServiceClient<tower::timeout::Timeout<Channel>>>) -> Result<PreparePhaseResp, Box<dyn std::error::Error>> {
//     let req = tonic::Request::new(
//         PreparePhaseReq {
//             version: "1".to_owned(),
//             command: "vote?".to_owned(),
//         }
//     );

//     // sending request and waiting for response
//     let response: PreparePhaseResp = client.borrow_mut().prepare(req).await?.into_inner();
//     println!("RESPONSE={:?}", response);
//     Ok(response)
// }

pub async fn send_client(client: RefCell<TwoPhaseCommitServiceClient<Channel>>) {
    let req = tonic::Request::new(
        PreparePhaseReq {
            version: "1".to_owned(),
            command: "vote?".to_owned(),
        }
    );
    println!("send_client: {:?}", client);
    // sending request and waiting for response
    let resp = client.borrow_mut().prepare(req).await;
    println!("RESPONSE={:?}", resp);
    // Ok(())
}

