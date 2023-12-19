#![allow(dead_code)]
#![allow(unused_imports)]

use tonic::{transport::Server, Request, Response, Status, Result};
use rpc::two_phase_commit::two_phase_commit_service_client::TwoPhaseCommitServiceClient;
use rpc::two_phase_commit::{PreparePhaseReq, PreparePhaseResp};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // creating a channel ie connection to server
    // let channel = tonic::transport::Channel::from_static("http://[::1]:50051")
    let channel = tonic::transport::Channel::from_static("http://127.0.0.1:50051")
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