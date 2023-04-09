//! Contains implementation of the Fixture node type.
//!
//! Fixture nodes can be parsed from a GDTF file, allowing easy imports of complex fixture personalities.

use opengdtf;
use std::fs::File;

use crate::graph::Node;

/// Represents a fixture in the graph.
pub struct Fixture {}

// TODO
impl Node for Fixture {
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

impl Default for Fixture {
    fn default() -> Self {
        todo!()
    }
}

/// A wrapper around File that holds GDTF files.
pub struct GDTFFile(File);

impl TryFrom<GDTFFile> for Fixture {
    type Error = opengdtf::Error;

    /// Attempt to obtain a Fixture from a GDTF file.
    fn try_from(gf: GDTFFile) -> Result<Self, Self::Error> {
        let f = gf.0;

        // Parse the file into a GDTF struct and a vec of handled problems.
        // Handled problems are problems that were successfully resolved by the parser.
        // Although resolved could mean that those fields were just ignored.
        let opengdtf::ParsedGdtf { gdtf, problems } = opengdtf::parse(f)?;

        dbg!(gdtf, problems);

        //TODO replace with actually constructing a fixture node from gdtf
        let fixture = Fixture::default();
        Ok(fixture)
    }
}
