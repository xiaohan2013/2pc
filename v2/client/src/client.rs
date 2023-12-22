#![allow(dead_code)]
#![allow(unused_imports)]

use tonic::{transport::Server, Request, Response, Status, Result};
use rpc::two_phase_commit::two_phase_commit_service_client::TwoPhaseCommitServiceClient;
use rpc::two_phase_commit::{PreparePhaseReq, PreparePhaseResp};
use clap::Parser;
use rpc::two_phase_commit_client::client_service_client::ClientServiceClient;
use rpc::two_phase_commit_client::{ClientSubmitReq, ClientSubmitResp};
use std::collections::HashMap;
use std::any::Any;
use rpc::two_phase_commit_common::common_service_client::CommonServiceClient;
use rpc::two_phase_commit_common::{RegisterParticipantReq, RegisterParticipantResp};
use std::io;
use local_ip_address::local_ip;
use local_ip_address::list_afinet_netifas;
use std::borrow::Borrow;
use std::concat;
use std::string::String;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    name: String,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}



#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let _name = args.name;

    // creating a channel ie connection to server
    let _local_ip = local_ip_address::local_ip()?;
    tracing::info!("local ip : {:?}", _local_ip);
    // let addr = concat!("http://", Box::leak(_local_ip.into_boxed_str()), ":50051");
    let __local_ip = String::from(_local_ip.to_string());
    let _addr = format!("http://{}:50051", __local_ip);
    let __addr = Box::leak(_addr.into_boxed_str());
    // let channel = tonic::transport::Channel::from_static("http://[::1]:50051")
    let channel = tonic::transport::Channel::from_static(__addr)
    .connect()
    .await?;

    match _name.as_str() {
        "client" => {
            // creating gRPC client from channel
            let mut _client = TwoPhaseCommitServiceClient::new(channel);
            // creating a new Request
            let _request = tonic::Request::new(
                PreparePhaseReq {
                    version: "1".to_owned(),
                    command: "vote?".to_owned(),
                }
            );
            let response = _client.prepare(_request).await?.into_inner();
            println!("RESPONSE={:?}", response);
        },
        "submit" => {
            // creating gRPC client from channel
            let mut _client = ClientServiceClient::new(channel);
            let mut data = HashMap::new();
            data.insert("ab".to_string(), "ddddddddddd".to_string());
            data.insert("bcd".to_string(), "232323".to_string());
            // creating a new Request
            let _request = tonic::Request::new(
                ClientSubmitReq {
                    version: "1".to_owned(),
                    data: data,
                }
            );
            let response = _client.submit(_request).await?.into_inner();
            // sending request and waiting for response
            println!("RESPONSE={:?}", response);
        },
        "regist" => {
            // creating gRPC client from channel
            let mut _client = CommonServiceClient::new(channel);
            let __ip = local_ip()?;
            let _ip = __ip.to_string();
            let __host = hostname::get()?;
            let _host = __host.into_string().unwrap();
            // creating a new Request
            let _request: Request<RegisterParticipantReq> = tonic::Request::new(
                RegisterParticipantReq {
                    version: "1".to_owned(),
                    name: _host,
                    endpoint: _ip,
                }
            );
            println!("{:?}", _request);
            let response = _client.register_participant(_request).await?.into_inner();
            // sending request and waiting for response
            println!("RESPONSE={:?}", response);
        }
        _ => panic!("ooops"),
    };

    Ok(())
}