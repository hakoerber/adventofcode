mod helpers;
mod output;
mod puzzle;

pub use output::Output;

fn main() {
    let input = puzzle::parse(&std::fs::read_to_string("input").expect("input could not be read"));

    match std::env::args().nth(1) {
        Some(s) if s == "1" => println!("part 1: {}", puzzle::part_1(&input, 101, 103)),
        Some(s) if s == "2" => {
            puzzle::part_2(&input, 101, 103);
        }
        _ => panic!("specify part"),
    }
}
