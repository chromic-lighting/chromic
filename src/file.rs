//! Functions for operating on files, and saving and restoring the scene graph and patch to them.

use std::fs::File;

use crate::graph;

impl graph::Graph {
    /// Save a scene graph to a file
    pub fn save(&self, _f: File) -> Result<(), anyhow::Error> {
        todo!();
    }

    /// Load a scene graph from a file
    pub fn load(_f: File) -> Result<Self, anyhow::Error> {
        todo!();
    }
}

/// The My Virtual Rig (MVR) format is a file format that is used to share data for a scene between a lighting console, a visualizer, a CAD program or similar tools.
impl graph::Graph {
    /// Save the patch to an MVR file
    pub fn save_patch(&self, _f: File) -> Result<(), anyhow::Error> {
        todo!()
    }

    /// Load the patch from an MVR file
    pub fn load_patch(&self, _f: File) -> Result<(), anyhow::Error> {
        todo!()
    }
}
