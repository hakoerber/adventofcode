use crate::helpers::{Point, Vector};

use std::io::{self, Write as _};

#[derive(Debug, Clone)]
pub struct Robot {
    position: Point,
    velocity: Vector,
}

impl Robot {
    fn step(&mut self, steps: usize, width: usize, height: usize) {
        let new_position = self
            .position
            .add_wrapping(
                &self.velocity.mul(isize::try_from(steps).unwrap()),
                width,
                height,
            )
            .unwrap();

        self.position = new_position
    }
}

#[derive(Debug, Clone)]
pub struct Input {
    robots: Vec<Robot>,
}

pub fn parse(input: &str) -> Input {
    Input {
        robots: input
            .lines()
            .map(|line| {
                let fields = line.split_once(' ').unwrap();
                Robot {
                    position: {
                        let fields = fields.0.split_once('=').unwrap().1.split_once(',').unwrap();
                        Point {
                            x: fields.0.parse::<usize>().unwrap(),
                            y: fields.1.parse::<usize>().unwrap(),
                        }
                    },

                    velocity: {
                        let fields = fields.1.split_once('=').unwrap().1.split_once(',').unwrap();
                        Vector {
                            x: fields.0.parse::<isize>().unwrap(),
                            y: fields.1.parse::<isize>().unwrap(),
                        }
                    },
                }
            })
            .collect(),
    }
}

pub fn part_1(input: &Input, width: usize, height: usize) -> crate::Output {
    let width_middle = width / 2;
    let height_middle = height / 2;

    let mut robots = input.robots.clone();

    robots
        .iter_mut()
        .for_each(|robot| robot.step(100, width, height));

    let counts_per_quadrant = robots.iter().fold((0, 0, 0, 0), |mut acc, robot| {
        if robot.position.x < width_middle && robot.position.y < height_middle {
            acc.0 += 1;
        } else if robot.position.x < width_middle && robot.position.y > height_middle {
            acc.1 += 1;
        } else if robot.position.x > width_middle && robot.position.y < height_middle {
            acc.2 += 1;
        } else if robot.position.x > width_middle && robot.position.y > height_middle {
            acc.3 += 1;
        }
        acc
    });

    (counts_per_quadrant.0 * counts_per_quadrant.1 * counts_per_quadrant.2 * counts_per_quadrant.3)
        .into()
}

pub fn part_2(input: &Input, width: usize, height: usize) -> crate::Output {
    let mut robots = input.robots.clone();
    for i in 1..=(101 * 103) {
        robots
            .iter_mut()
            .for_each(|robot| robot.step(1, width, height));
        if !detect_line(&robots) {
            continue;
        }
        display_grid(width, height, &robots);
        println!("{i}");
        return crate::Output::empty();
    }
    panic!("no suitable grid found")
}

fn detect_line(robots: &[Robot]) -> bool {
    robots.iter().any(|robot| {
        let mut has_line = true;
        for i in 1..=10 {
            if let Some(pos) = robot.position.add(&Vector { x: i, y: 0 }) {
                if !robots.iter().any(|other| pos == other.position) {
                    has_line = false;
                    break;
                }
            }
        }

        has_line
    })
}

fn display_grid(width: usize, height: usize, robots: &[Robot]) {
    let mut stdout = io::stdout();
    for y in 0..height {
        for x in 0..width {
            if robots
                .iter()
                .any(|robot| robot.position.x == x && robot.position.y == y)
            {
                write!(stdout, "X").unwrap();
            } else {
                write!(stdout, ".").unwrap();
            }
        }
        writeln!(stdout).unwrap();
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn example_input() -> Input {
        parse(&std::fs::read_to_string("example").unwrap())
    }

    #[test]
    fn example_part_1() {
        assert_eq!(part_1(&example_input(), 11, 7), 12.into());
    }
}
