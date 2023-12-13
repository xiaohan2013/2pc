extern crate docopt;
extern crate serde;
extern crate log4rs;
extern crate log;
extern crate ipc_channel;
extern crate rand;
extern crate uuid;

use docopt::Docopt;
use serde::Deserialize;
use log::{error, info, warn};
use ipc_channel::platform;
use std::thread;

mod coordinator;
mod participant;
mod client;

const USAGE: &'static str = "
Two Phase Commit.

Usage:
  two_phase_commit [-l | --m=<mode> | -c | -p | -r | -s ]
  two_phase_commit (-h | --help)
  two_phase_commit (-V | --version)

Options:
    -h, --help       Prints help information
    -V, --version    Prints version information
    -l               Specifies path to directory where logs are stored
    -m               Mode: \"run\" starts 2PC, 
                           \"client\" starts a client process, 
                           \"participant\" starts a participant process, 
                           \"check\" checks logs produced by previous run
    -c               Number of clients making requests
    -p               Number of participants in protocol
    -r               Number of requests made per client
    -s               Probability participants successfully execute requests
    -S               Probability participants successfully send messages
    -v               Output verbosity: 0->No Output, 5->Output Everything
     --ipc-path      Path for IPC socket for communication
     --num           Participant / Client number for naming the log files. Ranges from 0 to num_clients - 1 or num_participants - 1
";

#[derive(Debug, Deserialize)]
struct Args {
    flag_help: Option<bool>,
    flag_version: Option<bool>,
    flag_l: Option<String>,
    arg_mode: Option<String>,
    flag_c: Option<u8>,
    flag_p: Option<u8>,
    flag_r: Option<u8>,
    flag_s: Option<String>,
    flag_S: Option<String>,
    flag_v: Option<bool>,
    flag_ipc_path: Option<String>,
    flag_num: Option<u8>
}

fn main() {
    // let argv = || vec!["two_phase_commit.exe", "-s", ".95", "-c", "4", "-p", "10", "-r", "10", "-m", "run"];
    let version = env!("CARGO_PKG_NAME").to_string() + ", version: " + env!("CARGO_PKG_VERSION");
    println!("======>{:?}", version);
    log4rs::init_file("config/log4rs.yaml", Default::default()).unwrap();
    info!("booting up");
    // let args: Args = Docopt::new(USAGE)
    //     .and_then(|d| {
    //         println!("{:?}", d);
    //         d.deserialize()
    //     })
    //     .unwrap_or_else(|e| e.exit());

    // println!("{:?}", args);
    // println!("{:?}", args.arg_mode)
    // let (tx, rx) = platform::channel().unwrap();
    
    // thread::spawn(move || {
    //     let val = "hi".as_bytes();
    //     tx.send(&val, vec![], vec![]).unwrap();
    // });

    // let (rev, _, _) = rx.recv().unwrap();
    // let res = String::from_utf8(rev).expect("Found invalid UTF-8");
    // println!("Got: {}", res)
    // register/launch clients, participants, coordinator
    let mut c1 = coordinator::Coordinator::new("main".to_string());
    let log_dir = "log/";
    c1.register_participant("a", log_dir);
    client::Client::commit(&mut c1, "commit");
}
