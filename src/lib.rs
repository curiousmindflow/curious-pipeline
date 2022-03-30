#![allow(dead_code)]

mod error;
mod pipeline;
mod socket;

pub use error::Error;
pub use pipeline::{Pipe, Pipeline, PipelineBuilder};
