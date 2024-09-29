use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{anychar, digit1, line_ending},
    combinator::map_res,
    multi::separated_list1,
    IResult,
};

use crate::Operation;

pub fn parse_operations(input: &str) -> IResult<&str, Vec<Operation>> {
    separated_list1(
        line_ending,
        alt((
            swap_positions,
            swap_letters,
            rotate_left,
            rotate_right,
            rotate_letter,
            reverse_index_span,
            move_between,
        )),
    )(input)
}

fn swap_positions(i: &str) -> IResult<&str, Operation> {
    let (i, _) = tag("swap position ")(i)?;
    let (i, index_one) = map_res(digit1, |s: &str| s.parse::<usize>())(i)?;
    let (i, _) = tag(" with position ")(i)?;
    let (i, index_two) = map_res(digit1, |s: &str| s.parse::<usize>())(i)?;

    Ok((
        i,
        Operation::SwapPositions {
            index_one,
            index_two,
        },
    ))
}

fn swap_letters(i: &str) -> IResult<&str, Operation> {
    let (i, _) = tag("swap letter ")(i)?;
    let (i, letter_one) = anychar(i)?;
    let (i, _) = tag(" with letter ")(i)?;
    let (i, letter_two) = anychar(i)?;

    Ok((
        i,
        Operation::SwapLetters {
            letter_one,
            letter_two,
        },
    ))
}

fn rotate_left(i: &str) -> IResult<&str, Operation> {
    let (i, _) = tag("rotate left ")(i)?;
    let (i, steps) = map_res(digit1, |s: &str| s.parse::<usize>())(i)?;
    let (i, _) = alt((tag(" steps"), tag(" step")))(i)?;

    Ok((i, Operation::RotateLeft { steps }))
}

fn rotate_right(i: &str) -> IResult<&str, Operation> {
    let (i, _) = tag("rotate right ")(i)?;
    let (i, steps) = map_res(digit1, |s: &str| s.parse::<usize>())(i)?;
    let (i, _) = alt((tag(" steps"), tag(" step")))(i)?;

    Ok((i, Operation::RotateRight { steps }))
}

fn rotate_letter(i: &str) -> IResult<&str, Operation> {
    let (i, _) = tag("rotate based on position of letter ")(i)?;
    let (i, letter) = anychar(i)?;

    Ok((i, Operation::RotateOnLetter { letter }))
}

fn reverse_index_span(i: &str) -> IResult<&str, Operation> {
    let (i, _) = tag("reverse positions ")(i)?;
    let (i, index_one) = map_res(digit1, |s: &str| s.parse::<usize>())(i)?;
    let (i, _) = tag(" through ")(i)?;
    let (i, index_two) = map_res(digit1, |s: &str| s.parse::<usize>())(i)?;

    Ok((
        i,
        Operation::ReverseIndexSpan {
            index_one,
            index_two,
        },
    ))
}

fn move_between(i: &str) -> IResult<&str, Operation> {
    let (i, _) = tag("move position ")(i)?;
    let (i, index_one) = map_res(digit1, |s: &str| s.parse::<usize>())(i)?;
    let (i, _) = tag(" to position ")(i)?;
    let (i, index_two) = map_res(digit1, |s: &str| s.parse::<usize>())(i)?;

    Ok((
        i,
        Operation::MoveBetweenPositions {
            index_one,
            index_two,
        },
    ))
}
