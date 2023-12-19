
#![allow(dead_code)]
#![allow(unused_imports)]

use tonic::{transport::Server, Request, Response, Status, Result};
use rpc::server::TwoPhaseCommitPrepare;
use rpc::two_phase_commit::two_phase_commit_service_server::TwoPhaseCommitServiceServer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    

    // defining address for our service
    // let addr: std::net::SocketAddr = "[::1]:50051".parse().unwrap();
    // let addr: std::net::SocketAddr = "127.0.0.1:9001".parse().unwrap();
    let addr: std::net::SocketAddr = "127.0.0.1:50051".parse().unwrap();
    
    // creating a service
    let prepare_phase = TwoPhaseCommitPrepare::default();
    println!("Rpc Server listening on {:?}", addr);
    // adding our service to our server.
    Server::builder()
        .add_service(TwoPhaseCommitServiceServer::new(prepare_phase))
        .serve(addr)
        .await?;

    Ok(())
}