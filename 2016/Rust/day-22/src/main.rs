use itertools::Itertools;
use nom::{
    bytes::complete::{tag, take_until},
    character::complete::{digit1, line_ending, space1},
    combinator::map_res,
    multi::separated_list1,
    sequence::{delimited, tuple},
    IResult,
};

fn main() {
    let input = include_str!("../input.txt").trim();
    let nodes: Vec<Node> = parse(input).expect("A valid parse!").1;

    let viable_count: usize = nodes
        .iter()
        .permutations(2)
        .map(|pair| pair[0].can_viably_contain(pair[1]))
        .filter(|&valid| valid)
        .count();

    println!("Part 1. Number of vialble_combinations: {viable_count}");

    /*
       Carefully looking at the nodes, we note the following:

           *   Only one node is empty

           *   A transfer is only allowed if ALL data can be fully moved from one node
               to another, and the node contents are such that we can never merge the
               contents of two nodes. Only the empty node can take data, leaving the
               giver nodes as the new emtpy node. This reduces the problem to moving
               the empty node around.

           *   A few nodes are much larger than the rest, and they can never move data
               to another node. For the movement of the empty node, they act as obstacles
               we need to go around. In the grid, these nodes actually form a wall that
               almost divides the grid into an upper and lower half, with the empty node
               starting out in the lower half, and our targets in the top row.

       Using this information, we can basically calculate the minimum number of steps by hand.
       Manually move the empty node around the wall, up to the node to the left of the goal data,
       then move the goal data one step to the left. Finally we need five actual moves per
       move to the left of the goal data, until we reach the node at (0, 0).

    */
}

#[derive(Debug, PartialEq)]
struct Node {
    x: i32,
    y: i32,
    size: usize,
    used: usize,
    available: usize,
    percentage: usize,
}

impl Node {
    fn can_viably_contain(&self, other_node: &Node) -> bool {
        if self == other_node {
            return false;
        }

        other_node.used > 0 && other_node.used < self.available
    }
}

fn parse(i: &str) -> IResult<&str, Vec<Node>> {
    let (i, _) = tuple((take_until("\n"), line_ending, take_until("\n"), line_ending))(i)?;
    separated_list1(line_ending, parse_node)(i)
}

fn parse_node(i: &str) -> IResult<&str, Node> {
    // /dev/grid/node-x0-y0     92T   72T    20T   78%
    let (i, _) = tag("/dev/grid/node-x")(i)?;
    let (i, x) = map_res(digit1, |s: &str| s.parse::<i32>())(i)?;
    let (i, _) = tag("-y")(i)?;
    let (i, y) = map_res(digit1, |s: &str| s.parse::<i32>())(i)?;

    let (i, size) = map_res(delimited(space1, digit1, tag("T")), |s: &str| {
        s.parse::<usize>()
    })(i)?;

    let (i, used) = map_res(delimited(space1, digit1, tag("T")), |s: &str| {
        s.parse::<usize>()
    })(i)?;

    let (i, available) = map_res(delimited(space1, digit1, tag("T")), |s: &str| {
        s.parse::<usize>()
    })(i)?;

    let (i, percentage) = map_res(delimited(space1, digit1, tag("%")), |s: &str| {
        s.parse::<usize>()
    })(i)?;

    Ok((
        i,
        Node {
            x,
            y,
            size,
            used,
            available,
            percentage,
        },
    ))
}
