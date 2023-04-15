//! Parsing and execution of Command Line Interface
//!
//! ## Example Commands
//! - Set Fixture {15 Thru 19 + 22 Thru 36} at {D: 15, G: G7, Z: 15}
//! - Set Group {2} at {D: 15}
//! - Connect Handle 1.1.7 At Cue 17
//! - Clear
//! - Connect Cue 5 At Group 7
//! - Select Cue 7
//! - Select Programmer 1
//! - SelFix {15 Thru 27}
//! - Create Cue 7

pub mod range;

use crate::command::*;

use anyhow::Result;
use nom::{
    branch::alt, bytes::complete as bc, character::complete as cc, combinator as c, multi as m,
    sequence as s, IResult,
};

use crate::graph;

impl HandleID {
    pub fn parse(i: &str) -> IResult<&str, Self> {
        c::map(m::separated_list1(bc::tag("."), cc::u64), Self::from)(i)
    }
}

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
