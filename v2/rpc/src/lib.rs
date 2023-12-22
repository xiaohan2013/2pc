extern crate tokio;

pub mod two_phase_commit_common;
pub mod two_phase_commit_client;
pub mod two_phase_commit;
pub mod server;
pub mod client;

use tonic::{transport::Server, Request, Response, Status, Result};

use std::fmt::Debug;
#[tonic::async_trait]
pub trait RpcClient where Self : Debug + Send + Sync {
    async fn send_prepare_participant(&self, endpoint: &'static str) -> Result<(), Box<dyn std::error::Error>>;
}