use crate::error::Error;

use super::pipe::Pipe;

pub struct Pipeline<T> {
    middlewares: Vec<Box<dyn Pipe<T>>>
}

impl<T> Pipeline<T> {
    pub async fn process(&self, context: T) -> Result<T, Error> {
        // match 
        unimplemented!()
    }
}
