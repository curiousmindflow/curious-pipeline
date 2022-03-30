use crate::error::Error;

use super::PipeDyn;

pub struct Pipeline<T>
where
    T: Send,
{
    pub(in crate::pipeline) middlewares: Vec<PipeDyn<T>>,
}

impl<T> Pipeline<T>
where
    T: Send,
{
    pub async fn process_forward(&self, mut ctx: T) -> Result<T, Error> {
        for midd in self.middlewares.iter() {
            midd.forward(&mut ctx)?
        }
        Ok(ctx)
    }

    pub async fn process_backward(&self, mut ctx: T) -> Result<T, Error> {
        for midd in self.middlewares.iter().rev() {
            midd.backward(&mut ctx)?
        }
        Ok(ctx)
    }
}
