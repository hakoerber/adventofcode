mod helpers;
mod output;
use std::collections::HashMap;

use output::Output;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Point {
    x: isize,
    y: isize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Location {
    Empty,
    Antenna(char),
}

#[derive(Debug, Clone)]
struct Input {
    grid: Vec<Vec<Location>>,
}

fn parse(input: &str) -> Input {
    Input {
        grid: input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| match c {
                        'a'..='z' | 'A'..='Z' | '0'..='9' => Location::Antenna(c),
                        '.' => Location::Empty,
                        _ => panic!("invalid input"),
                    })
                    .collect()
            })
            .collect(),
    }
}

fn part_1(input: &Input) -> Output {
    let mut locs: HashMap<char, Vec<Point>> = HashMap::new();

    for (y, line) in input.grid.iter().enumerate() {
        for (x, loc) in line.iter().enumerate() {
            if let Location::Antenna(c) = loc {
                let point = Point {
                    x: x as isize,
                    y: y as isize,
                };
                locs.entry(*c)
                    .and_modify(|v| v.push(point))
                    .or_insert(vec![point]);
            }
        }
    }

    let mut locs_with_antenna: Vec<Point> = Vec::new();

    let mut record = |point: Point| {
        if point.x < 0
            || point.y < 0
            || point.y >= input.grid.len() as isize
            || point.x >= input.grid[0].len() as isize
        {
            return;
        }
        if !locs_with_antenna.iter().any(|l| *l == point) {
            locs_with_antenna.push(point);
        };
    };

    for antenna_type in locs.keys() {
        let locations = locs.get(antenna_type).unwrap();
        for i in 0..locations.len() {
            for j in (i + 1)..locations.len() {
                let l1 = locations[i];
                let l2 = locations[j];

                let diff_x = l1.x.abs_diff(l2.x) as isize;
                let diff_y = l1.y.abs_diff(l2.y) as isize;

                match (l1.x < l2.x, l1.y < l2.y) {
                    // 1
                    //   2
                    (true, true) => {
                        let antinode1 = Point {
                            x: l2.x + diff_x,
                            y: l2.y + diff_y,
                        };
                        record(antinode1);

                        let antinode2 = Point {
                            x: l1.x - diff_x,
                            y: l1.y - diff_y,
                        };
                        record(antinode2);
                    }
                    //   2
                    // 1
                    (true, false) => {
                        let antinode1 = Point {
                            x: l2.x + diff_x,
                            y: l2.y - diff_y,
                        };
                        record(antinode1);

                        let antinode2 = Point {
                            x: l1.x - diff_x,
                            y: l1.y + diff_y,
                        };
                        record(antinode2);
                    }
                    //   1
                    // 2
                    (false, true) => {
                        let antinode1 = Point {
                            x: l2.x - diff_x,
                            y: l2.y + diff_y,
                        };
                        record(antinode1);

                        let antinode2 = Point {
                            x: l1.x + diff_x,
                            y: l1.y - diff_y,
                        };
                        record(antinode2);
                    }
                    // 2
                    //   1
                    (false, false) => {
                        let antinode1 = Point {
                            x: l2.x - diff_x,
                            y: l2.y - diff_y,
                        };
                        record(antinode1);

                        let antinode2 = Point {
                            x: l1.x + diff_x,
                            y: l1.y + diff_y,
                        };
                        record(antinode2);
                    }
                }
            }
        }
    }

    locs_with_antenna.len().into()
}

fn part_2(input: &Input) -> Output {
    let mut locs: HashMap<char, Vec<Point>> = HashMap::new();

    for (y, line) in input.grid.iter().enumerate() {
        for (x, loc) in line.iter().enumerate() {
            if let Location::Antenna(c) = loc {
                let point = Point {
                    x: x as isize,
                    y: y as isize,
                };
                locs.entry(*c)
                    .and_modify(|v| v.push(point))
                    .or_insert(vec![point]);
            }
        }
    }

    let mut locs_with_antenna: Vec<Point> = Vec::new();

    let mut record = |point: Point| -> bool {
        if point.x < 0
            || point.y < 0
            || point.y >= input.grid.len() as isize
            || point.x >= input.grid[0].len() as isize
        {
            return false;
        }
        if !locs_with_antenna.iter().any(|l| *l == point) {
            locs_with_antenna.push(point);
        };
        true
    };

    for antenna_type in locs.keys() {
        let locations = locs.get(antenna_type).unwrap();
        for i in 0..locations.len() {
            for j in (i + 1)..locations.len() {
                let l1 = locations[i];
                let l2 = locations[j];

                let diff_x = l1.x.abs_diff(l2.x) as isize;
                let diff_y = l1.y.abs_diff(l2.y) as isize;

                match (l1.x < l2.x, l1.y < l2.y) {
                    // 1
                    //   2
                    (true, true) => {
                        for i in 0.. {
                            let antinode1 = Point {
                                x: l2.x + i * diff_x,
                                y: l2.y + i * diff_y,
                            };
                            if !record(antinode1) {
                                break;
                            }
                        }

                        for i in 0.. {
                            let antinode1 = Point {
                                x: l2.x - i * diff_x,
                                y: l2.y - i * diff_y,
                            };
                            if !record(antinode1) {
                                break;
                            }
                        }
                    }
                    //   2
                    // 1
                    (true, false) => {
                        for i in 0.. {
                            let antinode1 = Point {
                                x: l2.x + i * diff_x,
                                y: l2.y - i * diff_y,
                            };
                            if !record(antinode1) {
                                break;
                            }
                        }

                        for i in 0.. {
                            let antinode1 = Point {
                                x: l2.x - i * diff_x,
                                y: l2.y + i * diff_y,
                            };
                            if !record(antinode1) {
                                break;
                            }
                        }
                    }
                    //   1
                    // 2
                    (false, true) => {
                        for i in 0.. {
                            let antinode1 = Point {
                                x: l2.x - i * diff_x,
                                y: l2.y + i * diff_y,
                            };
                            if !record(antinode1) {
                                break;
                            }
                        }

                        for i in 0.. {
                            let antinode1 = Point {
                                x: l2.x + i * diff_x,
                                y: l2.y - i * diff_y,
                            };
                            if !record(antinode1) {
                                break;
                            }
                        }
                    }
                    // 2
                    //   1
                    (false, false) => {
                        for i in 0.. {
                            let antinode1 = Point {
                                x: l2.x - i * diff_x,
                                y: l2.y - i * diff_y,
                            };
                            if !record(antinode1) {
                                break;
                            }
                        }

                        for i in 0.. {
                            let antinode1 = Point {
                                x: l2.x + i * diff_x,
                                y: l2.y + i * diff_y,
                            };
                            if !record(antinode1) {
                                break;
                            }
                        }
                    }
                }
            }
        }
    }

    locs_with_antenna.len().into()
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
