#![allow(dead_code)]

mod pipeline;
mod error;

pub use pipeline::{Pipeline, Pipe, PipelineBuilder};
pub use error::Error;