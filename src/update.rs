use crate::graph;
use std::sync::mpsc;

impl graph::Graph {
    pub fn render(&mut self, t: std::time::Duration) -> Result<(), anyhow::Error> {
        let needs_updating = self.get_nodes().filter(|n| n.requires_update(t));

        needs_updating.for_each(|n| update_node(self, n));

        Ok(())
    }

    /// Function iterates through all messages currently in the channel, performing their operations on the graph.
    pub fn client_update(
        &mut self,
        _channel: &mpsc::Receiver<crate::command::Command>,
    ) -> anyhow::Result<()> {
        todo!()
    }
}

fn update_node(_g: &mut graph::Graph, _n: &dyn graph::Node) {}
