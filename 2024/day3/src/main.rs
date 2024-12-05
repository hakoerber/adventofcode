use regex::Regex;

#[derive(Debug, Clone)]
struct Command(usize, usize);

#[derive(Debug, Clone)]
enum Conditional {
    Enable,
    Disable,
}

#[derive(Debug, Clone)]
enum EntryKind {
    Command(Command),
    Conditional(Conditional),
}

#[derive(Debug, Clone)]
struct Entry {
    kind: EntryKind,
    position: usize,
}

#[derive(Debug, Clone)]
struct Input {
    entries: Vec<Entry>,
}

fn parse(input: &str) -> Input {
    let re_command = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();
    let re_cond = Regex::new(r"(do\(\)|don't\(\))").unwrap();

    let entries: Vec<Entry> = re_command
        .captures_iter(input)
        .map(|c| {
            let first = c.get(1).unwrap().as_str().parse::<usize>().unwrap();
            let second = c.get(2).unwrap().as_str().parse::<usize>().unwrap();
            let position = c.get(0).unwrap().start();
            Entry {
                position,
                kind: EntryKind::Command(Command(first, second)),
            }
        })
        .chain(re_cond.captures_iter(input).map(|c| {
            let m = c.get(0).unwrap();
            let position = m.start();
            let kind = match m.as_str() {
                "do()" => Conditional::Enable,
                "don't()" => Conditional::Disable,
                _ => unreachable!(),
            };
            Entry {
                position,
                kind: EntryKind::Conditional(kind),
            }
        }))
        .collect();

    Input { entries }
}

fn part_1(input: &Input) -> usize {
    println!("{input:?}");
    input
        .entries
        .iter()
        .filter_map(|entry| {
            if let Entry {
                kind: EntryKind::Command(Command(first, second)),
                position: _,
            } = entry
            {
                Some(first * second)
            } else {
                None
            }
        })
        .sum()
}

fn part_2(input: &Input) -> usize {
    #[derive(Debug)]
    struct State {
        enabled: bool,
        sum: usize,
    }

    let mut input = input.clone();
    input.entries.sort_by_key(|entry| entry.position);

    let mut state = State {
        enabled: true,
        sum: 0,
    };

    for entry in input.entries {
        match entry.kind {
            EntryKind::Command(command) => {
                if state.enabled {
                    state.sum += command.0 * command.1;
                }
            }
            EntryKind::Conditional(conditional) => {
                state.enabled = match conditional {
                    Conditional::Enable => true,
                    Conditional::Disable => false,
                }
            }
        }
    }

    state.sum
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
    const EXAMPLE_RESULT_PART_1: usize = 161;
    const EXAMPLE_RESULT_PART_2: usize = 48;

    use super::*;

    fn example_input() -> Input {
        parse(&std::fs::read_to_string("example").unwrap())
    }

    fn example_input_2() -> Input {
        parse(&std::fs::read_to_string("example2").unwrap())
    }

    #[test]
    fn example_part_1() {
        assert_eq!(part_1(&example_input()), EXAMPLE_RESULT_PART_1)
    }

    #[test]
    fn example_part_2() {
        assert_eq!(part_2(&example_input_2()), EXAMPLE_RESULT_PART_2)
    }
}
