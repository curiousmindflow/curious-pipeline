#[derive(Clone, Debug)]
pub struct Message {
    pub(crate) sender: usize,
    pub(crate) message: String,
}
