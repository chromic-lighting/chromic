//! The Command Line Interface allows commands to be directly ran on the graph, seperate from the gui.

use crate::command;
use tokio::sync::mpsc;

use std::io;

pub mod parse;

pub fn run(channel: mpsc::Sender<command::Command>) -> anyhow::Result<()> {
    let stdin = io::stdin();
    let mut input = String::new();
    loop {
        stdin.read_line(&mut input)?;
        let (_, cmd) = command::Command::parse(&input).unwrap();
        channel.blocking_send(cmd).unwrap();
    }
}
