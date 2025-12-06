#[derive(Debug, Clone)]
pub enum Direction {
    Left,
    Right,
}

#[derive(Debug, Clone)]
pub struct Rotation {
    direction: Direction,
    count: usize,
}

#[derive(Debug, Clone)]
pub struct Input {
    rotations: Vec<Rotation>,
}

pub fn parse(input: &str) -> Input {
    Input {
        rotations: input
            .lines()
            .map(|line| {
                let (first, rest) = line.split_at(1);
                let direction = match first {
                    "L" => Direction::Left,
                    "R" => Direction::Right,
                    _ => panic!("unknown direction {first}"),
                };
                let count = rest.parse::<usize>().expect("invalid number");
                Rotation { direction, count }
            })
            .collect(),
    }
}

pub fn part_1(input: &Input) -> crate::Output {
    let mut zero_count: usize = 0;
    let mut value: usize = 50;
    for op in &input.rotations {
        value = match op.direction {
            Direction::Left => (value + 100 - (op.count % 100)) % 100,
            Direction::Right => (value + op.count) % 100,
        };
        if value == 0 {
            zero_count += 1;
        }
    }
    zero_count.into()
}

pub fn part_2(input: &Input) -> crate::Output {
    let mut zero_count: usize = 0;
    let mut value: usize = 50;
    for op in &input.rotations {
        match op.direction {
            Direction::Left => {
                let mut count = op.count;
                while count > 0 {
                    count -= 1;
                    if value == 0 {
                        value = 99;
                    } else {
                        value -= 1;
                    }
                    if value == 0 {
                        zero_count += 1;
                    }
                }
            }
            Direction::Right => {
                let mut count = op.count;
                while count > 0 {
                    count -= 1;
                    if value == 99 {
                        value = 0;
                    } else {
                        value += 1;
                    }
                    if value == 0 {
                        zero_count += 1;
                    }
                }
            }
        }
    }
    zero_count.into()
}

#[cfg(test)]
pub const EXAMPLE_RESULTS: [Option<crate::Output>; 2] =
    [Some(crate::Output::Int(3)), Some(crate::Output::Int(6))];
