//! TODO!: A DMX output method (Can accept one or several universes)

use crate::graph::Node;

pub struct DMXOutput {}

impl Node for DMXOutput {
    fn get_ports(&self) -> std::collections::HashMap<crate::graph::PortID, crate::graph::Port> {
        todo!()
    }

    fn has_port(&self, _id: &crate::graph::PortID) -> bool {
        todo!()
    }
}

impl Default for DMXOutput {
    fn default() -> Self {
        todo!()
    }
}
