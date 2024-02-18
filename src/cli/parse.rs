//! Parsing and execution of Command Line Interface
//!
//! ## Example Commands
//! - Delete Fixture 15
//! - Delete Cue 17.5
//! - Delete Preset 15
//! - Load <Graphlayout>
//! - Connect Fixture 15 output Universe 2 input
//! - Set Fixture 5 input <Values>
//! - Set Preset 1 input <Values>
//! - Graph
//! - Shutdown

pub mod range;

use crate::command::*;

use anyhow::Result;
use nom::{
    branch::alt, bytes::complete as bc, character::complete as cc, combinator as c, multi as m,
    sequence as s, IResult,
};

use crate::graph;

impl From<Vec<u64>> for HandleID {
    fn from(_value: Vec<u64>) -> Self {
        todo!()
    }
}

/// Parse a single type of Operator
macro_rules! Oper_Opand {
    ($ts: literal, $op: path, $inner: path) => {
        c::map(
            s::preceded(s::pair(bc::tag($ts), cc::multispace0), $inner),
            $op,
        )
    };
}

/// A set of entities to be operated on
impl Operand {
    fn parse(i: &str) -> IResult<&str, Self> {
        alt((
            Oper_Opand!("Fixture", Operand::Fixture, ItemSelection::parse),
            Oper_Opand!("Universe", Operand::Universe, ItemSelection::parse),
            Oper_Opand!("Cue", Operand::Cue, ItemSelection::parse),
            Oper_Opand!("Programmer", Operand::Programmer, ItemSelection::parse),
            Oper_Opand!("Handle", Operand::Handle, HandleID::parse),
        ))(i)
    }
}

impl Command {
    pub fn parse(i: &str) -> IResult<&str, Self> {
        alt((
            Oper_Opand!("Clear", Command::Clear, Operand::parse),
            Oper_Opand!("DeSelFix", Command::DeSelFix, Operand::parse),
            Oper_Opand!("DeSelect", Command::DeSelect, Operand::parse),
            Oper_Opand!("SelFix", Command::SelFix, Operand::parse),
            Oper_Opand!("Select", Command::Select, Operand::parse),
        ))(i)
    }

    pub fn execute(_g: graph::Graph) -> Result<()> {
        todo!()
    }

    pub fn verify(_g: graph::Graph) -> bool {
        todo!()
    }
}

/// An ID for a physical handle on some hardware, e.g. MIDI controller or keyboard.
#[derive(Debug, Clone, PartialEq)]
pub struct HandleID {
    controller: usize,
    section: usize,
    page: usize,
    handle: usize,
}
impl HandleID {
    pub fn parse(i: &str) -> IResult<&str, Self> {
        c::map(m::separated_list1(bc::tag("."), cc::u64), Self::from)(i)
    }
}

/// A range either to be included, or not included in a selection
#[derive(PartialEq, Debug, Clone)]
pub enum RangeType {
    Add(RangeInclusive<u64>),
    Sub(RangeInclusive<u64>),
}

/// A selection of Items
#[derive(Debug, Clone, PartialEq)]
pub struct ItemSelection(pub Vec<RangeType>);

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_fixture_operand() {
        let (rem, op) = Operand::parse("Fixture {1}").unwrap();
        assert_eq!("", rem);
        assert_eq!(op, Operand::Fixture(ItemSelection::parse("{1}").unwrap().1));
    }

    #[test]
    fn test_selfix_command() {
        let (rem, op) = Command::parse("SelFix Fixture {1 Thru 2}").unwrap();
        assert_eq!("", rem);
        assert_eq!(
            op,
            Command::SelFix(Operand::Fixture(
                ItemSelection::parse("{1 Thru 2}").unwrap().1
            ))
        );
    }
}
