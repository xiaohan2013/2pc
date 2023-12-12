use std:Arc;

#[derive(Debug)]
pub struct OpLog {
    seqno: i32,
    log_arc: Arc<Mutex<HashMap<i32, message::ProtocolMessage>>>,
    path: String,
    lf: File,
}

impl OpLog {
    pub fn new(fpath: String) -> OpLog {}
}