use std::collections::HashMap;

use crate::{helpers, Output};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Stone(usize);

#[derive(Debug, Clone)]
pub struct Input {
    stones: Vec<Stone>,
}

pub fn parse(input: &str) -> Input {
    Input {
        stones: input
            .split_whitespace()
            .map(|s| Stone(s.parse::<usize>().unwrap()))
            .collect(),
    }
}

fn split_number(num: usize) -> [usize; 2] {
    let digits = helpers::number_of_digits(num);
    assert!(digits % 2 == 0);
    let left = num / (10_usize.pow(digits / 2));
    let right = num - (left * 10_usize.pow(digits / 2));
    [left, right]
}

fn step(stones: Vec<Stone>) -> Vec<Stone> {
    let mut result = Vec::new();
    for stone in stones {
        match stone.0 {
            0 => result.push(Stone(1)),
            _ => {
                if helpers::number_of_digits(stone.0) % 2 == 0 {
                    result.extend(split_number(stone.0).into_iter().map(Stone));
                } else {
                    result.push(Stone(stone.0 * 2024));
                }
            }
        }
    }
    result
}

fn handle_stone<const UNTIL: u8>(
    stone: Stone,
    step: u8,
    cache: &mut HashMap<(Stone, u8), usize>,
) -> usize {
    if let Some(v) = cache.get(&(stone, step)) {
        *v
    } else {
        let result = if step == UNTIL {
            1
        } else {
            match stone.0 {
                0 => handle_stone::<UNTIL>(Stone(1), step + 1, cache),
                _ => {
                    if helpers::number_of_digits(stone.0) % 2 == 0 {
                        split_number(stone.0)
                            .into_iter()
                            .map(|s| handle_stone::<UNTIL>(Stone(s), step + 1, cache))
                            .sum()
                    } else {
                        handle_stone::<UNTIL>(Stone(stone.0 * 2024), step + 1, cache)
                    }
                }
            }
        };
        assert!(cache.insert((stone, step), result).is_none());
        result
    }
}

pub fn part_1(input: &Input) -> Output {
    let mut stones = input.stones.clone();
    for _step in 0..25 {
        stones = step(stones);
    }
    stones.len().into()
}

pub fn part_2(input: &Input) -> Output {
    let mut cache: HashMap<(Stone, u8), usize> = HashMap::new();

    input
        .stones
        .iter()
        .map(|stone| handle_stone::<75>(*stone, 0, &mut cache))
        .sum::<usize>()
        .into()
}

#[cfg(test)]
pub const EXAMPLE_RESULTS: [Option<Output>; 2] = [Some(Output::Int(55312)), None];
