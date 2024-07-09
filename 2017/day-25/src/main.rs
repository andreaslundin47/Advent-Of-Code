use std::collections::HashSet;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, anychar, digit1, multispace1, newline},
    combinator::map_res,
    multi::separated_list1,
    sequence::{delimited, pair, preceded, separated_pair},
    Finish, IResult,
};

fn main() {
    //let input = include_str!("../sample1-input.txt").trim();
    let input = include_str!("../input.txt").trim();
    let (start_state, steps, states) = parse(input).finish().expect("Valid parse").1;

    let mut ones: HashSet<i32> = HashSet::new();
    let mut active_state: &State = &states[start_state];
    let mut cursor: i32 = 0;

    for _ in 0..steps {
        let actions = if ones.contains(&cursor) {
            &active_state.one_actions
        } else {
            &active_state.zero_actions
        };

        for action in actions {
            match action {
                Action::Write(write_value) => {
                    if write_value == &1 {
                        ones.insert(cursor);
                    } else {
                        ones.remove(&cursor);
                    }
                },
                Action::MoveLeft => cursor -= 1,
                Action::MoveRight => cursor += 1,
                Action::NextState(next_state) => active_state = &states[*next_state],
            }
        }
    }

    let count_ones = ones.len();
    println!("Part 1. Number of ones = {count_ones}");
}

enum Action {
    MoveLeft,
    MoveRight,
    Write(usize),
    NextState(usize),
}

struct State {
    zero_actions: Vec<Action>,
    one_actions: Vec<Action>,
}

fn parse_write(i: &str) -> IResult<&str, Action> {
    let (i, value) = map_res(
        delimited(tag("    - Write the value "), digit1, tag(".")),
        |v: &str| v.parse::<usize>(),
    )(i)?;

    Ok((i, Action::Write(value)))
}

fn parse_move(i: &str) -> IResult<&str, Action> {
    let (i, dir) = delimited(tag("    - Move one slot to the "), alpha1, tag("."))(i)?;
    let action = match dir {
        "left" => Action::MoveLeft,
        "right" => Action::MoveRight,
        _ => panic!("Invalid move next input!"),
    };

    Ok((i, action))
}

fn parse_next(i: &str) -> IResult<&str, Action> {
    let (i, next) = delimited(tag("    - Continue with state "), anychar, tag("."))(i)?;
    let next = next as usize - 'A' as usize;
    let action = Action::NextState(next);

    Ok((i, action))
}

fn parse_branch(i: &str) -> IResult<&str, Vec<Action>> {
    let (i, _) = delimited(tag("  If the current value is "), digit1, tag(":"))(i)?;
    let (i, _) = newline(i)?;
    let (i, actions) = separated_list1(newline, alt((parse_write, parse_move, parse_next)))(i)?;

    Ok((i, actions))
}

fn parse_state(i: &str) -> IResult<&str, State> {
    let (i, _) = delimited(tag("In state "), anychar, tag(":"))(i)?;
    let (i, _) = newline(i)?;
    let (i, branches) = separated_pair(parse_branch, newline, parse_branch)(i)?;

    Ok((
        i,
        State {
            zero_actions: branches.0,
            one_actions: branches.1,
        },
    ))
}

fn parse(i: &str) -> IResult<&str, (usize, usize, Vec<State>)> {
    let (i, start) = delimited(tag("Begin in state "), anychar, tag("."))(i)?;
    let (i, _) = newline(i)?;
    let (i, steps) = map_res(
        preceded(tag("Perform a diagnostic checksum after "), digit1),
        |n: &str| n.parse::<usize>(),
    )(i)?;

    let (i, _) = pair(tag(" steps."), multispace1)(i)?;
    let (i, states) = separated_list1(multispace1, parse_state)(i)?;

    let start = start as usize - 'A' as usize;

    Ok((i, (start, steps, states)))
}
