fn main() {
    let input = std::fs::read_to_string("./input").unwrap();

    #[derive(Debug)]
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

    struct SpelledOutNumber(u32);

    impl SpelledOutNumber {
        fn parse(value: &str) -> Option<Self> {
            if value.starts_with("one") {
                Some(Self(1))
            } else if value.starts_with("two") {
                Some(Self(2))
            } else if value.starts_with("three") {
                Some(Self(3))
            } else if value.starts_with("four") {
                Some(Self(4))
            } else if value.starts_with("five") {
                Some(Self(5))
            } else if value.starts_with("six") {
                Some(Self(6))
            } else if value.starts_with("seven") {
                Some(Self(7))
            } else if value.starts_with("eight") {
                Some(Self(8))
            } else if value.starts_with("nine") {
                Some(Self(9))
            } else {
                None
            }
        }
    }

    let out: u32 = input
        .lines()
        .map(|line| {
            let line: Vec<char> = line.chars().collect();
            let mut recorder = FirstLast::new();
            for i in 0..line.len() {
                let c = &line[i];
                if let Some(digit) = c.to_digit(10) {
                    recorder.record(digit)
                } else {
                    let rest = &line[i..].iter().collect::<String>();
                    if let Some(digit) = SpelledOutNumber::parse(rest.as_str()) {
                        recorder.record(digit.0)
                    }
                }
            }
            recorder
        })
        .map(FirstLast::value)
        .sum();

    println!("{out}");
}
