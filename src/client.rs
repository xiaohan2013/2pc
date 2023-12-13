// mod coordinator;

use coordinator::Coordinator;

#[derive(Debug)]
pub struct Client {
    name: String,
}

impl Client{
    pub fn commit(coordinator: &mut Coordinator, msg: &str) {
        coordinator.commit(msg);
    }
}