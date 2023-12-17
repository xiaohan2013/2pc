extern crate tokio;

use tonic::{transport::Server, Request, Response, Status};
use two_phase_commit::{PreparePhaseReq, PreparePhaseResp};
use two_phase_commit::prepare_phase_server::{PreparePhase, PreparePhaseServer};
mod two_phase_commit; 

// defining a struct for our service
#[derive(Default)]
pub struct TwoPhaseCommitPrepare {}

// implementing rpc for service defined in .proto
#[tonic::async_trait]
impl PreparePhase for TwoPhaseCommitPrepare {
    async fn prepare(&self, req: Request<PreparePhaseReq>) -> Result<Response<PreparePhaseResp>, Status> {
        let _command = &req.get_ref().command;
        let _version = &req.get_ref().version;
        Ok(Response::new(PreparePhaseResp{
            version: "1".to_string(),
            ack: "YES".to_string()
        }))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // defining address for our service
    let addr = "[::1]:50051".parse().unwrap();
    // creating a service
    let prepare_phase = TwoPhaseCommitPrepare::default();
    println!("Server listening on {}", addr);
    // adding our service to our server.
    Server::builder()
        .add_service(PreparePhaseServer::new(prepare_phase))
        .serve(addr)
        .await?;
    Ok(())
}
