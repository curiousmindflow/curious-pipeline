use tokio::task::JoinHandle;

pub struct Event;

impl Event {
    pub fn from_iterator<I, C, A>(iterator: I, callback: C) -> JoinHandle<()>
    where
        I: Iterator<Item = A> + Send + 'static,
        C: FnOnce(A) -> () + Send + 'static + Copy,
    {
        println!("something");
        tokio::spawn(async move { iterator.for_each(|item| callback(item)) })
    }
}
