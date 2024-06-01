use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, digit1, multispace1, newline, one_of},
    combinator::map_res,
    multi::separated_list1,
    sequence::{preceded, separated_pair},
    Finish, IResult,
};
use std::{collections::HashMap, ops::Range};

type Part = HashMap<PartCateory, i32>;

fn main() {
    let input = include_str!("../input.txt").trim();
    let (workflows, parts) = parse(input);

    let rating_numbers_sum: usize = parts
        .iter()
        .filter(|part| validate_part(part, &workflows))
        .map(|valid_part| valid_part.values().sum::<i32>() as usize)
        .sum();
    println!("Part 1. Sum = {rating_numbers_sum}");

    let count = count_distinct(&workflows);
    println!("Part 2. Combinations = {count}");
}

fn validate_part(part: &Part, workflows: &HashMap<String, WorkFlow>) -> bool {
    let init_flow = String::from("in");

    let mut flow_name = &init_flow;
    let mut current_rule_index = 0;

    loop {
        let rule = &workflows[flow_name].rules[current_rule_index];

        let result: &Result = match rule {
            Rule::Comparator {
                expression,
                success_result,
            } => match expression {
                CompExpression::GreaterThan(category, value) => {
                    if part[category] > *value {
                        success_result
                    } else {
                        &Result::NextRule
                    }
                }
                CompExpression::LessThan(category, value) => {
                    if part[category] < *value {
                        success_result
                    } else {
                        &Result::NextRule
                    }
                }
            },
            Rule::Default { result } => result,
        };

        match result {
            Result::Accepted => return true,
            Result::Rejected => return false,
            Result::NextRule => current_rule_index += 1,
            Result::RoutedToWorkFlow { name } => {
                flow_name = name;
                current_rule_index = 0;
            }
        }
    }
}

fn count_distinct(workflows: &HashMap<String, WorkFlow>) -> i64 {
    let start_block = RatingsBlock {
        xr: 1..4001,
        mr: 1..4001,
        ar: 1..4001,
        sr: 1..4001,
        flow_id: String::from("in"),
        rule_index: 0,
    };

    let mut blocks = vec![start_block];
    let mut accepted_blocks: Vec<RatingsBlock> = vec![];

    while let Some(mut block) = blocks.pop() {
        let rule = &workflows[&block.flow_id].rules[block.rule_index];

        match rule {
            Rule::Comparator {
                expression,
                success_result,
            } => {
                let (accept_block, reject_block) = block.range_split(expression);
                if let Some(mut acc) = accept_block {
                    match success_result {
                        Result::Accepted => accepted_blocks.push(acc),
                        Result::Rejected => (),
                        Result::RoutedToWorkFlow { name } => {
                            name.clone_into(&mut acc.flow_id);
                            acc.rule_index = 0;
                            blocks.push(acc);
                        }
                        Result::NextRule => unreachable!("This rule should not appear here!"),
                    }
                }
                if let Some(mut rej) = reject_block {
                    rej.rule_index += 1;
                    blocks.push(rej);
                }
            }
            Rule::Default { result } => match result {
                Result::Accepted => accepted_blocks.push(block),
                Result::Rejected => (),
                Result::RoutedToWorkFlow { name } => {
                    name.clone_into(&mut block.flow_id);
                    block.rule_index = 0;
                    blocks.push(block);
                }
                Result::NextRule => {
                    block.rule_index += 1;
                    blocks.push(block);
                }
            },
        }
    }

    accepted_blocks.iter().map(|block| block.combos()).sum()
}

#[derive(Clone)]
struct RatingsBlock {
    xr: Range<i32>,
    mr: Range<i32>,
    ar: Range<i32>,
    sr: Range<i32>,
    flow_id: String,
    rule_index: usize,
}

impl RatingsBlock {
    fn combos(&self) -> i64 {
        let dx = (self.xr.end - self.xr.start) as i64;
        let dm = (self.mr.end - self.mr.start) as i64;
        let da = (self.ar.end - self.ar.start) as i64;
        let ds = (self.sr.end - self.sr.start) as i64;

        dx * dm * da * ds
    }

    fn get_range(&self, category: &PartCateory) -> Range<i32> {
        match category {
            PartCateory::X => self.xr.clone(),
            PartCateory::M => self.mr.clone(),
            PartCateory::A => self.ar.clone(),
            PartCateory::S => self.sr.clone(),
        }
    }

    fn set_range(&self, category: &PartCateory, range: Range<i32>) -> Self {
        let mut clone = self.clone();
        match category {
            PartCateory::X => clone.xr = range,
            PartCateory::M => clone.mr = range,
            PartCateory::A => clone.ar = range,
            PartCateory::S => clone.sr = range,
        }

        clone
    }

