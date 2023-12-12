
use participant::Participant;

#[derive(Debug)]
pub struct Coordinator {
    name: String,
    participants: Vec<Participant>
}

impl Coordinator {
    pub fn new(name: String) -> Coordinator {
        Coordinator {
            name: name, 
            participants: vec![],
        }
    }

    pub fn query_to_commit(&self){
        for p in self.participants.iter() {
            
        }
    }
    pub fn commit(&self, msg: &str){
        println!("{:?}", msg)
        self.query_to_commit()
    }
    pub fn register_participants(){}
}



