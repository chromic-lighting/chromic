//! Functions for operating on MVR files, and saving and restoring the scene graph to them.
//!
//! The My Virtual Rig (MVR) format is a file format that is used to share data for a scene between a lighting console, a visualizer, a CAD program or similar tools.

use std::fs::File;

use crate::graph;

impl graph::Graph {
    /// Save a scene graph to an MVR file
    pub fn save(&self, _f: File) -> Result<(), anyhow::Error> {
        todo!();
    }

    /// Load a scene graph from an MVR file
    pub fn load(_f: File) -> Result<Self, anyhow::Error> {
        todo!();
    }
}
