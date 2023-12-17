extern crate tokio;

use two_phase_commit::prepare_phase_client::PreparePhaseClient;
use two_phase_commit::PreparePhaseReq;
mod two_phase_commit;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // creating a channel ie connection to server
    let channel = tonic::transport::Channel::from_static("http://[::1]:50051")
    .connect()
    .await?;

    // creating gRPC client from channel
    let mut client= PreparePhaseClient::new(channel);
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
