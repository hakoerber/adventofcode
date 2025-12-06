#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Battery {
    joltage: u8,
}

#[derive(Debug, Clone)]
pub struct Bank {
    batteries: Vec<Battery>,
}

#[derive(Debug, Clone)]
pub struct Input {
    banks: Vec<Bank>,
}

pub fn parse(input: &str) -> Input {
    Input {
        banks: input
            .lines()
            .map(|line| Bank {
                batteries: line
                    .chars()
                    .map(|c| Battery {
                        joltage: c.to_digit(10).unwrap().try_into().unwrap(),
                    })
                    .collect(),
            })
            .collect(),
    }
}

pub fn part_1(input: &Input) -> crate::Output {
    input
        .banks
        .iter()
        .map(|bank| {
            let first_battery = bank.batteries[0..bank.batteries.len() - 1]
                .iter()
                .max()
                .unwrap();

            let first_index = bank
                .batteries
                .iter()
                .enumerate()
                .find(|(_i, b)| b.joltage == first_battery.joltage)
                .unwrap()
                .0;

            let second_digit = bank.batteries[(first_index + 1)..]
                .iter()
                .max()
                .unwrap()
                .joltage;

            (first_battery.joltage * 10 + second_digit) as usize
        })
        .sum::<usize>()
        .into()
}

pub fn part_2(input: &Input) -> crate::Output {
    input
        .banks
        .iter()
        .map(|bank| {
            let mut acc: usize = 0;
            let len = bank.batteries.len();
            let mut index: usize = 0;

            for digit in 0..12 {
                let battery = bank.batteries[index..=(len - (12 - digit))]
                    .iter()
                    .max()
                    .unwrap();

                let battery_index = bank.batteries[index..=(len - (12 - digit))]
                    .iter()
                    .enumerate()
                    .find(|(_i, b)| b.joltage == battery.joltage)
                    .unwrap()
                    .0
                    + index;

                index = battery_index + 1;

                acc +=
                    battery.joltage as usize * 10_usize.pow(u32::try_from(12 - digit - 1).unwrap());
            }

            acc
        })
        .sum::<usize>()
        .into()
}

#[cfg(test)]
pub const EXAMPLE_RESULTS: [Option<crate::Output>; 2] = [
    Some(crate::Output::Int(357)),
    Some(crate::Output::Int(3121910778619)),
];
