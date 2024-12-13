use crate::helpers::{whole_div, Point, Vector};

#[derive(Debug, Clone)]
pub struct Machine {
    prize: Point,
    button_a: Vector,
    button_b: Vector,
}

impl Machine {
    fn winning_play(&self) -> Option<(usize, usize)> {
        let (p_x, p_y) = (self.prize.x as isize, self.prize.y as isize);
        let (a_x, a_y) = (self.button_a.x, self.button_a.y);
        let (b_x, b_y) = (self.button_b.x, self.button_b.y);

        let a = whole_div(p_y * b_x - p_x * b_y, a_y * b_x - a_x * b_y)?;
        let b = whole_div(p_x - (a * a_x), b_x)?;

        Some((a as usize, b as usize))
    }
}

#[derive(Debug, Clone)]
pub struct Input {
    machines: Vec<Machine>,
}

pub fn parse(input: &str) -> Input {
    Input {
        machines: input
            .split("\n\n")
            .map(|machine| {
                let lines: Vec<&str> = machine.lines().collect();
                assert_eq!(lines.len(), 3);
                Machine {
                    prize: {
                        let fields: Vec<&str> = lines[2].split_whitespace().collect();
                        assert_eq!(fields.len(), 3);
                        assert_eq!(fields[0], "Prize:");
                        Point {
                            x: fields[1]
                                .split_once("=")
                                .unwrap()
                                .1
                                .trim_end_matches(',')
                                .parse::<usize>()
                                .unwrap(),
                            y: fields[2]
                                .split_once("=")
                                .unwrap()
                                .1
                                .parse::<usize>()
                                .unwrap(),
                        }
                    },
                    button_a: {
                        let fields: Vec<&str> = lines[0].split_whitespace().collect();
                        assert_eq!(fields.len(), 4);
                        assert_eq!((fields[0], fields[1]), ("Button", "A:"));
                        Vector {
                            x: fields[2]
                                .split_once("+")
                                .unwrap()
                                .1
                                .trim_end_matches(',')
                                .parse::<isize>()
                                .unwrap(),
                            y: fields[3]
                                .split_once("+")
                                .unwrap()
                                .1
                                .parse::<isize>()
                                .unwrap(),
                        }
                    },
                    button_b: {
                        let fields: Vec<&str> = lines[1].split_whitespace().collect();
                        assert_eq!(fields.len(), 4);
                        assert_eq!((fields[0], fields[1]), ("Button", "B:"));
                        Vector {
                            x: fields[2]
                                .split_once("+")
                                .unwrap()
                                .1
                                .trim_end_matches(',')
                                .parse::<isize>()
                                .unwrap(),
                            y: fields[3]
                                .split_once("+")
                                .unwrap()
                                .1
                                .parse::<isize>()
                                .unwrap(),
                        }
                    },
                }
            })
            .collect(),
    }
}

pub fn part_1(input: &Input) -> crate::Output {
    input
        .machines
        .iter()
        .filter_map(|machine| {
            machine
                .winning_play()
                .map(|(a, b)| (a as usize * 3 + b as usize))
        })
        .sum::<usize>()
        .into()
}

pub fn part_2(input: &Input) -> crate::Output {
    input
        .clone()
        .machines
        .iter_mut()
        .filter_map(|machine| {
            machine.prize.x += 10000000000000;
            machine.prize.y += 10000000000000;
            machine
                .winning_play()
                .map(|(a, b)| (a as usize * 3 + b as usize))
        })
        .sum::<usize>()
        .into()
}

#[cfg(test)]
pub const EXAMPLE_RESULTS: [Option<crate::Output>; 2] = [Some(crate::Output::Int(480)), None];
