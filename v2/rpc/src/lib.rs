extern crate tokio;

pub mod two_phase_commit_common;
pub mod two_phase_commit_client;
pub mod two_phase_commit;
pub mod server;
pub mod client;
use std::any::Any;

use std::fmt::Debug;
pub trait RpcClient where Self : Debug + Send + Sync {}