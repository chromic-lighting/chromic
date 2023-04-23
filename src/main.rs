//! # Chromic
//!
//! Chromic is a processing engine for video and lighting data.
//!
//! It is a graph based program for routing video and lighting data from generators, cues, etc to outputs (Video outputs, DMX outputs)
//!
//! It is designed so that data can flow through the graph from sources to outputs, but metadata can flow backwards, allowing earlier nodes to easily adapt to changing outputs.

use std::{sync::mpsc, thread};

// Public to prevent unused code warnings
// TODO: Make private again when no longer needed
pub mod cli;
pub mod command;
pub mod file;
pub mod graph;
pub mod nodes;
pub mod update;

fn main() -> anyhow::Result<()> {
    let mut g = graph::Graph::new();
    let (cmd_send, cmd_recv): (
        mpsc::Sender<command::Command>,
        mpsc::Receiver<command::Command>,
    ) = mpsc::channel();

    let cli_channel = cmd_send.clone();
    let _cli_thread = thread::spawn(move || {
        cli::run(cli_channel).unwrap();
    });

    let gui_channel = cmd_send;
    let _gui_thread = thread::spawn(move || loop {
        let _ = gui_channel;
    });

    loop {
        g.render()?;
        g.client_update(&cmd_recv)?
    }
}
