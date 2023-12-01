fn main() {
    let input = std::fs::read_to_string("./input").unwrap();

    struct FirstLast {
        first: Option<u32>,
        last: Option<u32>,
    }

    impl FirstLast {
        fn new() -> Self {
            Self {
                first: None,
                last: None,
            }
        }

        fn record(&mut self, n: u32) {
            if self.first.is_none() {
                self.first = Some(n)
            } else {
                self.last = Some(n)
            }
        }

        fn value(self) -> u32 {
            self.first.unwrap() * 10 + self.last.unwrap_or(self.first.unwrap())
        }
    }

    let out: u32 = input
        .lines()
        .map(|line| {
            line.chars().fold(FirstLast::new(), |mut val, char| {
                if let Some(digit) = char.to_digit(10) {
                    val.record(digit)
                }
                val
            })
        })
        .map(FirstLast::value)
        .sum();

    println!("{:?}", out);
}
