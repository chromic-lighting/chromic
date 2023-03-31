//! # Chromic
//!
//! Chromic is a processing engine for video and lighting data.
//!
//! It is a graph based program for routing video and lighting data from generators, cues, etc to outputs (Video outputs, DMX outputs)
//!
//! It is designed so that data can flow through the graph from sources to outputs, but metadata can flow backwards, allowing earlier nodes to easily adapt to changing outputs.

// Public to prevent unused code warnings
// TODO: Make private again when no longer needed
pub mod cli;
pub mod file;
pub mod graph;
pub mod nodes;
pub mod update;

/// A simple placeholder main function for now.
///
/// TODO: Replace with actualy main function
fn main() -> anyhow::Result<()> {
    let mut g = graph::Graph::new();
    loop {
        g.render()?;
    }
}
