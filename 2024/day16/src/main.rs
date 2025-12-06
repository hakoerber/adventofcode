mod helpers;
mod output;
mod puzzle;

pub use output::Output;

fn main() {
    let input = puzzle::parse(&std::fs::read_to_string("input").expect("input could not be read"));

    match std::env::args().nth(1) {
        Some(s) if s == "1" => println!("part 1: {}", puzzle::part_1(&input)),
        Some(s) if s == "2" => println!("part 2: {}", puzzle::part_2(&input)),
        Some(s) if s.starts_with("example") => println!("{s}: {}", {
            let input =
                puzzle::parse(&std::fs::read_to_string(&s).expect("input could not be read"));
            println!("{}", puzzle::part_1(&input));
            println!("{}", puzzle::part_2(&input));
            crate::Output::empty()
        }),
        _ => panic!("specify part"),
    }
}

#[cfg(test)]
mod test {
    use super::puzzle;

    fn example_input(num: usize) -> puzzle::Input {
        puzzle::parse(&std::fs::read_to_string(format!("example{num}")).unwrap())
    }

    #[test]
    fn example_part_1() {
        if let Some(outputs) = &puzzle::EXAMPLE_RESULTS[0] {
            for (num, expected) in outputs.iter().enumerate() {
                let output = puzzle::part_1(&example_input(num + 1));
                assert_eq!(output, *expected);
            }
        }
    }

    #[test]
    fn example_part_2() {
        if let Some(outputs) = &puzzle::EXAMPLE_RESULTS[1] {
            for (num, expected) in outputs.iter().enumerate() {
                let output = puzzle::part_2(&example_input(num + 1));
                assert_eq!(output, *expected);
            }
        }
    }
}
