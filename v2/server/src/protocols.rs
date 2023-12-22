use tonic::{transport::Server, transport::Channel, Request, Response, Status, Result};
use consistencies::two_pc::participant::Participant;
use consistencies::two_pc::coordinator::Coordinator;
use rpc::two_phase_commit::{PreparePhaseReq, PreparePhaseResp, CommitPhaseReq, CommitPhaseResp};
use rpc::two_phase_commit::two_phase_commit_service_server::{TwoPhaseCommitService, TwoPhaseCommitServiceServer};
use std::cell::RefCell;
use std::sync::Arc;
use std::sync::RwLock;
use std::sync::Mutex;
use rpc::two_phase_commit_client::client_service_server::ClientService;
use rpc::two_phase_commit_client::{ClientSubmitReq, ClientSubmitResp};
use rpc::two_phase_commit_common::common_service_server::CommonService;
use rpc::two_phase_commit_common::{RegisterParticipantReq, RegisterParticipantResp, BaseReq, BaseResp};
use rpc::RpcClient;
use rpc::two_phase_commit_client::client_service_client::ClientServiceClient;
use std::collections::HashMap;
use futures::executor;
use timer;
use std::time::Duration;
use chrono;


#[derive(Default, Debug)]
pub struct RpcCoordinator {
    pub c: Mutex<RefCell<Coordinator>>,
}

#[derive(Debug)]
pub struct RpcParticipant {
    pub p: Mutex<RefCell<Participant>>,
}

#[tonic::async_trait]
impl RpcClient for RpcCoordinator{
    async fn send_prepare_participant(&self, endpoint: &'static str) -> Result<(), Box<dyn std::error::Error>> {
        // prepare rpc client
        tracing::info!("Start sending prepare request into Participant.");
        // let channel = Channel::from_static(endpoint)
        let _endpoint = endpoint.clone();
        let endpoint_obj = Channel::from_static(_endpoint);
        let channel = endpoint_obj.connect().await?;

        let mut _client = ClientServiceClient::new(channel);
        let mut data = HashMap::new();
        data.insert("ab".to_string(), "ddddddddddd".to_string());
        data.insert("bcd".to_string(), "232323".to_string());
        tracing::info!("Send sumit request to participant, {:?}", data);
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
        Ok(())
    }
    fn heart_pacemaker() { todo!(); }
}

#[tonic::async_trait]
impl RpcClient for RpcParticipant{
    async fn send_prepare_participant(&self, endpoint: &'static str) -> Result<(), Box<dyn std::error::Error>> {
        todo!();
    }
    fn heart_pacemaker() { 
        tracing::debug!("heart_pacemaker");
        let timer = timer::Timer::new();
        timer.schedule_repeating(chrono::Duration::milliseconds(5000), move || {
            tracing::debug!("heart beat!!!")
        });
    }
}

#[tonic::async_trait]
impl TwoPhaseCommitService for RpcParticipant {
    async fn prepare(&self, req: Request<PreparePhaseReq>) -> Result<Response<PreparePhaseResp>, Status> {
        let _command = &req.get_ref().command;
        let _version = &req.get_ref().version;
        println!("PreparePhaseReq: command: {}, version: {}", _command, _version);
        Ok(Response::new(PreparePhaseResp{
            version: "1".to_string(),
            ack: "YES".to_string()
        }))
    }
    async fn commit(&self, req: Request<CommitPhaseReq>) -> Result<Response<CommitPhaseResp>, Status> {
        Ok(Response::new(CommitPhaseResp { version: "1".to_owned() }))
    }
}

#[tonic::async_trait]
impl TwoPhaseCommitService for RpcCoordinator {
    async fn prepare(&self, req: Request<PreparePhaseReq>) -> Result<Response<PreparePhaseResp>, Status> {
        let _command = &req.get_ref().command;
        let _version = &req.get_ref().version;
        tracing::debug!("PreparePhaseReq: command: {}, version: {}", _command, _version);
        let mut _participants = {
            let c = self.c.try_lock().unwrap();
            // drop MutexGuard 
            let mut _c_ref = c.borrow_mut();
            _c_ref.pre_commit("aaaaaaaaaaaaaaaaaaa");
            tracing::debug!("{:?}", _c_ref);
            let _p = _c_ref.participants.clone();
            tracing::debug!("{:?}", _p);
            // std::mem::drop(c);
            _p
        };

        tracing::info!("Participants {:?}", _participants);
        for (name, host) in _participants {
            let _name = name;
            let _host = host.expect("failed to extract host from Option.");
            let _endpoint = format!("http://{}:50052", _host);
            let _endpoint_static_str: &'static str = Box::leak(_endpoint.into_boxed_str());
            self.send_prepare_participant(_endpoint_static_str).await.unwrap_or_else(|err| {
                panic!("{:?}", err)
            });
        }

        // let _ = executor::block_on(self.send_prepare_participant("http://127.0.0.1:50052"))
        // .unwrap_or_else(|err| {
        //     panic!("{:?}", err)
        // });

        Ok(Response::new(PreparePhaseResp{
            version: "1".to_string(),
            ack: "YES".to_string()
        }))
    }
    async fn commit(&self, req: Request<CommitPhaseReq>) -> Result<Response<CommitPhaseResp>, Status> {
        todo!();
    }
}

#[tonic::async_trait]
impl ClientService for RpcCoordinator {
    async fn submit(&self, req: Request<ClientSubmitReq>) -> Result<Response<ClientSubmitResp>, Status> {
        let _data = &req.get_ref().data;
        println!("{:?}", _data);
        {
            let c = self.c.try_lock().unwrap();
            // drop MutexGuard 
            c.borrow_mut().start_txn("aaaaaaaaaaaaaaaaaaa");
            // std::mem::drop(c);
        }
        Ok(Response::new(ClientSubmitResp {
             version: "1".to_owned() 
        }))
    }
}

#[tonic::async_trait]
impl ClientService for RpcParticipant {
    async fn submit(&self, req: Request<ClientSubmitReq>) -> Result<Response<ClientSubmitResp>, Status> {
        let _version = &req.get_ref().version;
        let _data = &req.get_ref().data;
        tracing::info!("{:?}", _data);
        {
            let c = self.p.try_lock().unwrap();
            // drop MutexGuard 
            c.borrow_mut().vote("participant submit".to_string());
            // std::mem::drop(c);
        }
        Ok(Response::new(ClientSubmitResp {
             version: "1".to_owned() 
        }))
    }
}


#[tonic::async_trait]
impl CommonService for RpcCoordinator {
    async fn register_participant(&self, req: Request<RegisterParticipantReq>) -> Result<Response<RegisterParticipantResp>, Status> {
        let _version = &req.get_ref().version;
        let _endpoint = &req.get_ref().endpoint;
        let _name = &req.get_ref().name;
        tracing::debug!("version: {}, name: {}, endpoint: {} ", _version, _name, _endpoint);
        {
            let c = self.c.try_lock().unwrap();
            // drop MutexGuard 
            let mut _c = c.borrow_mut();
            _c.register_participant(_name, _endpoint);
            // std::mem::drop(c);
            tracing::info!("{:?}", _c.participants)
        }

        Ok(Response::new(RegisterParticipantResp {
             version: "1".to_owned(),
             code: "0".to_string(),
             msg: "Success".to_string(),
        }))
    }

    async fn hearbeat(&self, req: Request<BaseReq>) -> Result<Response<BaseResp>, Status> {
        todo!();
    }
}
