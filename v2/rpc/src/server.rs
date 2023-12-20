use tonic::{transport::Server, Request, Response, Status};
use super::two_phase_commit::{PreparePhaseReq, PreparePhaseResp, CommitPhaseReq, CommitPhaseResp};
use super::two_phase_commit::two_phase_commit_service_server::{TwoPhaseCommitService, TwoPhaseCommitServiceServer};

// defining a struct for our service
#[derive(Default, Debug)]
pub struct TwoPhaseCommitPrepare {}

// implementing rpc for service defined in .proto
#[tonic::async_trait]
impl TwoPhaseCommitService for TwoPhaseCommitPrepare {
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