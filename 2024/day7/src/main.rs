mod helpers;
mod output;
use output::Output;

#[derive(Debug, Clone)]
struct Input {
    equations: Vec<(usize, Vec<usize>)>,
}

#[derive(Debug, Clone, Copy)]
enum Operator {
    Mul,
    Sum,
    Concat,
}

fn parse(input: &str) -> Input {
    Input {
        equations: input
            .lines()
            .map(|line| {
                let (result, values) = line.split_once(':').unwrap();
                (
                    result.parse::<usize>().unwrap(),
                    values
                        .split_whitespace()
                        .map(|v| v.parse::<usize>().unwrap())
                        .collect(),
                )
            })
            .collect(),
    }
}

fn generate_permutations<T>(len: usize, from: &[T]) -> Vec<Vec<T>>
where
    T: Copy,
{
    let mut result = Vec::new();
    if len == 1 {
        for f in from {
            result.push(vec![*f]);
        }
    } else {
        for f in from {
            for mut perm in generate_permutations(len - 1, from) {
                let mut state = vec![*f];
                state.append(&mut perm);
                result.push(state);
            }
        }
    }
    result
}

fn compute(values: &[usize], operators: &[Operator], result: usize) -> bool {
    let mut acc = values[0];
    for i in 1..values.len() {
        match operators[i - 1] {
            Operator::Mul => acc *= values[i],
            Operator::Sum => acc += values[i],
            Operator::Concat => {
                let value_to_append = values[i];
                acc *= 10_usize.pow(value_to_append.ilog(10) + 1);
                acc += value_to_append;
            }
        }
        if acc > result {
            return false;
        }
    }
    acc == result
}

fn with_operators(equations: &[(usize, Vec<usize>)], operators: &[Operator]) -> usize {
    equations
        .iter()
        .filter_map(|equation| {
            let operator_count = equation.1.len() - 1;
            let permutations = generate_permutations(operator_count, operators);

            for perm in &permutations {
                if compute(&equation.1, perm, equation.0) {
                    return Some(equation.0);
                }
            }
            None
        })
        .sum()
}

fn part_1(input: &Input) -> Output {
    with_operators(&input.equations, &[Operator::Mul, Operator::Sum]).into()
}

fn part_2(input: &Input) -> Output {
    with_operators(
        &input.equations,
        &[Operator::Mul, Operator::Sum, Operator::Concat],
    )
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
    const EXAMPLE_RESULT_PART_1: Output = Output::Int(3749);
    const EXAMPLE_RESULT_PART_2: Output = Output::Int(11387);

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
