use tonic::{transport::Server, Request, Response, Status};
use consistencies::two_pc::participant::Participant;
use consistencies::two_pc::coordinator::Coordinator;
use rpc::two_phase_commit::{PreparePhaseReq, PreparePhaseResp, CommitPhaseReq, CommitPhaseResp};
use rpc::two_phase_commit::two_phase_commit_service_server::{TwoPhaseCommitService, TwoPhaseCommitServiceServer};
use std::cell::RefCell;
use std::sync::Arc;
use std::sync::RwLock;
use std::sync::Mutex;


#[derive(Default, Debug)]
pub struct RpcCoordinator {
    pub c: Mutex<RefCell<Coordinator>>,
}

#[derive(Debug)]
pub struct RpcParticipant {
    pub p: Mutex<RefCell<Participant>>,
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
