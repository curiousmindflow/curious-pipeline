use std::collections::HashMap;

use tokio::sync::broadcast::Receiver;

use super::Error;

use super::{subscriber_infos::SubscriberInfos, Message};

pub struct Broker {
    subscribers: HashMap<usize, SubscriberInfos>,
}

impl Broker {
    pub fn new() -> Broker {
        Broker {
            subscribers: Default::default(),
        }
    }

    pub async fn register(&mut self, id: usize) -> Result<Receiver<Message>, Error> {
        if self.subscribers.contains_key(&id) {
            return Err(Error::SubscriberIdAlreadyTaken(id));
        }
        let sub_infos = SubscriberInfos::new();
        let receiver = sub_infos.get_receiver();
        self.subscribers.insert(id, sub_infos).unwrap();
        Ok(receiver)
    }

    pub async fn unregister(&mut self, id: usize) -> Result<(), Error> {
        if self.subscribers.remove(&id).is_none() {
            return Err(Error::SubscriberNotFound(id));
        }
        Ok(())
    }

    pub async fn send(
        &self,
        sender: usize,
        message: String,
        recipients: &[usize],
    ) -> Result<(), Error> {
        let message = Message { sender, message };
        for id in recipients.iter() {
            match self.subscribers.get(&id) {
                Some(recipient_sub_infos) => {
                    recipient_sub_infos
                        .get_sender()
                        .send(message.clone())
                        .unwrap();
                }
                None => {}
            }
        }

        Ok(())
    }
}
