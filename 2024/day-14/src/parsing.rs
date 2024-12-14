use nom::{
    bytes::complete::tag,
    character::complete::{digit1, newline},
    combinator::{map_res, opt, recognize},
    multi::separated_list1,
    sequence::preceded,
    IResult,
};

use crate::Robot;
use glam::IVec2;

pub fn parse_robots(input: &str) -> IResult<&str, Vec<Robot>> {
    let (i, mut robots) = separated_list1(newline, parse_robot)(input)?;
    for (id, r) in robots.iter_mut().enumerate() {
        r.id = id;
    }
    Ok((i, robots))
}

fn parse_robot(i: &str) -> IResult<&str, Robot> {
    //p=9,5 v=-3,-3
    let (i, _) = tag("p=")(i)?;
    let (i, p_x) = parse_i32(i)?;
    let (i, _) = tag(",")(i)?;
    let (i, p_y) = parse_i32(i)?;
    let (i, _) = tag(" v=")(i)?;
    let (i, v_x) = parse_i32(i)?;
    let (i, _) = tag(",")(i)?;
    let (i, v_y) = parse_i32(i)?;

    Ok((
        i,
        Robot {
            id: 0,
            pos: IVec2::new(p_x, p_y),
            vel: IVec2::new(v_x, v_y),
        },
    ))
}

fn parse_i32(input: &str) -> IResult<&str, i32> {
    let (i, number) = map_res(recognize(preceded(opt(tag("-")), digit1)), |s: &str| {
        s.parse::<i32>()
    })(input)?;

    Ok((i, number))
}
