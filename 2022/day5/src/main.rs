#[derive(Debug, Clone)]
struct Crate(char);

#[derive(Debug, Clone)]
struct Move {
    from: usize,
    to: usize,
    count: usize,
}

#[derive(Debug, Clone)]
struct Input {
    stacks: Vec<Vec<Crate>>,
    moves: Vec<Move>,
}

fn parse(input: &str) -> Input {
    let split = input.split("\n\n").collect::<Vec<&str>>();
    assert_eq!(split.len(), 2);

    let (stacks_input, moves_input) = (split[0], split[1]);

    let number_of_stacks = stacks_input
        .lines()
        .rev()
        .nth(1)
        .unwrap()
        .chars()
        .filter(|c| *c == '[')
        .count();

    let mut stacks: Vec<Vec<Crate>> = (0..number_of_stacks).map(|_| Vec::new()).collect();

    for line in stacks_input.lines().rev().skip(1) {
        for (i, stack) in stacks.iter_mut().enumerate().take(number_of_stacks) {
            let crate_id: char = line.chars().nth(i * 4 + 1).unwrap();
            if crate_id == ' ' {
                continue;
            } else {
                stack.push(Crate(crate_id));
            }
        }
    }

    let mut moves = Vec::new();
    for line in moves_input.lines() {
        let split = line.split_whitespace().collect::<Vec<&str>>();
        assert_eq!(split.len(), 6);
        moves.push(Move {
            from: split[3].parse::<usize>().unwrap() - 1,
            to: split[5].parse::<usize>().unwrap() - 1,
            count: split[1].parse::<usize>().unwrap(),
        })
    }

    Input { stacks, moves }
}

fn part_1(input: &Input) -> String {
    let mut stacks = input.stacks.clone();
    for m in &input.moves {
        for _i in 0..m.count {
            let elem = stacks[m.from].pop().unwrap();
            stacks[m.to].push(elem);
        }
    }
    stacks
        .into_iter()
        .map(|stack| stack.last().unwrap().0)
        .collect()
}

fn part_2(input: &Input) -> String {
    let mut stacks = input.stacks.clone();
    for m in &input.moves {
        let mut tmp = Vec::new();
        for _i in 0..m.count {
            let elem = stacks[m.from].pop().unwrap();
            tmp.push(elem);
        }
        for elem in tmp.into_iter().rev() {
            stacks[m.to].push(elem);
        }
    }
    stacks
        .into_iter()
        .map(|stack| stack.last().unwrap().0)
        .collect()
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
    const EXAMPLE_RESULT_PART_1: &str = "CMZ";
    const EXAMPLE_RESULT_PART_2: &str = "MCD";

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
