use std::sync::Arc;

use curious_server::{Error, Pipe, PipelineBuilder};

#[tokio::main]
pub async fn main() {
    let pipeline = PipelineBuilder::<Context>::init()
        .add_pipe(Box::new(ExamplePipe::new("Pipe_1".to_string())))
        .add_pipe(Box::new(ExamplePipe::new("Pipe_2".to_string())))
        .build()
        .unwrap();
    let pipeline = Arc::new(pipeline);

    let ctx = Context::new();

    let pipeline_clone = pipeline.clone();
    let front_handle = tokio::spawn(async move {
        let _ctx = pipeline_clone.process_forward(ctx).await.unwrap();
    });

    let ctx = Context::new();

    let pipeline_clone = pipeline.clone();
    let back_handle = tokio::spawn(async move {
        let _ctx = pipeline_clone.process_backward(ctx).await.unwrap();
    });

    front_handle.await.unwrap();
    back_handle.await.unwrap();
}

#[allow(dead_code)]
pub struct Context {
    var: String,
}

impl Context {
    pub fn new() -> Context {
        Context {
            var: "".to_string(),
        }
    }
}

#[allow(dead_code)]
pub struct ExamplePipe {
    name: String,
}

impl ExamplePipe {
    pub fn new(name: String) -> ExamplePipe {
        ExamplePipe { name }
    }
}

impl<T> Pipe<T> for ExamplePipe {
    fn forward(&self, _context: &mut T) -> Result<(), Error> {
        println!("From Pipe::forward()");
        Ok(())
    }

    fn backward(&self, _context: &mut T) -> Result<(), Error> {
        println!("From Pipe::backward()");
        Ok(())
    }
}
