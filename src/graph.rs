//! Handles the graph logic, managing nodes and edges.

use petgraph::{
    stable_graph::{EdgeIndex, NodeIndex, StableGraph},
    Directed,
};

use async_trait::async_trait;
use smol_str::SmolStr;

use tokio::{
    sync::{mpsc, oneshot, watch},
    task,
};

pub mod data_types;

pub enum NodeCtrl {
    SetChannel(SmolStr, watch::Receiver<data_types::Data>),
    GetChannel(SmolStr, oneshot::Sender<watch::Receiver<data_types::Data>>),
    Shutdown,
}

/**
Common trait for all nodes in the graph.

All nodes in the graph must implement Node.
 */
#[async_trait]
pub trait Node: Sync {
    async fn run(&self, cmd_chan: mpsc::Receiver<NodeCtrl>) {}
}

pub struct NodeContainer {
    node: Box<dyn Node>,
    handle: task::JoinHandle<()>,
    cmd_channel: mpsc::Sender<NodeCtrl>,
}

/// An edge is a connection between two ports.
pub struct Edge(pub SmolStr, pub SmolStr);

/// A graph is a collection of nodes and edges.
pub struct Graph(StableGraph<NodeContainer, Edge, Directed>);

impl Graph {
    /// Create a new, empty graph.
    pub fn new() -> Self {
        Self(StableGraph::new())
    }
}

impl Default for Graph {
    fn default() -> Self {
        Self::new()
    }
}

impl Graph {
    /// Add a node to the graph.
    pub fn add_node(&mut self, node: Box<dyn Node>) -> NodeIndex {
        let (snd, rcv) = mpsc::channel(1);
        let nc: NodeContainer = NodeContainer {
            node,
            handle: task::spawn(node.run(rcv)),
            cmd_channel: snd,
        };
        self.0.add_node(nc)
    }

    /// Remove a node from the graph, returning the node, if it exists.
    pub async fn remove_node(&mut self, i: NodeIndex) -> bool {
        let maybe_nc = self.0.remove_node(i);
        match maybe_nc {
            Some(nc) => {
                nc.cmd_channel.send(NodeCtrl::Shutdown).await;
                nc.handle.await.is_ok()
            }
            None => false,
        }
    }

    /// Get a specified node from the graph without removing it.
    pub fn get_node(&self, i: NodeIndex) -> Option<&NodeContainer> {
        self.0.node_weight(i)
    }

    /// Mutably get a specified node from the graph without removing it.
    pub fn get_mut_node(&mut self, i: NodeIndex) -> Option<&mut NodeContainer> {
        self.0.node_weight_mut(i)
    }

    /// Add an edge between two ports.
    pub fn add_edge(
        &mut self,
        a: NodeIndex,
        b: NodeIndex,
        edge: Edge,
    ) -> anyhow::Result<EdgeIndex> {
        todo!()
    }

    /// Remove all edges from the graph, e.g. disconnect everything from everything else.
    pub fn clear_edges(&mut self) {
        self.0.clear_edges();
    }

    pub fn get_nodes(&self) -> impl Iterator<Item = &NodeContainer> {
        self.0.node_weights()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn creating_new_graph_doesnt_panic() {
        let _ = Graph::new();
    }
}
