use crate::graph;
use tokio::sync::mpsc;

use petgraph::stable_graph::NodeIndex;

impl graph::Graph {
    pub async fn run(
        &mut self,
        _channel: &mpsc::Receiver<crate::command::Command>,
    ) -> Result<(), anyhow::Error> {
        let _t0 = tokio::time::Instant::now();

        // Walk the graph,
        let nodes: Vec<&dyn graph::Node> = self
            .walk_upwards()?
            .into_iter()
            .map(|nodeindex: NodeIndex| self.get_node(nodeindex))
            .collect::<Option<Vec<_>>>()
            .expect("failed to lookup at least one node");

        let _in_need_of_update: Vec<_> = nodes
            .into_iter()
            .map(|node| (node, node.requires_update()))
            .collect();

        Ok(())
    }
}

fn _get_dependencies(_of: NodeIndex) -> Vec<NodeIndex> {
    todo!()
}
