use tokio::sync::broadcast::{channel, Receiver, Sender};

use super::Message;

pub struct SubscriberInfos {
    sender: Sender<Message>,
    receiver: Receiver<Message>,
}

impl SubscriberInfos {
    pub fn new() -> SubscriberInfos {
        let (sender, receiver) = channel(128);
        SubscriberInfos { sender, receiver }
    }

    pub fn get_receiver(&self) -> Receiver<Message> {
        self.sender.subscribe()
    }

    pub fn get_sender(&self) -> &Sender<Message> {
        &self.sender
    }
}
