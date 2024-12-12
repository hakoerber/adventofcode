use super::helpers;

#[derive(Debug, Clone)]
pub enum Field {
    Operational,
    Damaged,
    Unknown,
}

impl From<char> for Field {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::Operational,
            '#' => Self::Damaged,
            '?' => Self::Unknown,
            _ => panic!("invalid input"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Input {
    lines: Vec<(Vec<Field>, Vec<usize>)>,
}

pub fn parse(input: &str) -> Input {
    Input {
        lines: input
            .lines()
            .map(|line| {
                let split: Vec<&str> = line.split_whitespace().collect();
                assert_eq!(split.len(), 2);
                let fields: Vec<Field> = split[0].chars().map(|c| c.into()).collect();
                let groups: Vec<usize> = split[1]
                    .split(',')
                    .map(|s| s.parse::<usize>().unwrap())
                    .collect();
                (fields, groups)
            })
            .collect(),
    }
}

pub fn part_1(input: &Input) -> crate::Output {
    println!("{input:?}");
    input
        .lines
        .iter()
        .map(|(fields, groups)| number_of_solutions(fields, groups))
        .sum::<usize>()
        .into()
}

fn number_of_solutions(fields: &[Field], groups: &[usize]) -> usize {
    0
}

pub fn part_2(input: &Input) -> crate::Output {
    0.into()
}

#[cfg(test)]
pub const EXAMPLE_RESULTS: [Option<crate::Output>; 2] =
    [Some(crate::Output::Int(21)), Some(crate::Output::Int(1))];
