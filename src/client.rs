// mod coordinator;

use coordinator::Coordinator;

#[derive(Debug)]
pub struct Client {
    name: String,
}

impl Client {
    pub fn commit(coordinator: &Coordinator, msg: &str) {
        coordinator.commit(msg);
    }
}