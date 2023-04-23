use crate::command;
use std::sync::mpsc;

pub mod parse;

pub fn run(channel: mpsc::Sender<command::Command>) {
    loop {
        let _ = channel;
    }
}
