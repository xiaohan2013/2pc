
#![allow(dead_code)]
#![allow(unused_imports)]

use futures::{future, FutureExt};
use std::time::Duration;
use crossbeam_channel::{bounded, tick, Receiver, select};
use tonic::{transport::Server, Request, Response, Status, Result};
use rpc::server::TwoPhaseCommitPrepare;
use rpc::two_phase_commit::two_phase_commit_service_server::TwoPhaseCommitServiceServer;
use consistencies::two_pc::coordinator::{self, Coordinator};
use config::{File, FileFormat, Config};
use std::rc::Rc;
use std::sync::Arc;
use std::cell::RefCell;
use std::sync::RwLock;
use std::sync::Mutex;

mod protocols;
use protocols::RpcCoordinator;


fn ctrl_channel() -> Result<Receiver<()>, ctrlc::Error> {
    let (sender, receiver) = bounded(100);
    ctrlc::set_handler(move || {
        let _ = sender.send(());
    })?;
    Ok(receiver)
}

async fn _grpc_server(c: Arc<RpcCoordinator>) -> std::io::Result<()>{
    // defining address for our service
    // let addr: std::net::SocketAddr = "[::1]:50051".parse().unwrap();
    // let addr: std::net::SocketAddr = "127.0.0.1:9001".parse().unwrap();
    let addr: std::net::SocketAddr = "127.0.0.1:50051".parse().unwrap();
    tracing::info!(message = "Starting server.", %addr);

    tokio::spawn(async move {
        // creating a service
        // let coordinator = Coordinator::default();
        // let prepare_phase = TwoPhaseCommitPrepare::default();
        println!("Rpc Server listening on {:?}", addr);
        // adding our service to our server.
        Server::builder()
            .trace_fn(|_| tracing::info_span!("helloworld_server"))
            .add_service(TwoPhaseCommitServiceServer::from_arc(c))
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
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let builder = Config::builder().add_source(File::new("config/conf.yaml", FileFormat::Yaml));
    let config = match builder.build() {
        Ok(config) => config,
        Err(err) => panic!("{:?}", err)
    };
    print!("{:?}", config.get_string("env")?);
    println!("{:?}", config.get_array("host")?);

    let c = RpcCoordinator{
        c: Mutex::new(RefCell::new(Coordinator::default()))
    };

    let mut servers = vec![];
    servers.push(_grpc_server(Arc::new(c)).boxed());
    // servers.push(_grpc_server_2().boxed());

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