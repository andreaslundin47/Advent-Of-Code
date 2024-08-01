use nom::{
    self,
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, newline},
    combinator::{map, map_res},
    multi::separated_list1,
    sequence::preceded,
    Finish, IResult,
};

use super::{Instr, Recipient};

fn parse_recipient(i: &str) -> IResult<&str, Recipient> {
    let parse_value = |i| map_res(digit1, |s: &str| s.parse::<usize>())(i);

    let parse_bot = map(preceded(tag("bot "), parse_value), |v: usize| {
        Recipient::Bot(v)
    });
    let parse_output = map(preceded(tag("output "), parse_value), |v: usize| {
        Recipient::Output(v)
    });

    alt((parse_bot, parse_output))(i)
}

fn parse_init(i: &str) -> IResult<&str, Instr> {
    let (i, _) = tag("value ")(i)?;
    let (i, value) = map_res(digit1, |s: &str| s.parse::<usize>())(i)?;
    let (i, _) = tag(" goes to bot ")(i)?;
    let (i, bot) = map_res(digit1, |s: &str| s.parse::<usize>())(i)?;

    Ok((i, Instr::Init { bot, value }))
}

fn parse_rule(i: &str) -> IResult<&str, Instr> {
    let (i, _) = tag("bot ")(i)?;
    let (i, bot) = map_res(digit1, |s: &str| s.parse::<usize>())(i)?;
    let (i, _) = tag(" gives low to ")(i)?;
    let (i, lower) = parse_recipient(i)?;
    let (i, _) = tag(" and high to ")(i)?;
    let (i, higher) = parse_recipient(i)?;

    Ok((i, Instr::Rule { bot, lower, higher }))
}

pub fn parse_instructions(i: &str) -> Vec<Instr> {
    separated_list1(newline, alt((parse_init, parse_rule)))(i)
        .finish()
        .expect("A valid parse")
        .1
}
