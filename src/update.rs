use crate::{command::Command, graph};
use tokio::sync::mpsc;

impl graph::Graph {
    pub async fn run(&mut self, channel: &mut mpsc::Receiver<Command>) -> anyhow::Result<()> {
        loop {
            let cmd = channel.recv().await;
            match cmd {
                Some(Command::Shutdown) => {
                    self.remove_all_nodes().await;
                    return Ok(());
                }
                Some(Command::GetGraphLayout(_ret)) => {
                    todo!()
                }
                _ => (),
            }
        }
    }
}
