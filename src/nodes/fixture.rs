use opengdtf;
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

impl Default for Fixture {
    fn default() -> Self {
        todo!()
    }
}

struct GDTFFile(File);

impl TryFrom<GDTFFile> for Fixture {
    type Error = opengdtf::Error;

    fn try_from(_value: GDTFFile) -> Result<Self, Self::Error> {
        // TODO replace with actually parsing the file
        let _gdtf = opengdtf::Gdtf::default();

        //TODO replace with actually constructing a fixture node from gdtf
        let fixture = Fixture::default();
        Ok(fixture)
    }
}
