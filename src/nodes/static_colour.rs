//! TODO!: A static colour.

use crate::graph::Node;

/// Contains a colour as defined in the CIE XYZ Colour space.
pub struct StaticColour {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Node for StaticColour {
    fn get_ports(&self) -> std::collections::HashMap<crate::graph::PortID, crate::graph::Port> {
        todo!()
    }

    fn has_port(&self, _id: &crate::graph::PortID) -> bool {
        todo!()
    }
}

impl Default for StaticColour {
    fn default() -> Self {
        todo!()
    }
}
