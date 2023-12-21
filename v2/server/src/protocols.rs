use tonic::{transport::Server, Request, Response, Status};
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

#[derive(Default, Debug)]
pub struct RpcCoordinator {
    pub c: Mutex<RefCell<Coordinator>>,
}

#[derive(Debug)]
pub struct RpcParticipant {
    pub p: Mutex<RefCell<Participant>>,
}

impl RpcClient for RpcCoordinator{}
impl RpcClient for RpcParticipant{}

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
        println!("PreparePhaseReq: command: {}, version: {}", _command, _version);
        {
            let c = self.c.try_lock().unwrap();
            // drop MutexGuard 
            c.borrow_mut().pre_commit("aaaaaaaaaaaaaaaaaaa");
            // std::mem::drop(c);
        }
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
impl CommonService for RpcCoordinator {
    async fn register_participant(&self, req: Request<RegisterParticipantReq>) -> Result<Response<RegisterParticipantResp>, Status> {
        let _version = &req.get_ref().version;
        let _endpoint = &req.get_ref().endpoint;
        let _name = &req.get_ref().name;
        println!("version: {}, name: {}, endpoint: {} ", _version, _name, _endpoint);
        {
            let c = self.c.try_lock().unwrap();
            // drop MutexGuard 
            let mut _c = c.borrow_mut();

            _c.register_participant(_name, _endpoint);

            // std::mem::drop(c);
        }

        Ok(Response::new(RegisterParticipantResp {
             version: "1".to_owned(),
             code: "0".to_string(),
             msg: "Success".to_string(),
        }))
    }

    async fn hearbeat(&self, req: Request<BaseReq>) -> Result<Response<BaseResp>, Status> {
        Ok(Response::new(BaseResp {
             version: "1".to_owned(),
        }))
    }
}
