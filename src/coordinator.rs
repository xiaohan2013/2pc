
use std::collections::HashMap;

use participant::Participant;


#[derive(Debug)]
pub struct Coordinator {
    name: String,
    participants: HashMap<String, Participant>
}

impl Coordinator {
    pub fn new(name: String) -> Coordinator {
        Coordinator {
            name: name, 
            // participants: vec![],
            participants: HashMap::new(),
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

    pub fn register_participant(&mut self, name: &str, log_dir: &str){
        let p = Participant::new(name.to_string(), log_dir.to_string());
        self.participants.insert(name.to_string(), p);
    }
}



