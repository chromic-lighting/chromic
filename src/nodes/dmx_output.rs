//! TODO!: A DMX output method (Can accept one or several universes)

use crate::graph::Node;

pub struct DMXOutput {}

impl Node for DMXOutput {
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

impl Default for DMXOutput {
    fn default() -> Self {
        todo!()
    }
}