    fn range_split(
        &self,
        comparison: &CompExpression,
    ) -> (Option<RatingsBlock>, Option<RatingsBlock>) {
        match comparison {
            CompExpression::GreaterThan(category, value) => {
                let range = self.get_range(category);

                let acc_range = range.start.max(*value + 1)..range.end;
                let rej_range = range.start..range.end.min(*value + 1);

                let accept_half = match acc_range.is_empty() {
                    true => None,
                    false => Some(self.set_range(category, acc_range)),
                };

                let reject_half = match rej_range.is_empty() {
                    false => Some(self.set_range(category, rej_range)),
                    true => None,
                };

                (accept_half, reject_half)
            }
            CompExpression::LessThan(category, value) => {
                let range = self.get_range(category);

                let acc_range = range.start..range.end.min(*value);
                let rej_range = range.start.max(*value)..range.end;

                let accept_half = match acc_range.is_empty() {
                    true => None,
                    false => Some(self.set_range(category, acc_range)),
                };

                let reject_half = match rej_range.is_empty() {
                    true => None,
                    false => Some(self.set_range(category, rej_range)),
                };

                (accept_half, reject_half)
            }
        }
    }
}

#[derive(Debug)]
enum CompExpression {
    GreaterThan(PartCateory, i32),
    LessThan(PartCateory, i32),
}

#[derive(Debug, Eq, PartialEq, Hash)]
enum PartCateory {
    X,
    M,
    A,
    S,
}

#[derive(Debug, Clone)]
enum Result {
    Accepted,
    Rejected,
    RoutedToWorkFlow { name: String },
    NextRule,
}

#[derive(Debug)]
enum Rule {
    Comparator {
        expression: CompExpression,
        success_result: Result,
    },
    Default {
        result: Result,
    },
}

#[derive(Debug)]
struct WorkFlow {
    name: String,
    rules: Vec<Rule>,
}

fn parse_comparison_rule(i: &str) -> IResult<&str, Rule> {
    let (i, category) = alpha1(i)?;
    let (i, comparator) = one_of("<>")(i)?;
    let (i, value) = digit1(i)?;
    let (i, _) = tag(":")(i)?;
    let (i, res) = alpha1(i)?;

    let category = match category {
        "x" => PartCateory::X,
        "m" => PartCateory::M,
        "a" => PartCateory::A,
        "s" => PartCateory::S,
        _ => panic!("Not a xmas category!"),
    };

    let value = value.parse::<i32>().unwrap();

    let success_result = match res {
        "A" => Result::Accepted,
        "R" => Result::Rejected,
        _ => Result::RoutedToWorkFlow {
            name: res.to_string(),
        },
    };

    let rule = match comparator {
        '>' => Rule::Comparator {
            expression: CompExpression::GreaterThan(category, value),
            success_result,
        },
        '<' => Rule::Comparator {
            expression: CompExpression::LessThan(category, value),
            success_result,
        },
        _ => panic!("Not a comparison!"),
    };

    Ok((i, rule))
}

fn parse_default_rule(i: &str) -> IResult<&str, Rule> {
    let (i, res) = alpha1(i)?;

    let result = match res {
        "A" => Result::Accepted,
        "R" => Result::Rejected,
        _ => Result::RoutedToWorkFlow {
            name: res.to_string(),
        },
    };

    Ok((i, Rule::Default { result }))
}

fn parse_rule(i: &str) -> IResult<&str, Rule> {
    alt((parse_comparison_rule, parse_default_rule))(i)
}

fn parse_workflow(i: &str) -> IResult<&str, WorkFlow> {
    let (i, name) = alpha1(i)?;
    let (i, _) = tag("{")(i)?;
    let (i, rules) = separated_list1(tag(","), parse_rule)(i)?;
    let (i, _) = tag("}")(i)?;

    let name = name.to_string();

    Ok((i, WorkFlow { name, rules }))
}

fn parse_part(i: &str) -> IResult<&str, Part> {
    let (i, _) = tag("{")(i)?;
    let (i, x) = map_res(preceded(tag("x="), digit1), |v: &str| v.parse::<i32>())(i)?;
    let (i, m) = map_res(preceded(tag(",m="), digit1), |v: &str| v.parse::<i32>())(i)?;
    let (i, a) = map_res(preceded(tag(",a="), digit1), |v: &str| v.parse::<i32>())(i)?;
    let (i, s) = map_res(preceded(tag(",s="), digit1), |v: &str| v.parse::<i32>())(i)?;
    let (i, _) = tag("}")(i)?;

    Ok((
        i,
        HashMap::from([
            (PartCateory::X, x),
            (PartCateory::M, m),
            (PartCateory::A, a),
            (PartCateory::S, s),
        ]),
    ))
}

fn workflows(i: &str) -> IResult<&str, HashMap<String, WorkFlow>> {
    let (i, workflows) = separated_list1(newline, parse_workflow)(i)?;
    let workflows = workflows
        .into_iter()
        .map(|wf| (wf.name.clone(), wf))
        .collect::<HashMap<String, WorkFlow>>();
    Ok((i, workflows))
}

fn parts(i: &str) -> IResult<&str, Vec<Part>> {
    separated_list1(newline, parse_part)(i)
}

fn parse(i: &str) -> (HashMap<String, WorkFlow>, Vec<Part>) {
    let (_, (w, p)) = separated_pair(workflows, multispace1, parts)(i)
        .finish()
        .expect("A valid parse");
    (w, p)
}
