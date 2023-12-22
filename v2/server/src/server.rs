
#![allow(dead_code)]
#![allow(unused_imports)]

use futures::{future, FutureExt};
use std::time::Duration;
use crossbeam_channel::{bounded, tick, Receiver, select};
use tonic::{transport::Server, Request, Response, Status, Result};
use rpc::server::TwoPhaseCommitPrepare;
use rpc::two_phase_commit::two_phase_commit_service_server::TwoPhaseCommitServiceServer;
use rpc::two_phase_commit_client::client_service_server::ClientServiceServer;
use rpc::two_phase_commit_common::common_service_server::CommonServiceServer;
use consistencies::two_pc::coordinator::{self, Coordinator};
use consistencies::two_pc::participant::{self, Participant};
use config::{File, FileFormat, Config};
use std::rc::Rc;
use std::sync::Arc;
use std::cell::RefCell;
use std::sync::RwLock;
use std::sync::Mutex;
use clap::Parser;
use local_ip_address::local_ip;

mod protocols;
use protocols::{RpcCoordinator, RpcParticipant};

// macro_rules! unwrap_or_return {
//     ( $e:expr ) => {
//         match $e {
//             Ok(x) => x,
//             Err(_) => (),
//         }
//     }
// }


fn ctrl_channel() -> Result<Receiver<()>, ctrlc::Error> {
    let (sender, receiver) = bounded(100);
    ctrlc::set_handler(move || {
        let _ = sender.send(());
    })?;
    Ok(receiver)
}

async fn _grpc_server_coordinator(c: Arc<RpcCoordinator>) -> std::io::Result<()>{
    // defining address for our service
    // let addr: std::net::SocketAddr = "[::1]:50051".parse().unwrap();
    // let addr: std::net::SocketAddr = "127.0.0.1:9001".parse().unwrap();
    let _local_ip = local_ip_address::local_ip().unwrap();
    tracing::info!("local ip : {:?}", _local_ip);
    let addr: std::net::SocketAddr = format!("{:?}:50051", _local_ip).parse().unwrap();
    tracing::info!(message = "Starting server.", %addr);

    let _c1 = c.clone();
    let _c2 = c.clone();

    tokio::spawn(async move {
        // creating a service
        // let coordinator = Coordinator::default();
        // let prepare_phase = TwoPhaseCommitPrepare::default();
        println!("Rpc Server listening on {:?}", addr);
        // adding our service to our server.
        Server::builder()
            .trace_fn(|_| tracing::info_span!("GRPC Server ===> Coordinator"))
            .add_service(TwoPhaseCommitServiceServer::from_arc(c))
            .add_service(ClientServiceServer::from_arc(_c1))
            .add_service(CommonServiceServer::from_arc(_c2))
            .serve(addr)
            .await
            .expect("msg");
    });

    Ok(())
}

async fn _grpc_server_participant(p: Arc<RpcParticipant>) -> std::io::Result<()>{
    // defining address for our service
    // let addr: std::net::SocketAddr = "[::1]:50051".parse().unwrap();
    // let addr: std::net::SocketAddr = "127.0.0.1:9001".parse().unwrap();
    let _local_ip = local_ip_address::local_ip().unwrap();
    tracing::info!("local ip : {:?}", _local_ip);
    let addr: std::net::SocketAddr = format!("{:?}:50052", _local_ip).parse().unwrap();
    tracing::info!(message = "Starting particpant GRPC server.", %addr);
    let _p = p.clone();

    tokio::spawn(async move {
        // creating a service
        // let prepare_phase = TwoPhaseCommitPrepare::default();
        println!("Participant Rpc Server listening on {:?}", addr);
        
        // adding our service to our server.
        Server::builder()
            .trace_fn(|_| tracing::info_span!("GRPC Server ===> Participant"))
            .add_service(TwoPhaseCommitServiceServer::from_arc(p))
            .add_service(ClientServiceServer::from_arc(_p))
            .serve(addr)
            .await
            .expect("msg");
    });

    Ok(())
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    role: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let __role = args.role;

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let builder = Config::builder().add_source(File::new("config/conf.yaml", FileFormat::Yaml));
    let config = match builder.build() {
        Ok(config) => config,
        Err(err) => panic!("{:?}", err)
    };
    tracing::info!("{:?}", config.get_string("env")?);
    tracing::info!("{:?}", config.get_array("host")?);

    let mut servers = vec![];
    let _role = __role.as_str();
    if _role == "coordinator" {
        let c = RpcCoordinator{
            c: Mutex::new(RefCell::new(Coordinator::default()))
        };
        servers.push(_grpc_server_coordinator(Arc::new(c)).boxed());
    }

    if _role == "participant" {
        let p = RpcParticipant{
            p: Mutex::new(RefCell::new(Participant::default()))
        };
        servers.push(_grpc_server_participant(Arc::new(p)).boxed());
    }

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