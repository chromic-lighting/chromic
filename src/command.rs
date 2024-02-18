use petgraph::stable_graph::{NodeIndex, StableGraph};
use smol_str::SmolStr;
use std::ops::RangeInclusive;
use tokio::sync::oneshot;

/// A Command to be executed
#[derive(Debug, PartialEq)]
pub enum Command {
    Delete(Operand),
    Load(GraphLayout),
    Connect((Operand, SmolStr), (Operand, SmolStr)), // TODO: Figure out API
    Set((Operand, SmolStr), Values),
    GetGraphLayout(GraphLayoutReturnChannel),
    Shutdown,
}

#[derive(Debug)]
pub struct GraphLayoutReturnChannel(oneshot::Sender<GraphLayout>);

impl PartialEq for GraphLayoutReturnChannel {
    fn eq(&self, _: &Self) -> bool {
        false
    }
}
#[derive(Debug)]
struct GraphLayout(StableGraph<SummaryNode, SummaryEdge>);

impl PartialEq for GraphLayout {
    fn eq(&self, _: &Self) -> bool {
        false
    }
}

#[derive(Debug, PartialEq)]
struct SummaryNode {
    name: String,
}

#[derive(Debug, PartialEq)]
struct SummaryEdge {
    data_type: String,
    output_port: SmolStr,
    input_port: SmolStr,
}

/// An item to be operated on, e.g. a Fixture or Cue (Something that implements Node)
#[derive(Clone, Debug, PartialEq)]
pub struct Operand(NodeIndex);

/// A const set of paramaters
#[derive(Debug, PartialEq)]
pub struct Values {} // TODO: Figure out API
