use petgraph::{
    stable_graph::{EdgeIndex, NodeIndex, StableGraph},
    Directed,
};
use std::collections::HashMap;

use smol_str::SmolStr;

#[derive(Clone, Eq, PartialEq, Hash)]
pub struct PortID {
    pub id: SmolStr,
    pub direction: PortDirection,
}

#[derive(Clone, Eq, PartialEq, Hash)]
pub enum PortDirection {
    Input,
    Output,
}
pub struct Port {}

pub struct Node {
    pub ports: HashMap<PortID, Port>,
}

impl Node {
    pub fn has_port(&self, id: &PortID) -> bool {
        self.ports.contains_key(id)
    }
}

pub struct Edge(pub PortID, pub PortID);

pub struct Graph(StableGraph<Node, Edge, Directed>);

impl Graph {
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
    pub fn add_node(&mut self, node: Node) -> NodeIndex {
        self.0.add_node(node)
    }

    pub fn remove_node(&mut self, i: NodeIndex) -> Option<Node> {
        self.0.remove_node(i)
    }

    pub fn get_node(&self, i: NodeIndex) -> Option<&Node> {
        self.0.node_weight(i)
    }

    pub fn add_edge(
        &mut self,
        a: NodeIndex,
        b: NodeIndex,
        edge: Edge,
    ) -> anyhow::Result<EdgeIndex> {
        match (self.get_node(a), self.get_node(b)) {
            (None, _) => anyhow::bail!("Node A doesn't exist"),
            (_, None) => anyhow::bail!("Node B doesn't exist"),
            (Some(node_a), Some(node_b)) => {
                match (node_a.has_port(&edge.0), node_b.has_port(&edge.1)) {
                    (false, _) => anyhow::bail!("Output port doesn't exist"),
                    (_, false) => anyhow::bail!("input port doesn't exist"),
                    (true, true) => Ok(self.0.add_edge(a, b, edge)),
                }
            }
        }
    }

    pub fn clear_edges(&mut self) {
        self.0.clear_edges();
    }
}
