mod pipe;
mod pipeline;
mod pipeline_builder;

pub use pipe::Pipe;
pub use pipeline::Pipeline;
pub use pipeline_builder::PipelineBuilder;

type PipeDyn<T> = Box<dyn Pipe<T> + Send + Sync>;
