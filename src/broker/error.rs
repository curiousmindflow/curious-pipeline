use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Subscriber id: {0} already taken")]
    SubscriberIdAlreadyTaken(usize),
    #[error("Subscriber not found with id: {0}")]
    SubscriberNotFound(usize),
}
