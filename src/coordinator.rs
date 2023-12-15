
use std::borrow::BorrowMut;
use std::collections::HashMap;
use std::sync::mpsc::{channel, Sender, Receiver};
use std::rc::Rc;

use participant::Participant;

pub type CHANNEL<T> = (Sender<T>, Receiver<T>);

#[derive(Debug)]
pub struct Coordinator {
    name: String,
    participants: HashMap<String, Participant>,
    channels: Vec<Rc<CHANNEL<String>>>,
}

impl Coordinator {
    pub fn new(name: String) -> Self {
        Coordinator {
            name: name, 
            // participants: vec![],
            participants: HashMap::new(),
            channels: vec![],
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
        for c in self.channels.iter_mut() {
            c.0.send("commmmmit".to_string()).unwrap_or_else(|error| {
                panic!("{:?}", error)
            });
        }
        self.query_to_commit(msg.to_string());
    }

    pub fn register_participant(self: &mut Self, name: &str, log_dir: &str){
        let c = channel::<String>();
        let c_ref = Rc::new(c);
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



