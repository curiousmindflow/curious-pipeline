use crate::error::Error;

pub trait Pipe<T> {
    fn forward(&self, context: T) -> Result<T, Error>;
    fn backward(&self, context: T) -> Result<T, Error>;
}
