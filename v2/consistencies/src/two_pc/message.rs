const VOTE_COMMIT: &'static str = "vote";
const VOTE_ABORT: &'static str = "abort";

#[derive(Debug)]
pub enum STATUS {
    LOCKED,
    UNLOCK,
    COMMITED,
}