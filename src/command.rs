use petgraph::stable_graph::StableGraph;
use std::ops::RangeInclusive;
use std::sync::mpsc;

use crate::graph::{InputPortID, OutputPortID};

/// A Command to be executed
#[derive(Debug, PartialEq)]
pub enum Command {
    Clear(Operand),
    Connect, // TODO: Figure out API
    DeSelFix(Operand),
    DeSelect(Operand),
    SelFix(Operand),
    Select(Operand),
    Set(Operand, Values),
    GetGraphLayout(GraphLayoutReturnChannel),
    GetNode(Operand),
}

#[derive(Debug)]
pub struct GraphLayoutReturnChannel(mpsc::Sender<GraphLayout>);

impl PartialEq for GraphLayoutReturnChannel {
    fn eq(&self, _: &Self) -> bool {
        false
    }
}

struct GraphLayout(StableGraph<SummaryNode, SummaryEdge>);

#[derive(PartialEq)]
struct SummaryNode {
    name: String,
}

#[derive(PartialEq)]
struct SummaryEdge {
    data_type: String,
    output_port: OutputPortID,
    input_port: InputPortID,
}

/// An ID for a physical handle on some hardware, e.g. MIDI controller or keyboard.
#[derive(Debug, Clone, PartialEq)]
pub struct HandleID {
    controller: usize,
    section: usize,
    page: usize,
    handle: usize,
}
/// An item to be operated on, e.g. a Fixture or Cue (Something that implements Node)
#[derive(Clone, Debug, PartialEq)]
pub enum Operand {
    Fixture(ItemSelection),
    Universe(ItemSelection),
    Cue(ItemSelection),
    Programmer(ItemSelection),
    Handle(HandleID),
}
/// A const set of paramaters
#[derive(Debug, PartialEq)]
pub struct Values {} // TODO: Figure out API

/// A range either to be included, or not included in a selection
#[derive(PartialEq, Debug, Clone)]
pub enum RangeType {
    Add(RangeInclusive<u64>),
    Sub(RangeInclusive<u64>),
}

/// A selection of Items
#[derive(Debug, Clone, PartialEq)]
pub struct ItemSelection(pub Vec<RangeType>);
