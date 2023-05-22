use crate::graph;
use tokio::sync::mpsc;

impl graph::Graph {
    pub async fn run(
        &mut self,
        _channel: &mpsc::Receiver<crate::command::Command>,
    ) -> Result<(), anyhow::Error> {
        Ok(())
    }
}
