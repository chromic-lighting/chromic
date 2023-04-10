//! TODO!: A static colour.

use crate::graph::Node;

/// Contains a colour as defined in the CIE XYZ Colour space.
pub struct StaticColour {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Node for StaticColour {
    fn get_ports(&self) -> std::collections::HashSet<crate::graph::PortID> {
        todo!()
    }

    fn update(&self, _: std::time::Duration, _data: crate::graph::DataSet) -> anyhow::Result<()> {
        todo!()
    }

    fn get_output(&self, _pid: crate::graph::OutputPortID) -> crate::graph::data_types::Data {
        todo!()
    }
}

impl Default for StaticColour {
    fn default() -> Self {
        todo!()
    }
}
