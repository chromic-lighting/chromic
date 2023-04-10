//! TODO!: A static colour.

use std::collections::HashSet;

use crate::graph;
use crate::graph::data_types::{Colour, Data};

/// Contains a colour as defined in the CIE XYZ Colour space.
pub struct StaticColour(Colour);

impl graph::Node for StaticColour {
    fn get_ports(&self) -> HashSet<graph::PortID> {
        HashSet::from([graph::PortID::Output(graph::OutputPortID("Colour".into()))])
    }

    fn update(&self, _: std::time::Duration, _data: crate::graph::DataSet) -> anyhow::Result<()> {
        Ok(())
    }

    fn get_output(&self, _pid: crate::graph::OutputPortID) -> Data {
        Data::Colour(self.0)
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
