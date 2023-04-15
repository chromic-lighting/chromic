use crate::graph;
use std::sync::mpsc;

impl graph::Graph {
    pub fn render(&mut self) -> Result<(), anyhow::Error> {
        todo!()
    }

    /// Function iterates through all messages currently in the channel, performing their operations on the graph.
    pub fn client_update(
        &mut self,
        _channel: &mpsc::Receiver<crate::command::Command>,
    ) -> anyhow::Result<()> {
        todo!()
    }
}
