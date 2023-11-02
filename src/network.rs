use crate::message::Message;
use crossbeam::channel::{Receiver, Sender};

pub struct NodeChannel {
    pub sender: Sender<Message>,
    pub receiver: Receiver<Message>,
}
