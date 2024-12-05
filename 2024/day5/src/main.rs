mod helpers;
mod output;
use output::Output;

#[derive(Debug, Clone)]
struct Input {
    orderings: Vec<(usize, usize)>,
    updates: Vec<Vec<usize>>,
}

fn parse(input: &str) -> Input {
    let split = input.split("\n\n").collect::<Vec<&str>>();
    assert_eq!(split.len(), 2);
    let (orderings, updates) = (split[0], split[1]);

    let orderings = orderings
        .lines()
        .map(|line| helpers::parse_into_fields::<usize, 2, '|'>(line).into())
        .collect();

    let updates = updates
        .lines()
        .map(|line| {
            let update = helpers::parse_into_vec::<usize, ','>(line);
            assert!(update.len() % 2 == 1);
            update
        })
        .collect();

    Input { orderings, updates }
}

fn is_valid(update: &[usize], orderings: &[(usize, usize)]) -> bool {
    for (i, elem) in update.iter().enumerate() {
        let before = &update[0..i];
        let after = &update[i + 1..update.len()];

        for before in before {
            // look for (elem, before), which would break the ordering
            if orderings.iter().any(|o| *o == (*elem, *before)) {
                return false;
            }
        }

        for after in after {
            // look for (after, elem), which would break the ordering
            if orderings.iter().any(|o| *o == (*after, *elem)) {
                return false;
            }
        }
    }
    true
}

fn fix_ordering(update: &[usize], orderings: &[(usize, usize)]) -> Vec<usize> {
    //! It's effectively a sort operations with a custom comparison. Effectiely, this is
    //! selection sort. ! We iterate over the elements of update, and always
    //! look for the one that we can yield. ! An element can be yielded if
    //! there is no remaining element (to its right) that is "smaller".
    //!
    //! Implemented in-place.
    let mut update = update.to_vec();

    for i in 0..update.len() {
        for c in i..update.len() {
            if update[(c + 1)..update.len()]
                .iter()
                .any(|rest| orderings.iter().any(|o| *o == (*rest, update[c])))
            {
                continue;
            }
            update.swap(i, c);
            break;
        }
    }

    // new_ordering;
    update
}

fn part_1(input: &Input) -> Output {
    input
        .updates
        .iter()
        .filter(|u| is_valid(u, &input.orderings))
        .map(|u| u[u.len() / 2])
        .sum::<usize>()
        .into()
}

fn part_2(input: &Input) -> Output {
    input
        .updates
        .iter()
        .filter(|u| !is_valid(u, &input.orderings))
        .map(|u| fix_ordering(u, &input.orderings))
        .map(|u| u[u.len() / 2])
        .sum::<usize>()
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
    const EXAMPLE_RESULT_PART_1: Output = Output::Int(143);
    const EXAMPLE_RESULT_PART_2: Output = Output::Int(123);

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
