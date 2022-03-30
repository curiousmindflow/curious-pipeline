use crate::{Error, Pipeline};

use super::PipeDyn;

pub struct PipelineBuilder<T> {
    middlewares: Vec<PipeDyn<T>>,
}

impl<T> PipelineBuilder<T>
where
    T: Send,
{
    pub fn init() -> PipelineBuilder<T> {
        PipelineBuilder {
            middlewares: vec![],
        }
    }

    pub fn add_pipe(mut self, pipe: PipeDyn<T>) -> PipelineBuilder<T> {
        self.middlewares.push(pipe);
        self
    }

    pub fn build(self) -> Result<Pipeline<T>, Error> {
        let pipeline = Pipeline {
            middlewares: self.middlewares,
        };
        Ok(pipeline)
    }
}
