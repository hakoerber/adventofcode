mod helpers;
mod output;
mod puzzle;

pub use output::Output;

fn main() {
    let input = puzzle::parse(&std::fs::read_to_string("input").expect("input could not be read"));

    match std::env::args().nth(1) {
        Some(s) if s == "1" => println!("part 1: {}", puzzle::part_1(&input)),
        Some(s) if s == "2" => println!("part 2: {}", puzzle::part_2(&input)),
        Some(s) if s == "example2" => println!("example2: {}", {
            let input = puzzle::parse(
                &std::fs::read_to_string("example2").expect("input could not be read"),
            );
            puzzle::part_2(&input)
        }),
        _ => panic!("specify part"),
    }
}

#[cfg(test)]
mod test {
    use super::puzzle;

    fn example_input() -> puzzle::Input {
        puzzle::parse(&std::fs::read_to_string("example").unwrap())
    }

    #[test]
    fn example_part_1() {
        if let Some(output) = &puzzle::EXAMPLE_RESULTS[0] {
            assert_eq!(puzzle::part_1(&example_input()), *output);
        }
    }

    #[test]
    fn example_part_2() {
        if let Some(output) = &puzzle::EXAMPLE_RESULTS[1] {
            assert_eq!(puzzle::part_2(&example_input()), *output);
        }
    }
}
