//! TODO!: A static colour.

use tokio::sync::mpsc;

use crate::graph;
use crate::graph::data_types::Colour;

/// Contains a colour as defined in the CIE XYZ Colour space.
pub struct StaticColour(Colour);

#[async_trait::async_trait]
impl graph::Node for StaticColour {
    async fn run(&self, cmd_chan: mpsc::Receiver<graph::NodeCtrl>) {
        let _ = cmd_chan;
    }
}

/// Black by default
impl Default for StaticColour {
    fn default() -> Self {
        Self(Colour {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        })
    }
}
