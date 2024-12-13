use nom::{
    bytes::complete::tag,
    character::complete::{digit1, multispace1, newline},
    combinator::map_res,
    multi::separated_list1,
    IResult,
};

use crate::Entry;

pub fn parse_entries(i: &str) -> IResult<&str, Vec<Entry>> {
    separated_list1(multispace1, parse_entry)(i)
}

fn parse_entry(i: &str) -> IResult<&str, Entry> {
    let mut parse_int = map_res(digit1, |s: &str| s.parse::<i64>());

    let (i, _) = tag("Button A: X+")(i)?;
    let (i, x_a) = parse_int(i)?;
    let (i, _) = tag(", Y+")(i)?;
    let (i, y_a) = parse_int(i)?;

    let (i, _) = newline(i)?;

    let (i, _) = tag("Button B: X+")(i)?;
    let (i, x_b) = parse_int(i)?;
    let (i, _) = tag(", Y+")(i)?;
    let (i, y_b) = parse_int(i)?;

    let (i, _) = newline(i)?;

    let (i, _) = tag("Prize: X=")(i)?;
    let (i, x_target) = parse_int(i)?;
    let (i, _) = tag(", Y=")(i)?;
    let (i, y_target) = parse_int(i)?;

    Ok((
        i,
        Entry {
            x_a,
            x_b,
            y_a,
            y_b,
            x_target,
            y_target,
        },
    ))
}
