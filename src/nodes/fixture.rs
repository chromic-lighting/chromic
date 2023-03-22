use std::fs::File;

use crate::graph::Node;

struct Fixture {}

// TODO
impl Node for Fixture {
    fn get_ports(&self) -> std::collections::HashMap<crate::graph::PortID, crate::graph::Port> {
        todo!()
    }

    fn has_port(&self, _id: &crate::graph::PortID) -> bool {
        todo!()
    }
}

struct GDTFFile(File);

impl TryFrom<GDTFFile> for Fixture {
    type Error;

    fn try_from(value: GDTFFile) -> Result<Self, Self::Error> {
        todo!()
    }
}
