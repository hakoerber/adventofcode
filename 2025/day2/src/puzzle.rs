use std::ops::RangeInclusive;

#[derive(Debug, Clone)]
pub struct Input {
    ranges: Vec<RangeInclusive<usize>>,
}

pub fn parse(input: &str) -> Input {
    Input {
        ranges: input
            .trim()
            .split(',')
            .map(|range| {
                let (start, end) = range.split_once('-').unwrap();
                let start: usize = start.parse().unwrap();
                let end: usize = end.parse().unwrap();
                start..=end
            })
            .collect(),
    }
}

fn has_digit_pattern_twice(value: usize) -> bool {
    let digits: Vec<u8> = value
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap().try_into().unwrap())
        .collect();

    if digits.len().is_multiple_of(2) {
        let mid = digits.len() / 2;
        let (first_half, second_half) = (&digits[0..mid], &digits[mid..]);
        first_half == second_half
    } else {
        false
    }
}

fn has_repeating_digit_pattern(value: usize) -> bool {
    let digits: Vec<u8> = value
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap().try_into().unwrap())
        .collect();

    if digits.len() != 1 {
        for i in 0..digits.len().div_euclid(2) {
            let chunks: Vec<&[u8]> = digits.chunks(i + 1).collect();

            if chunks.iter().skip(1).all(|chunk| *chunk == chunks[0]) {
                return true;
            }
        }
    }

    false
}

fn find_twice_digit_patterns(range: RangeInclusive<usize>) -> Vec<usize> {
    range.filter(|i| has_digit_pattern_twice(*i)).collect()
}

fn find_repeating_digit_patterns(range: RangeInclusive<usize>) -> Vec<usize> {
    range.filter(|i| has_repeating_digit_pattern(*i)).collect()
}

pub fn part_1(input: &Input) -> crate::Output {
    input
        .ranges
        .iter()
        .flat_map(|range| find_twice_digit_patterns(range.clone()))
        .sum::<usize>()
        .into()
}

pub fn part_2(input: &Input) -> crate::Output {
    input
        .ranges
        .iter()
        .flat_map(|range| find_repeating_digit_patterns(range.clone()))
        .sum::<usize>()
        .into()
}

#[cfg(test)]
pub const EXAMPLE_RESULTS: [Option<crate::Output>; 2] = [
    Some(crate::Output::Int(1227775554)),
    Some(crate::Output::Int(4174379265)),
];

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn digit_patterns_01() {
        assert_eq!(find_twice_digit_patterns(11..=22), [11, 22]);
        assert_eq!(find_twice_digit_patterns(998..=1012), [1010]);
    }

    #[test]
    fn digit_patterns_02() {
        assert_eq!(find_repeating_digit_patterns(11..=22), [11, 22]);
        assert_eq!(find_repeating_digit_patterns(998..=1012), [999, 1010]);
        assert_eq!(find_repeating_digit_patterns(3..=22), [11, 22]);
    }
}
