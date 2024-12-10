mod helpers;
mod output;

use std::fmt::Display;

use helpers::{Grid, Point, Vector};
use output::Output;

#[derive(Debug, Clone)]
struct Input {
    grid: Grid<u8>,
}

fn parse(input: &str) -> Input {
    Input {
        grid: Grid::from_str_with(input, |c| u8::try_from(c.to_digit(10).unwrap()).unwrap()),
    }
}

struct Path(Vec<Point>);

impl Display for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, point) in self.0.iter().enumerate() {
            write!(f, "({},{})", point.y, point.x)?;
            if i != self.0.len() - 1 {
                write!(f, " -> ")?;
            }
        }
        Ok(())
    }
}

// find all paths from that point, including the point itself
fn path_from(grid: &Grid<u8>, point: &Point, value: u8) -> Vec<Path> {
    if value == 9 {
        vec![Path(vec![point.clone()])]
    } else {
        let mut next_paths: Vec<Path> = Vec::new();
        for (x, y) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
            if let Some(p) = point.add(&Vector { y, x }) {
                if let Some(val) = grid.get(&p) {
                    if *val == value + 1 {
                        let mut paths = path_from(grid, &p, value + 1);
                        paths.iter_mut().for_each(|v| v.0.insert(0, point.clone()));
                        next_paths.append(&mut paths);
                    }
                }
            }
        }
        next_paths
    }
}

fn trailheads(grid: &Grid<u8>) -> impl Iterator<Item = Point> + use<'_> {
    grid.iter()
        .filter_map(|(point, value)| (value == 0).then_some(point))
}

fn part_1(input: &Input) -> Output {
    trailheads(&input.grid)
        .map(|trailhead| {
            helpers::unique(
                path_from(&input.grid, &trailhead, 0)
                    .into_iter()
                    .map(|trail| trail.0.last().unwrap().clone()),
            )
            .len()
        })
        .sum::<usize>()
        .into()
}

fn part_2(input: &Input) -> Output {
    trailheads(&input.grid)
        .map(|trailhead| {
            path_from(&input.grid, &trailhead, 0)
                .into_iter()
                .map(|trail| trail.0.last().unwrap().clone())
                .len()
        })
        .sum::<usize>()
        .into()
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
    const EXAMPLE_RESULT_PART_1: Output = Output::Int(36);
    const EXAMPLE_RESULT_PART_2: Output = Output::Int(81);

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
