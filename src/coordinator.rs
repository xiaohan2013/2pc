
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

    pub fn query_to_commit(&self){
        for (name, p) in self.participants.iter() {
            println!("{:?} => {}", name, p)
        }
    }
    pub fn commit(&self, msg: &str){
        println!("{:?}", msg);
        self.query_to_commit();
    }

    pub fn register_participant(&mut self, name: &str){
        let p = Participant::new(name.to_string());
        self.participants.insert(name.to_string(), p);
    }
}



