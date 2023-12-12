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

use std::sync::{Arc, atomic::AtomicBool};

use coordinator::Coordinator;

#[derive(Debug)]
pub struct Participant {
    name: String,
}

fn register_participants(
    coordinator: &mut Coordinator,
    n_participants: i32,
    logpathbase: &String,
    running: &Arc<AtomicBool>, 
    success_prob: f64) -> Vec<Participant> {

    let participants = vec![];
    // TODO
    // register participants with coordinator (set up communication channels and sync objects)
    // add client to the vector and return the vector.
    
    participants
}

