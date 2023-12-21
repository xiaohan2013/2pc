extern crate docopt;
extern crate serde;
extern crate log4rs;
extern crate log;
extern crate ipc_channel;
extern crate rand;
extern crate uuid;
extern crate memmap2;
extern crate futures as futures_core;
extern crate tower;

mod message;
pub mod coordinator;
pub mod participant;