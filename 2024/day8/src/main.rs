mod helpers;
mod output;

use helpers::{Grid, Point};
use output::Output;

use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Location {
    Empty,
    Antenna(char),
}

impl From<char> for Location {
    fn from(value: char) -> Self {
        match value {
            'a'..='z' | 'A'..='Z' | '0'..='9' => Self::Antenna(value),
            '.' => Self::Empty,
            _ => panic!("invalid input"),
        }
    }
}

impl std::fmt::Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => write!(f, "."),
            Self::Antenna(c) => write!(f, "{c}"),
        }
    }
}

#[derive(Debug, Clone)]
struct Input {
    grid: Grid<Location>,
}

fn parse(input: &str) -> Input {
    Input {
        grid: Grid::from_str(input),
    }
}

fn get_antinodes_at_mul(
    grid: &Grid<Location>,
    range: &(impl Iterator<Item = isize> + Clone),
) -> Vec<Point> {
    let mut locs: HashMap<char, Vec<Point>> = HashMap::new();

    for (point, loc) in grid.iter() {
        if let Location::Antenna(c) = loc {
            locs.entry(c)
                .and_modify(|v| v.push(point.clone()))
                .or_insert_with(|| vec![point.clone()]);
        }
    }

    let mut locs_with_antenna: Vec<Point> = Vec::new();

    let mut record = |point: Point| {
        if !locs_with_antenna.iter().any(|l| *l == point) {
            locs_with_antenna.push(point);
        };
    };

    for antenna_type in locs.keys() {
        let locations = locs.get(antenna_type).unwrap();
        for i in 0..locations.len() {
            for j in (i + 1)..locations.len() {
                let l1 = &locations[i];
                let l2 = &locations[j];

                let v = l1.vector_to(&l2);

                for i in range.clone() {
                    if let Some(p) = l1.add(v.rev().mul(i)) {
                        if !grid.contains(&p) {
                            break;
                        }
                        record(p);
                    } else {
                        break;
                    }
                }

                for i in range.clone() {
                    if let Some(p) = l2.add(v.mul(i)) {
                        if !grid.contains(&p) {
                            break;
                        }
                        record(p);
                    } else {
                        break;
                    }
                }
            }
        }
    }

    locs_with_antenna
}

fn part_1(input: &Input) -> Output {
    get_antinodes_at_mul(&input.grid, &std::iter::once(1))
        .len()
        .into()
}

fn part_2(input: &Input) -> Output {
    get_antinodes_at_mul(&input.grid, &(0..)).len().into()
}

fn main() {
    let input = parse(&std::fs::read_to_string("input").expect("input could not be read"));

    match std::env::args().nth(1) {
        Some(s) if s == "1" => println!("part 1: {}", part_1(&input)),
        Some(s) if s == "2" => println!("part 2: {}", part_2(&input)),
        _ => panic!("specify part"),
    }
}

#[cfg(test)]
mod test {
    const EXAMPLE_RESULT_PART_1: Output = Output::Int(14);
    const EXAMPLE_RESULT_PART_2: Output = Output::Int(34);

    use super::*;

    fn example_input() -> Input {
        parse(&std::fs::read_to_string("example").unwrap())
    }

    #[test]
    fn example_part_1() {
        assert_eq!(part_1(&example_input()), EXAMPLE_RESULT_PART_1);
    }

    #[test]
    fn example_part_2() {
        assert_eq!(part_2(&example_input()), EXAMPLE_RESULT_PART_2);
    }
}
