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

pub trait Node {
    fn get_ports(&self) -> HashMap<PortID, Port>;
    fn has_port(&self, id: &PortID) -> bool;
}

use std::fmt;
impl fmt::Debug for dyn Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Test implementation of debug")
    }
}

pub struct Edge(pub PortID, pub PortID);

pub struct Graph(StableGraph<Box<dyn Node>, Edge, Directed>);

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
    pub fn add_node(&mut self, node: Box<dyn Node>) -> NodeIndex {
        self.0.add_node(node)
    }

    pub fn remove_node(&mut self, i: NodeIndex) -> Option<Box<dyn Node>> {
        self.0.remove_node(i)
    }

    pub fn get_node(&self, i: NodeIndex) -> Option<&dyn Node> {
        Some(self.0.node_weight(i)?.as_ref())
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

impl Node for Box<dyn Node> {
    fn get_ports(&self) -> HashMap<PortID, Port> {
        self.as_ref().get_ports()
    }

    fn has_port(&self, id: &PortID) -> bool {
        self.as_ref().has_port(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creating_new_graph_doesnt_panic() {
        let _ = Graph::new();
    }

    #[derive(Debug, PartialEq)]
    struct EmptyNode {}
    impl Node for EmptyNode {
        fn get_ports(&self) -> HashMap<PortID, Port> {
            HashMap::new()
        }
        fn has_port(&self, _: &PortID) -> bool {
            false
        }
    }

    struct SimpleNode {}
    impl Node for SimpleNode {
        fn get_ports(&self) -> HashMap<PortID, Port> {
            HashMap::from([
                (
                    PortID {
                        id: "out".into(),
                        direction: PortDirection::Output,
                    },
                    Port {},
                ),
                (
                    PortID {
                        id: "in".into(),
                        direction: PortDirection::Input,
                    },
                    Port {},
                ),
            ])
        }

        fn has_port(&self, id: &PortID) -> bool {
            if (id.id == SmolStr::from("out") && id.direction == PortDirection::Output)
                || (id.id == SmolStr::from("in") && id.direction == PortDirection::Input)
            {
                true
            } else {
                false
            }
        }
    }

    impl std::cmp::PartialEq for dyn Node {
        fn eq(&self, other: &Self) -> bool {
            format!("{:?}", self) == format!("{:?}", other)
        }
    }

    #[test]
    fn create_graph_and_add_node() {
        let mut graph = Graph::new();
        let n1 = graph.add_node(Box::new(EmptyNode {}));
        let node = graph.remove_node(n1).unwrap();
        assert_eq!(node.as_ref(), &EmptyNode {} as &dyn Node);
    }
    #[test]
    fn create_graph_and_add_2_nodes() {
        let mut graph = Graph::new();
        let n1 = graph.add_node(Box::new(EmptyNode {}));
        let n2 = graph.add_node(Box::new(EmptyNode {}));
        let node1 = graph.remove_node(n1).unwrap();
        let node2 = graph.remove_node(n2).unwrap();
        assert_eq!(node1.as_ref(), &EmptyNode {} as &dyn Node);
        assert_eq!(node2.as_ref(), &EmptyNode {} as &dyn Node);
    }
    #[test]
    fn create_edge() {
        let mut graph = Graph::new();
        let n1 = graph.add_node(Box::new(SimpleNode {}));
        let n2 = graph.add_node(Box::new(SimpleNode {}));
        let _ = graph
            .add_edge(
                n1,
                n2,
                Edge(
                    PortID {
                        id: "out".into(),
                        direction: PortDirection::Output,
                    },
                    PortID {
                        id: "in".into(),
                        direction: PortDirection::Input,
                    },
                ),
            )
            .unwrap();
    }
}
