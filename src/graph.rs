//! Handles the graph logic, managing nodes and edges.

use petgraph::{
    stable_graph::{EdgeIndex, NodeIndex, StableGraph},
    Directed,
};

use async_trait::async_trait;
use smol_str::SmolStr;

use tokio::sync::{mpsc, oneshot, watch};

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

/// An edge is a connection between two ports.
pub struct Edge(pub SmolStr, pub SmolStr);

/// A graph is a collection of nodes and edges.
pub struct Graph(StableGraph<Box<dyn Node>, Edge, Directed>);

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
        self.0.add_node(node)
    }

    /// Remove a node from the graph, returning the node, if it exists.
    pub fn remove_node(&mut self, i: NodeIndex) -> Option<Box<dyn Node>> {
        self.0.remove_node(i)
    }

    /// Get a specified node from the graph without removing it.
    pub fn get_node(&self, i: NodeIndex) -> Option<&dyn Node> {
        self.0.node_weight(i).map(|n| n.as_ref())
    }

    /// Mutably get a specified node from the graph without removing it.
    pub fn get_mut_node(&mut self, i: NodeIndex) -> Option<&mut (dyn Node + 'static)> {
        self.0.node_weight_mut(i).map(|n| n.as_mut())
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

    pub fn get_nodes(&self) -> impl Iterator<Item = &dyn Node> {
        self.0.node_weights().map(|n| n.as_ref())
    }

    pub fn walk_upwards(&self) -> anyhow::Result<Vec<NodeIndex>> {
        petgraph::algo::toposort(&self.0, None)
            .map_err(|_| anyhow::anyhow!("Graph contained a cycle"))
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
