use glam::IVec2;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

fn main() {
    let raw_input = include_str!("../input.txt");
    let grid = Grid::from(&raw_input);
    solve_part_one(&grid);
    solve_part_two(&grid);
}

fn solve_part_one(grid: &Grid) {
    let start = BeamHead::new(IVec2::new(0, 0), IVec2::new(1, 0));
    let count = count_energized(start, grid);
    println!("Part 1. {}", count);
}

fn solve_part_two(grid: &Grid) {
    let top = (0..grid.width).map(|x| BeamHead::new(IVec2::new(x, 0), IVec2::new(0, 1)));

    let bottom =
        (0..grid.width).map(|x| BeamHead::new(IVec2::new(x, grid.height - 1), IVec2::new(0, -1)));

    let left = (0..grid.height).map(|y| BeamHead::new(IVec2::new(0, y), IVec2::new(1, 0)));

    let right =
        (0..grid.height).map(|y| BeamHead::new(IVec2::new(grid.height - 1, y), IVec2::new(-1, 0)));

    let starting_beams = top.chain(bottom).chain(left).chain(right);

    let count = starting_beams
        .map(|beam| count_energized(beam, &grid))
        .max()
        .unwrap();
    println!("Part 2. {}", count);
}

fn count_energized(start: BeamHead, grid: &Grid) -> usize {
    let mut current_beam_heads = vec![start];
    let mut created: HashSet<BeamHead> = HashSet::from([start]);

    while !current_beam_heads.is_empty() {
        current_beam_heads = current_beam_heads
            .into_iter()
            .flat_map(|beam| {
                let new_beams = move_beam_head(&beam, &grid);

                new_beams.into_iter().filter(|new_beam| {
                    grid.is_inside(&new_beam.position) && !created.contains(&new_beam)
                })
            })
            .collect::<Vec<BeamHead>>();

        current_beam_heads.iter().for_each(|&beam| {
            created.insert(beam);
        });
    }

    created.iter().map(|head| head.position).unique().count()
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct BeamHead {
    position: IVec2,
    direction: IVec2,
}

impl BeamHead {
    fn new(pos: IVec2, dir: IVec2) -> BeamHead {
        BeamHead {
            position: pos,
            direction: dir,
        }
    }
}

#[derive(Debug)]
enum Element {
    Empty,
    MirrorInc,
    MirrorDec,
    SplitterHor,
    SplitterVer,
}

impl std::fmt::Display for Element {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let repr = match self {
            Self::Empty => ".",
            Self::MirrorInc => "/",
            Self::MirrorDec => "\\",
            Self::SplitterHor => "-",
            Self::SplitterVer => "|",
        };
        write!(f, "{repr}")
    }
}

struct Grid {
    elements: HashMap<IVec2, Element>,
    width: i32,
    height: i32,
}

impl Grid {
    fn from(input: &str) -> Grid {
        use Element::*;
        let elements = input
            .lines()
            .enumerate()
            .flat_map(|(y, row)| {
                row.chars().enumerate().map(move |(x, c)| {
                    let elem = match c {
                        '.' => Empty,
                        '/' => MirrorInc,
                        '\\' => MirrorDec,
                        '-' => SplitterHor,
                        '|' => SplitterVer,
                        _ => panic!("Character should not appear in input!"),
                    };
                    (IVec2::new(x as i32, y as i32), elem)
                })
            })
            .collect::<HashMap<IVec2, Element>>();

        let max_x = elements.keys().max_by(|v1, v2| v1.x.cmp(&v2.x)).unwrap().x;
        let max_y = elements.keys().max_by(|v1, v2| v1.y.cmp(&v2.y)).unwrap().y;

        Grid {
            elements,
            width: max_x + 1,
            height: max_y + 1,
        }
    }

    fn is_inside(&self, pos: &IVec2) -> bool {
        pos.x >= 0 && pos.y >= 0 && pos.x < self.width && pos.y < self.height
    }
}

fn move_beam_head(head: &BeamHead, grid: &Grid) -> Vec<BeamHead> {
    use Element::*;
    let left90 = IVec2::new(0, -1);
    let right90 = IVec2::new(0, 1);

    if let Some(element) = grid.elements.get(&head.position) {
        match element {
            Empty => {
                vec![BeamHead::new(
                    head.position + head.direction,
                    head.direction,
                )]
            }
            MirrorInc => {
                let direction = if head.direction.x != 0 {
                    left90.rotate(head.direction)
                } else {
                    right90.rotate(head.direction)
                };
                vec![BeamHead::new(head.position + direction, direction)]
            }
            MirrorDec => {
                let direction = if head.direction.x != 0 {
                    right90.rotate(head.direction)
                } else {
                    left90.rotate(head.direction)
                };
                vec![BeamHead::new(head.position + direction, direction)]
            }
            SplitterHor => {
                if head.direction.x != 0 {
                    vec![BeamHead::new(
                        head.position + head.direction,
                        head.direction,
                    )]
                } else {
                    vec![
                        BeamHead::new(head.position + IVec2::new(-1, 0), IVec2::new(-1, 0)),
                        BeamHead::new(head.position + IVec2::new(1, 0), IVec2::new(1, 0)),
                    ]
                }
            }
            SplitterVer => {
                if head.direction.y != 0 {
                    vec![BeamHead::new(
                        head.position + head.direction,
                        head.direction,
                    )]
                } else {
                    vec![
                        BeamHead::new(head.position + IVec2::new(0, -1), IVec2::new(0, -1)),
                        BeamHead::new(head.position + IVec2::new(0, 1), IVec2::new(0, 1)),
                    ]
                }
            }
        }
    } else {
        vec![]
    }
}

fn _draw_layout(visited: &HashSet<IVec2>, grid: Grid) {
    for y in 0..grid.height {
        for x in 0..grid.width {
            let pos = IVec2::new(x, y);
            if visited.contains(&pos) {
                print!("#");
            } else {
                let elem = grid.elements.get(&pos).unwrap();
                print!("{elem}");
            }
        }
        println!();
    }
    println!("!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!");
}
