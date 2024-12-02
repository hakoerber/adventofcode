use std::cmp::Ordering;

#[derive(Debug, Clone)]
struct Input(Vec<Vec<usize>>);

fn parse(input: &str) -> Input {
    Input(
        input
            .lines()
            .map(|line| {
                line.split_whitespace()
                    .map(|elem| elem.parse::<usize>().expect("invalid number"))
                    .collect()
            })
            .collect(),
    )
}

fn valid(report: &[usize]) -> bool {
    match report[0].cmp(&report[1]) {
        Ordering::Less => report
            .windows(2)
            .map(|w| w[0] < w[1] && w[1] - w[0] <= 3)
            .all(|e| e),
        Ordering::Greater => report
            .windows(2)
            .map(|w| w[0] > w[1] && w[0] - w[1] <= 3)
            .all(|e| e),
        Ordering::Equal => false,
    }
}

fn part_1(input: &Input) -> usize {
    input
        .0
        .iter()
        .map(|report| valid(report))
        .filter(|e| *e)
        .count()
}

fn part_2(input: &Input) -> usize {
    input
        .0
        .iter()
        .map(|report| {
            let mut is_valid = false;
            for rem in 0..report.len() {
                let mut report = report.clone();
                report.remove(rem);
                if valid(&report) {
                    is_valid = true;
                    break;
                }
            }
            is_valid
        })
        .filter(|e| *e)
        .count()
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
    const EXAMPLE_RESULT_PART_1: usize = 2;
    const EXAMPLE_RESULT_PART_2: usize = 4;

    use super::*;

    fn example_input() -> Input {
        let input = parse(&std::fs::read_to_string("example").unwrap());
        println!("{input:?}");
        input
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
