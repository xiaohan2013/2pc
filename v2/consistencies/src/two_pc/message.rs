const VOTE_COMMIT: &'static str = "vote";
const VOTE_ABORT: &'static str = "abort";

#[derive(Debug, Clone, Copy)]
pub enum STATUS {
    LOCKED,
    UNLOCK,
    COMMITED,
}