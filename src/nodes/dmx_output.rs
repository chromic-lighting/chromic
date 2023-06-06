//! TODO!: A DMX output method (Can accept one or several universes)

use crate::graph::{self, Node};
use tokio::sync::mpsc;

pub struct DMXOutput {}

#[async_trait::async_trait]
impl Node for DMXOutput {
    async fn run(&self, cmd_chan: mpsc::Receiver<graph::NodeCtrl>) {
        let _ = cmd_chan;
    }
}
