use std::io;

use thiserror::Error;

use crate::broker;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Std io")]
    IO {
        #[from]
        source: io::Error,
    },
    #[error("serde_json")]
    Serde {
        #[from]
        source: serde_json::Error,
    },
    #[error("Broker error")]
    BrokerError {
        #[from]
        source: broker::Error,
    },
}
