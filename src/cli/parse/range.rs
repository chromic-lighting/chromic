//! Parsing ranges of the form "{X Thru Y - Z Thru A + B}"

use crate::command::*;
use nom::{
    branch::alt, bytes::complete as bc, character::complete as cc, combinator as c, multi as m,
    sequence as s, IResult,
};
use std::{collections::HashSet, ops::RangeInclusive};

/// A Range of the form "X Thru Y"
fn range(i: &str) -> IResult<&str, RangeInclusive<u64>> {
    alt((
        c::map(
            s::separated_pair(
                cc::u64,
                s::delimited(cc::multispace0, bc::tag("Thru"), cc::multispace0),
                cc::u64,
            ),
            |(lo, hi)| lo..=hi,
        ),
        c::map(cc::u64, |n| n..=n),
    ))(i)
}

impl RangeType {
    fn parse(i: &str) -> IResult<&str, Self> {
        alt((
            c::map(
                s::preceded(s::tuple((bc::tag("+"), cc::multispace0)), range),
                RangeType::Add,
            ),
            c::map(
                s::preceded(s::tuple((bc::tag("-"), cc::multispace0)), range),
                RangeType::Sub,
            ),
        ))(i)
    }
}

impl ItemSelection {
    pub fn parse(i: &str) -> IResult<&str, Self> {
        s::delimited(
            bc::tag("{"),
            c::map(
                s::pair(
                    c::map(range, RangeType::Add),
                    m::many0(s::preceded(cc::multispace0, RangeType::parse)),
                ),
                |(a, mut rem)| {
                    Self({
                        rem.push(a);
                        rem
                    })
                },
            ),
            bc::tag("}"),
        )(i)
    }
}

impl From<ItemSelection> for HashSet<u64> {
    fn from(value: ItemSelection) -> Self {
        let (adds, subs): (HashSet<u64>, HashSet<u64>) =
            value
                .0
                .into_iter()
                .fold((HashSet::new(), HashSet::new()), |mut sets, r| {
                    match r {
                        RangeType::Add(range) => sets.0.extend(range),
                        RangeType::Sub(range) => sets.1.extend(range),
                    };
                    sets
                });
        &adds - &subs
    }
}
#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_range() {
        assert_eq!(range("15 Thru 25"), Ok(("", 15..=25)));
        assert_eq!(range("15"), Ok(("", 15..=15)));
        assert_eq!(range("15Thru25"), Ok(("", 15..=25)));
        assert_eq!(range("1 Thru 1999"), Ok(("", 1..=1999)));
    }

    #[test]
    fn test_range_type() {
        assert_eq!(
            RangeType::parse("+ 15 Thru 30"),
            Ok(("", RangeType::Add(15..=30)))
        );
        assert_eq!(
            RangeType::parse("- 15 Thru 30"),
            Ok(("", RangeType::Sub(15..=30)))
        );
        assert_eq!(
            RangeType::parse("+ 1 Thru 395"),
            Ok(("", RangeType::Add(1..=395)))
        );
        assert_eq!(
            RangeType::parse("+1 Thru 395"),
            Ok(("", RangeType::Add(1..=395)))
        );
        assert_eq!(
            RangeType::parse("+  1 Thru 395"),
            Ok(("", RangeType::Add(1..=395)))
        );
    }

    macro_rules! item_selection_to_hashmap_assertion {
        ($sel: literal, $result: expr) => {{
            let (rem, is) = ItemSelection::parse($sel).unwrap();
            assert_eq!(rem, "");
            let hs: HashSet<u64> = is.into();
            assert_eq!(hs, HashSet::from($result));
        }};
    }

    #[test]
    fn test_item_selection() {
        item_selection_to_hashmap_assertion!("{1 Thru 3}", [1, 2, 3]);
        item_selection_to_hashmap_assertion!("{1}", [1]);
        item_selection_to_hashmap_assertion!("{1 Thru 2 - 2}", [1]);
        item_selection_to_hashmap_assertion!("{1 Thru 5 - 2 Thru 4}", [1, 5]);
        item_selection_to_hashmap_assertion!("{1 Thru 5-2 Thru 4}", [1, 5]);
    }
}
