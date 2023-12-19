
#![allow(dead_code)]
#![allow(unused_imports)]

use futures::{future, FutureExt};
use std::time::Duration;
use crossbeam_channel::{bounded, tick, Receiver, select};
use tonic::{transport::Server, Request, Response, Status, Result};
use rpc::server::TwoPhaseCommitPrepare;
use rpc::two_phase_commit::two_phase_commit_service_server::TwoPhaseCommitServiceServer;

fn ctrl_channel() -> Result<Receiver<()>, ctrlc::Error> {
    let (sender, receiver) = bounded(100);
    ctrlc::set_handler(move || {
        let _ = sender.send(());
    })?;
    Ok(receiver)
}

async fn _grpc_server() -> std::io::Result<()>{
    // defining address for our service
    // let addr: std::net::SocketAddr = "[::1]:50051".parse().unwrap();
    // let addr: std::net::SocketAddr = "127.0.0.1:9001".parse().unwrap();
    let addr: std::net::SocketAddr = "127.0.0.1:50051".parse().unwrap();

    tokio::spawn(async move {
        // creating a service
        let prepare_phase = TwoPhaseCommitPrepare::default();
        println!("Rpc Server listening on {:?}", addr);
        // adding our service to our server.
        Server::builder()
            .add_service(TwoPhaseCommitServiceServer::new(prepare_phase))
            .serve(addr)
            .await
            .expect("msg");
    });

    Ok(())
}

async fn _grpc_server_2() -> std::io::Result<()>{
    // defining address for our service
    // let addr: std::net::SocketAddr = "[::1]:50051".parse().unwrap();
    // let addr: std::net::SocketAddr = "127.0.0.1:9001".parse().unwrap();
    let addr: std::net::SocketAddr = "127.0.0.1:50052".parse().unwrap();

    tokio::spawn(async move {
        // creating a service
        let prepare_phase = TwoPhaseCommitPrepare::default();
        println!("Rpc Server listening on {:?}", addr);
        // adding our service to our server.
        Server::builder()
            .add_service(TwoPhaseCommitServiceServer::new(prepare_phase))
            .serve(addr)
            .await
            .expect("msg");
    });

    Ok(())
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut servers = vec![];
    servers.push(_grpc_server().boxed());
    servers.push(_grpc_server_2().boxed());

    if servers.is_empty() {
        panic!("can not find a valid server setup in conf!");
    }
    future::try_join_all(servers).await?;

    let ctrl_c_events= ctrl_channel()?;
    let ticks = tick(Duration::from_secs(5));
    loop {
        select! {
            recv(ticks) -> _ => {
                println!("working!");
            }
            recv(ctrl_c_events) -> _ => {
                println!();
                println!("Goodbye!");
                break;
            }
        }
    }
    Ok(())
}