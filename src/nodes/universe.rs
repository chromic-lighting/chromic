//! TODO!: Implements a single DMX Universe

use crate::graph::Node;

pub struct Universe {}

impl Node for Universe {
    fn get_ports(&self) -> std::collections::HashMap<crate::graph::PortID, crate::graph::Port> {
        todo!()
    }

    fn has_port(&self, _id: &crate::graph::PortID) -> bool {
        todo!()
    }
}

impl Default for Universe {
    fn default() -> Self {
        todo!()
    }
}
