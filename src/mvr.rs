use std::fs::File;

use crate::graph;

impl graph::Graph {
    pub fn save(&self, _f: File) -> Result<(), anyhow::Error> {
        todo!();
    }
    pub fn load(_f: File) -> Result<Self, anyhow::Error> {
        todo!();
    }
}
