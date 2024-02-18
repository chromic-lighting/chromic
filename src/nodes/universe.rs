//! TODO!: Implements a single DMX Universe

use crate::graph::{self, Node};
use tokio::sync::mpsc;

pub struct Universe {}

#[async_trait::async_trait]
impl Node for Universe {
    async fn run(&self, cmd_chan: mpsc::Receiver<graph::NodeCtrl>) {
        let _ = cmd_chan;
    }
}

impl Default for Universe {
    fn default() -> Self {
        todo!()
    }
}
