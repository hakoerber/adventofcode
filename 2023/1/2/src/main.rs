struct NumberPair(Option<u32>, Option<u32>);

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

fn try_parse_at(input: &[char]) -> Option<u32> {
    if let Some(digit) = input[0].to_digit(10) {
        Some(digit)
    } else {
        SpelledOutNumber::parse(&input[0..input.len()]).map(|s| s.0)
    }
}

fn main() {
    let input = std::fs::read_to_string("./input").unwrap();

    let approaches: Vec<Box<dyn Fn(&[char]) -> NumberPair>> = vec![
        // go through the string one by one, check the value at that position and record it into
        // a `FirstLast` recorder that holds state
        Box::new(|line| {
            #[derive(Debug)]
            struct Recorder {
                first: Option<u32>,
                last: Option<u32>,
            }

            impl Recorder {
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

                fn finish(self) -> NumberPair {
                    NumberPair(self.first, self.last)
                }
            }

            let mut recorder = Recorder::new();
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
            recorder.finish()
        }),
        //
        // Go through the string one by one, transform it into an array of digits and go from there
        // I prefer this approach, as there is no stateful iteration and it's very easy to understand
        Box::new(|line| {
            let result = (0..line.len())
                .map(move |pos| try_parse_at(&line[pos..line.len()]))
                // remove none values
                .filter_map(|e| e)
                .collect::<Vec<u32>>();
            // peculiar: if there is only one digit, use it for both the tenths digit and and ones digit
            NumberPair(result.get(0).copied(), result.last().copied())
        }),
        //
        // this one does two scans, one from each end. it's elegant because it does not require special
        // handling for lines containing only one digit, i.e. it will *always* return (Some, Some)
        Box::new(|line| {
            NumberPair(
                {
                    let mut tenth = None;
                    for pos in 0..line.len() {
                        if let Some(digit) = try_parse_at(&line[pos..line.len()]) {
                            tenth = Some(digit);
                            break;
                        }
                    }
                    tenth
                },
                {
                    let mut ones = None;

                    for pos in (0..line.len()).rev() {
                        if let Some(digit) = try_parse_at(&line[pos..line.len()]) {
                            ones = Some(digit);
                            break;
                        }
                    }
                    ones
                },
            )
        }),
    ];

    let result: u32 = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .map(|line| {
            (
                approaches
                    .iter()
                    .map(|approach| approach(&line))
                    .map(|pair| {
                        (
                            // we assume that there will *always* at least be one numbers in there
                            pair.0.unwrap(),
                            // if there is no second number, we "reuse" the first. so "7" => 77
                            pair.1.or(pair.0).unwrap(),
                        )
                    })
                    .collect::<Vec<(u32, u32)>>(),
                line,
            )
        })
        .map(|(results, line)| {
            // check that all approaches result in the same result
            let result = results[0];
            let mut all_agree = true;
            for other_result in &results[1..] {
                if *other_result != result {
                    all_agree = false;
                }
            }

            if !all_agree {
                eprintln!("approaches yield different results!");
                eprintln!("input: {}", line.iter().collect::<String>());
                eprintln!("results: {results:?}");
            }

            results[0]
        })
        .map(|(tenth, ones)| tenth * 10 + ones)
        .sum();

    println!("{result}");
}
