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
    fn parse(value: &[char]) -> Option<Self> {
        let value: String = value.into_iter().collect();
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

fn main() {
    let input = std::fs::read_to_string("./input").unwrap();

    let approaches: Vec<u32> = vec![
        // go through the string one by one, check the value at that position and record it into
        // a `FirstLast` recorder that holds state
        input
            .lines()
            .map(|line| {
                let line: Vec<char> = line.chars().collect();
                let mut recorder = FirstLast::new();
                for i in 0..line.len() {
                    let c = &line[i];
                    if let Some(digit) = c.to_digit(10) {
                        recorder.record(digit)
                    } else {
                        if let Some(digit) = SpelledOutNumber::parse(&line[i..]) {
                            recorder.record(digit.0)
                        }
                    }
                }
                recorder
            })
            .map(FirstLast::value)
            .sum(),
        //
        // go through the string one by one, transform it into an array of digits and go from there
        // i prefer this approach, as there is no stateful iteration
        input
            .lines()
            .map(|line| line.chars().collect::<Vec<char>>())
            .map(|line| {
                (0..line.len())
                    .map(move |pos| {
                        if let Some(digit) = line[pos].to_digit(10) {
                            Some(digit)
                        } else {
                            SpelledOutNumber::parse(&line[pos..line.len()]).map(|s| s.0)
                        }
                    })
                    .filter_map(|e| e)
                    .collect::<Vec<u32>>()
            })
            // peculiar: if there is only one digit, use it for both the tenths digit and and ones digit
            .map(|result| (result[0], *result.last().unwrap_or(&result[0])))
            .map(|(d1, d2)| d1 * 10 + d2)
            .sum(),
    ];

    enum Acc {
        Init,
        Some(u32),
    }

    impl Acc {
        fn unwrap(self) -> u32 {
            match self {
                Self::Init => panic!("Accumulator did not contain a number"),
                Self::Some(val) => val,
            }
        }
    }

    // check that all approaches result in the same result
    let result = approaches
        .iter()
        // try_reduce would be nicer here
        .try_fold(Acc::Init, |acc, value| {
            if let Acc::Some(acc) = acc {
                if acc != *value {
                    return Err("approaches give different results");
                }
            };
            Ok(Acc::Some(*value))
        })
        .unwrap() // check that there were no different results
        .unwrap() // check that there were results at all (cannot fail)
    ;

    println!("{result}");
}
