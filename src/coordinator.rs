
use std::borrow::BorrowMut;
use std::collections::HashMap;
use std::sync::mpsc::{channel, Sender, Receiver};
use std::rc::Rc;
use std::sync::Arc;
use std::cell::RefCell;
use std::sync::atomic::{self, AtomicBool, Ordering};


// use participant::Participant;
use crate::participant::Participant;
use crate::rpc;
use std::thread;
use std::time::Duration;
use rpc::two_phase_commit::two_phase_commit_service_client::TwoPhaseCommitServiceClient;
use rpc::two_phase_commit::PreparePhaseReq;
use tokio::sync::futures;
use tonic::transport::Channel;
use futures_core::executor; 

pub type CHANNEL<T> = (Sender<T>, Receiver<T>);

#[derive(Debug)]
pub struct Coordinator {
    name: String,
    participants: HashMap<String, Participant>,
    channels: Vec<Arc<CHANNEL<String>>>,
    rpc_client: Option<RefCell<TwoPhaseCommitServiceClient<Channel>>>,
}

impl Coordinator {
    pub fn new(name: String) -> Self {
        Coordinator {
            name: name, 
            // participants: vec![],
            participants: HashMap::new(),
            channels: vec![],
            rpc_client: None,
        }
    }

    pub fn init_rpc(&mut self) {
        println!("Initalize rpc server...");
        let server_running = Arc::new(AtomicBool::new(false));
        let _server_running = server_running.clone();
        thread::spawn(move || {
            println!("Starting Rpc Server");
            let _ = rpc::server::init_rpc_server();
            // let mut res = executor::block_on().into();
            // println!("{}", res.into())
            // _server_running.store(true, atomic::Ordering::SeqCst);
        });

        // wait server starting
        // while !server_running.load(Ordering::SeqCst) {}
        println!("Initalize rpc client...");
        // let _self = Arc::new(self);
        let _rpc_client = rpc::client::init_rpc_client("http://[::1]:50051");
        let rpc_client = _rpc_client.unwrap_or_else(|error| {
            panic!("initialize rpc client error, {}", error)
        });
        self.rpc_client = Some(RefCell::new(rpc_client));
        
    }

    pub fn query_to_commit(&mut self, msg: String){
        for (name, p) in self.participants.iter_mut() {
            println!("{:?} => {}", name, p);
            p.vote(msg.to_string());
            let _rpc_client = self.rpc_client.take().expect("msg");
            println!("Executing send query msg to rpc-client");
            let _ = rpc::client::send_client(_rpc_client);
        }
    }

    pub fn commit(&mut self, msg: &str){
        println!("{:?}", msg);
        for c in self.channels.iter_mut() {
            c.0.send("commmmmit".to_string()).unwrap_or_else(|error| {
                panic!("{:?}", error)
            });
        }
        self.query_to_commit(msg.to_string());
    }

    pub fn register_participant(self: &mut Self, name: &str, log_dir: &str){
        let c = channel::<String>();
        let c_ref = Arc::new(c);
        let c_ref2 = c_ref.clone();
        let mut p = Participant::new(name.to_string(), log_dir.to_string(),  c_ref);
        p.run();
        if !self.participants.contains_key(name) {
            self.participants.insert(name.to_string(), p);
        } else {
            println!("Participant {:} has been existed!!!", name)
        }
        self.channels.push(c_ref2);
    }
}



