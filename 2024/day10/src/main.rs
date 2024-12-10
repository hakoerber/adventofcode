mod helpers;
mod output;

use helpers::{Grid, Path, Point};
use output::Output;

#[derive(Debug, Clone)]
struct Input {
    grid: Grid<u8>,
}

fn parse(input: &str) -> Input {
    Input {
        grid: Grid::from_str_as_digits(input),
    }
}

// find all paths from that point, including the point itself
fn path_from(grid: &Grid<u8>, point: &Point, value: u8) -> impl Iterator<Item = Path> {
    if value == 9 {
        vec![Path(vec![point.clone()])]
    } else {
        let mut next_paths: Vec<Path> = Vec::new();
        for p in grid.neighbors_of(point) {
            if *p.value == value + 1 {
                next_paths.extend(path_from(grid, &p.point, value + 1).map(|mut path| {
                    path.0.insert(0, point.clone());
                    path
                }));
            }
        }
        next_paths
    }
    .into_iter()
}

fn trailheads(grid: &Grid<u8>) -> impl Iterator<Item = Point> + use<'_> {
    grid.iter()
        .filter_map(|(point, value)| (value == 0).then_some(point))
}

fn part_1(input: &Input) -> Output {
    trailheads(&input.grid)
        .map(|trailhead| {
            helpers::unique(
                path_from(&input.grid, &trailhead, 0).map(|trail| trail.0.last().unwrap().clone()),
            )
            .len()
        })
        .sum::<usize>()
        .into()
}

fn part_2(input: &Input) -> Output {
    trailheads(&input.grid)
        .map(|trailhead| path_from(&input.grid, &trailhead, 0).count())
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
