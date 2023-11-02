use crate::message::Message;
use crate::network::NodeChannel;
use crate::node::{Node, NodeId};
use crossbeam::channel::{bounded, RecvTimeoutError};
use std::collections::{HashMap, HashSet};
use std::thread;
use std::time::Duration;

mod message;
mod network;
mod node;
mod role;
mod term;

fn main() {
    let num_nodes = 100;

    let nodes: HashSet<NodeId> = (0..num_nodes).into_iter().collect();

    let mut node_channels = HashMap::new();

    let (node_sender, hub_receiver) = bounded(100);

    for &node_id in &nodes {
        let (hub_sender, node_receiver) = bounded(100);

        node_channels.insert(
            node_id,
            NodeChannel {
                sender: hub_sender,
                receiver: hub_receiver.clone(),
            },
        );

        let nodes = nodes.clone();
        let node_sender = node_sender.clone();
        thread::spawn(move || {
            let mut node = Node::new(node_id, nodes.clone(), node_sender, node_receiver);
            loop {
                node.tick();
            }
        });
    }

    let receive_message = |node_id: NodeId,
                           node_channels: &HashMap<NodeId, NodeChannel>|
     -> Result<(), RecvTimeoutError> {
        match node_channels
            .get(&node_id)
            .unwrap()
            .receiver
            .recv_timeout(Duration::from_millis(10))
        {
            Ok(message) => forward_message(message, &node_channels),
            Err(err) => match err {
                RecvTimeoutError::Timeout => {}
                RecvTimeoutError::Disconnected => return Err(err),
            },
        };
        Ok(())
    };

    fn forward_message(message: Message, node_channels: &HashMap<NodeId, NodeChannel>) {
        match message {
            Message::Vote(ref vote) => node_channels.get(&vote.votee).unwrap().sender.send(message),
            Message::Heartbeat(ref heartbeat) => node_channels
                .get(&heartbeat.receiver)
                .unwrap()
                .sender
                .send(message),
            Message::RequestVote(ref request) => node_channels
                .get(&request.requestee)
                .unwrap()
                .sender
                .send(message),
        }
        .unwrap();
    }

    loop {
        for &node_id in &nodes {
            receive_message(node_id, &node_channels).unwrap();
        }
    }
}
