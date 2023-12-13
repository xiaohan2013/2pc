
use std::borrow::BorrowMut;
use std::collections::HashMap;
use std::sync::mpsc::{channel, Sender, Receiver};

use participant::Participant;

pub type CHANNEL<T> = (Sender<T>, Receiver<T>);

#[derive(Debug)]
pub struct Coordinator<'a> {
    name: String,
    participants: HashMap<String, Participant<'a>>,
    channel: CHANNEL<String>,
}

impl<'a> Coordinator<'a> {
    pub fn new(name: String) -> Coordinator<'a> {
        Coordinator {
            name: name, 
            // participants: vec![],
            participants: HashMap::new(),
            channel: channel::<String>(),
        }
    }

    pub fn query_to_commit(&mut self, msg: String){
        for (name, p) in self.participants.iter_mut() {
            println!("{:?} => {}", name, p);
            p.vote(msg.to_string());
        }
    }
    pub fn commit(&mut self, msg: &str){
        println!("{:?}", msg);
        self.query_to_commit(msg.to_string());
    }

    pub fn register_participant(&'a mut self, name: &str, log_dir: &str){
        let p = Participant::new(name.to_string(), log_dir.to_string(), self.channel.1.borrow_mut());
        if !self.participants.contains_key(name) {
            self.participants.insert(name.to_string(), p);
        // } else {
        //     println!("Participant {:} has been existed!!!", name)
        }
    }
}



