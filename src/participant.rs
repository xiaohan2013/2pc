/// 
/// register_participants()
/// 
/// The coordinator needs to know about all participants. 
/// This function should create participants and use some communication 
/// primitive to ensure the coordinator and participants are aware of 
/// each other and able to exchange messages. Starting threads to run the
/// participant protocol should be deferred until after all the communication 
/// structures are created. 
/// 
/// HINT: you probably want to look at rust's mpsc::channel or crossbeam 
///       channels to set up communication. Note that communication in 2PC 
///       is duplex!
/// 
/// HINT: read the logpathbase documentation carefully.
/// 
/// <params>
///     coordinator: the coordinator!
///     n_participants: number of participants to create an register
///     logpathbase: each participant, client, and the coordinator 
///         needs to maintain its own operation and commit log. 
///         The project checker assumes a specific directory structure 
///         for files backing these logs. Concretely, participant log files 
///         will be expected to be produced in:
///            logpathbase/participant_<num>.log
///     running: atomic bool indicating whether the simulation is still running
///     success_prob: [0.0..1.0] probability that operations or sends succeed.
///
/// 
/// 

use rand::Rng;
use std::sync::{Arc, atomic::AtomicBool};
use std::fmt::{Display, Formatter, Result};
use std::fs::File;
use std::io::{Write, ErrorKind};
use std::sync::mpsc::{channel, Sender, Receiver};
use std::cell::{RefCell, RefMut};
use std::rc::Rc;
use std::thread::{self, JoinHandle};

use super::coordinator::Coordinator;

use crate::coordinator::CHANNEL;

#[derive(Debug)]
pub struct Participant {
    name: String,
    success_prob: f64,
    log_file: File,
    channel: Rc<CHANNEL<String>>,
    worker: Option<JoinHandle<()>>,
}

impl Participant {
    pub fn new(name: String, log_file: String, receiver: Rc<CHANNEL<String>>) -> Self {
        let mut rng = rand::thread_rng();
        let log_file_name = format!("{}participant_{}.log", log_file, name);
        let c_logfile_name = &log_file_name;
        let mut log_file = File::options().append(true).open(c_logfile_name).unwrap_or_else(|error| {
            if error.kind() == ErrorKind::NotFound {
                File::create(c_logfile_name).unwrap_or_else(|error| {
                    panic!("Problem creating the file: {:?}", error);
                })
            } else {
                panic!("Problem opening the file: {:?}", error);
            }
        });

        Participant {
            name: name,
            success_prob: rng.gen(),
            log_file: log_file,
            channel: receiver,
            worker: None,
        }
    }

    pub fn append_log(&mut self, msg: String) {
        // writeln!(self.log_file, ).unwrap_or_else(|error| {
        //     panic!("Append log Error {:?}", error);
        // });
        self.log_file.write_all(msg.as_bytes()).unwrap_or_else(|error| {
            panic!("Append log Error {:?}", error);
        });
    }

    pub fn vote(&mut self, msg: String) {
        let res = self.channel.1.recv().unwrap_or_else(|error| {
            panic!("{:?}", error)
        });
        println!(">>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>> {:?}", res);
        self.append_log(msg);
        self.start();
    }

    pub fn run(self: &mut Self) {
        let mut handler = thread::spawn(move || {
            println!("this is thread number {}", "abc".to_string());
        });
        self.worker = Some(handler);
    }

    pub fn start(self: &mut Self) {
        // Start all threads
        self.worker
            .take().expect("Called stop on non-running thread")
            .join().expect("Could not join spawned thread");
    }
}

impl Display for Participant {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, " Participant[name:{}, success_prob: {:.2}], log_file: {:?}", self.name, self.success_prob, self.log_file)
    }
}


