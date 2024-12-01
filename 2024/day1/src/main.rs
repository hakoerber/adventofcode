use std::collections::HashMap;

#[derive(Clone)]
struct Input {
    left: Vec<usize>,
    right: Vec<usize>,
}

impl From<(Vec<usize>, Vec<usize>)> for Input {
    fn from((left, right): (Vec<usize>, Vec<usize>)) -> Self {
        Self { left, right }
    }
}

fn parse(input: &str) -> Input {
    input
        .lines()
        .filter_map(|line| {
            if line.is_empty() {
                None
            } else {
                let fields: Vec<&str> = line.split_whitespace().collect();
                assert_eq!(fields.len(), 2);
                Some((
                    fields[0].parse::<usize>().expect("invalid number"),
                    fields[1].parse::<usize>().expect("invalid number"),
                ))
            }
        })
        .unzip()
        .into()
}

fn compute_distances(input: &Input) -> usize {
    let input = input.clone();
    let (mut left, mut right) = (input.left, input.right);

    left.sort_unstable();
    right.sort_unstable();

    left.into_iter()
        .zip(right)
        .map(|(l, r)| usize::abs_diff(l, r))
        .sum()
}

fn compute_similarity(input: &Input) -> usize {
    let mut freq: HashMap<usize, usize> = HashMap::new();
    for elem in &input.right {
        freq.entry(*elem).and_modify(|v| *v += 1).or_insert(1);
    }

    input
        .left
        .iter()
        .map(|v| freq.get(v).copied().unwrap_or(0) * v)
        .sum()
}

fn main() {
    let input = parse(
        &std::fs::read_to_string(
            std::env::args()
                .nth(1)
                .expect("pass input file path as first arg"),
        )
        .expect("input could not be read"),
    );

    println!("part 1: {}", compute_distances(&input));
    println!("part 2: {}", compute_similarity(&input));
}

#[cfg(test)]
mod test {
    use super::*;

    fn example_input() -> Input {
        parse(&std::fs::read_to_string("example").unwrap())
    }

    #[test]
    fn example_part_1() {
        assert_eq!(compute_distances(&example_input()), 11)
    }

    #[test]
    fn example_part_2() {
        assert_eq!(compute_similarity(&example_input()), 31)
    }
}
