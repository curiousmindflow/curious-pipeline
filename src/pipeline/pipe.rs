use crate::error::Error;

pub trait Pipe<T> {
    fn forward(&self, context: &mut T) -> Result<(), Error>;
    fn backward(&self, context: &mut T) -> Result<(), Error>;
}
