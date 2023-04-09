//! TODO!: A DMX output method (Can accept one or several universes)

use crate::graph::Node;

pub struct DMXOutput {}

impl Node for DMXOutput {
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

impl Default for DMXOutput {
    fn default() -> Self {
        todo!()
    }
}
