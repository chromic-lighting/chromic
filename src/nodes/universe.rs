//! TODO!: Implements a single DMX Universe

use crate::graph::Node;

pub struct Universe {}

impl Node for Universe {
    fn get_output(&self, _pid: crate::graph::PortID) -> crate::graph::data_types::Data {
        todo!()
    }

    fn get_ports(&self) -> std::collections::HashSet<crate::graph::PortID> {
        todo!()
    }

    fn update(&self, _data: crate::graph::DataSet) -> anyhow::Result<()> {
        todo!()
    }
}

impl Default for Universe {
    fn default() -> Self {
        todo!()
    }
}
