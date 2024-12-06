mod helpers;
mod output;
use output::Output;

#[derive(Debug, Clone, Copy)]
enum Spring {
    Good,
    Damanged,
    Unknown,
}

impl From<char> for Spring {
    fn from(value: char) -> Self {
        match value {
            '?' => Self::Unknown,
            '.' => Self::Good,
            '#' => Self::Damanged,
            _ => panic!("unknown spring char"),
        }
    }
}

#[derive(Debug, Clone)]
struct Input(Vec<(Vec<Spring>, Vec<usize>)>);

fn parse(input: &str) -> Input {
    Input(
        input
            .lines()
            .map(|line| {
                let (springs, info) = helpers::parse_into_two_fields::<_, _, ' '>(input);
                springs.chars().map(|c| c.into())
            })
            .collect(),
    )
}

fn part_1(input: &Input) -> Output {
    0.into()
}

fn part_2(input: &Input) -> Output {
    0.into()
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
    const EXAMPLE_RESULT_PART_1: Output = Output::Int(1);
    const EXAMPLE_RESULT_PART_2: Output = Output::Int(1);

    use super::*;

    fn example_input() -> Input {
        parse(&std::fs::read_to_string("example").unwrap())
    }

    #[test]
    fn example_part_1() {
        assert_eq!(part_1(&example_input()), EXAMPLE_RESULT_PART_1)
    }

    #[test]
    fn example_part_2() {
        assert_eq!(part_2(&example_input()), EXAMPLE_RESULT_PART_2)
    }
}
