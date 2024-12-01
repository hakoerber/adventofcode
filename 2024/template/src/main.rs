#[derive(Clone)]
struct Input {}

fn parse(input: &str) -> Input {
    Input {}
}

fn part_1(input: &Input) -> usize {
    0
}

fn part_2(input: &Input) -> usize {
    0
}

fn main() {
    let input = parse(&std::fs::read_to_string("input").expect("input could not be read"));

    println!("part 1: {}", part_1(&input));
    println!("part 2: {}", part_2(&input));
}

#[cfg(test)]
mod test {
    const EXAMPLE_RESULT_PART_1: usize = 1;
    const EXAMPLE_RESULT_PART_2: usize = 1;

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
