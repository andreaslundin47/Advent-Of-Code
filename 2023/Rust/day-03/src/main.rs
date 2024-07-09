use std::collections::{HashMap, HashSet};

use glam::IVec2;
use nom::{
    branch::alt,
    bytes::complete::{is_not, take_till1},
    character::complete::digit1,
    combinator::iterator,
    IResult, Parser,
};
use nom_locate::LocatedSpan;

type Span<'a> = LocatedSpan<&'a str>;
type SpanIVec2<'a> = LocatedSpan<&'a str, IVec2>;

#[derive(Debug, Eq, PartialEq, Hash)]
struct Number {
    value: usize,
    len: usize,
    pos: IVec2,
}

impl Number {
    fn contains(&self, pos: &IVec2) -> bool {
        let x_range = (self.pos.x - 1)..=(self.pos.x + self.len as i32);
        let y_range = (self.pos.y - 1)..=(self.pos.y + 1);

        x_range.contains(&pos.x) && y_range.contains(&pos.y)
    }
}

#[derive(Debug, Eq, PartialEq)]
enum Value<'a> {
    Void,
    Number(SpanIVec2<'a>),
    Symbol(SpanIVec2<'a>),
}

fn with_xy(span: Span) -> SpanIVec2 {
    let x = span.get_column() as i32 - 1;
    let y = span.location_line() as i32 - 1;
    span.map_extra(|_| IVec2::new(x, y))
}

fn parse_grid(input: Span) -> IResult<Span, Vec<Value>> {
    let mut it = iterator(
        input,
        alt((
            digit1.map(|span| with_xy(span)).map(Value::Number),
            is_not(".\n0123456789")
                .map(|span| with_xy(span))
                .map(Value::Symbol),
            take_till1(|c: char| c.is_ascii_digit() || c != '.' && c != '\n').map(|_| Value::Void),
        )),
    );

    let parsed = it
        .filter(|value| value != &Value::Void)
        .collect::<Vec<Value>>();

    let res: IResult<_, _> = it.finish();

    res.map(|(input, _)| (input, parsed))
}

fn main() {
    let raw_input = include_str!("../input.txt").trim();
    let span = Span::new(raw_input);

    let (_, nums_and_symbs) = parse_grid(span).expect("Valid parse");

    let symbols: HashMap<IVec2, char> = nums_and_symbs
        .iter()
        .filter_map(|val| {
            if let Value::Symbol(s) = val {
                Some((s.extra, s.fragment().chars().next().unwrap()))
            } else {
                None
            }
        })
        .collect();

    let numbers: HashSet<Number> = nums_and_symbs
        .iter()
        .filter_map(|num| {
            let Value::Number(s) = num else {
                return None;
            };
            let number = s.fragment();
            let value = number.parse::<usize>().unwrap();
            Some(Number {
                value,
                len: number.len(),
                pos: s.extra,
            })
        })
        .collect();

    let part_numbers_sum: usize = numbers
        .iter()
        .filter(|num| symbols.keys().any(|s| num.contains(s)))
        .map(|n| n.value)
        .sum();
    println!("Part 1. Part Numbers Sum: {part_numbers_sum}");

    let prod_sum: usize = symbols
        .iter()
        .filter(|(_, &sym)| sym == '*')
        .filter_map(|(pos, _)| {
            let adj = numbers
                .iter()
                .filter_map(|n| n.contains(&pos).then_some(n.value))
                .collect::<Vec<usize>>();
            if adj.len() == 2 {
                Some(adj[0] * adj[1])
            } else {
                None
            }
        })
        .sum();
    println!("Part 2. Sum of Products = {prod_sum}");
}
