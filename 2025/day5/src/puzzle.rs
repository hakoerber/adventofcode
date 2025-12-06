use std::ops::RangeInclusive;

#[derive(Debug, Clone)]
pub struct Input {
    fresh_ranges: Vec<RangeInclusive<usize>>,
    available: Vec<usize>,
}

pub fn parse(input: &str) -> Input {
    let mut fresh_ranges = Vec::new();
    let mut available = Vec::new();
    let mut in_ranges = true;

    for line in input.lines() {
        if line.is_empty() {
            in_ranges = false;
            continue;
        }

        if in_ranges {
            let (start, end) = line.split_once('-').unwrap();
            let start: usize = start.parse().unwrap();
            let end: usize = end.parse().unwrap();
            fresh_ranges.push(start..=end);
        } else {
            available.push(line.parse::<usize>().unwrap());
        }
    }

    Input {
        fresh_ranges,
        available,
    }
}

pub fn part_1(input: &Input) -> crate::Output {
    input
        .available
        .iter()
        .filter(|id| input.fresh_ranges.iter().any(|range| range.contains(id)))
        .count()
        .into()
}

fn try_compact_ranges(
    a: &RangeInclusive<usize>,
    b: &RangeInclusive<usize>,
) -> Option<RangeInclusive<usize>> {
    let (a, b) = if a.start() <= b.start() {
        (a, b)
    } else {
        (b, a)
    };

    if a.end() >= b.start() {
        let end = usize::max(*a.end(), *b.end());
        Some((*a.start())..=end)
    } else {
        None
    }
}

fn compact_ranges(ranges: &mut Vec<RangeInclusive<usize>>) {
    let mut i = 0;
    loop {
        if i == ranges.len() - 1 {
            break;
        }
        let curr = &ranges[i];
        let next = &ranges[i + 1];
        if let Some(new_range) = try_compact_ranges(curr, next) {
            ranges[i] = new_range;
            ranges.remove(i + 1);
        } else {
            i += 1;
        }
    }
}

fn dedup_ranges(sorted_ranges: &[RangeInclusive<usize>]) -> Vec<RangeInclusive<usize>> {
    let mut current_start = 0;
    let mut ranges: Vec<RangeInclusive<usize>> = vec![];

    for range in sorted_ranges {
        if current_start == *range.start() {
            // found a duplicate
            current_start = *range.start();
            let a = ranges.last().unwrap();
            let b = range;

            if a.end() >= b.end() {
                // nothing to do, existing range is already same or bigger
            } else {
                ranges.pop();
                ranges.push(b.clone());
            }
        } else {
            current_start = *range.start();
            ranges.push(range.clone());
        }
    }

    ranges
}

pub fn part_2(input: &Input) -> crate::Output {
    let ranges = {
        let mut ranges = input.fresh_ranges.clone();
        ranges.sort_by_key(|range| *range.start());
        ranges
    };

    let mut ranges = dedup_ranges(&ranges);

    compact_ranges(&mut ranges);

    ranges
        .into_iter()
        .map(|range| range.end() - range.start() + 1)
        .sum::<usize>()
        .into()
}

#[cfg(test)]
pub const EXAMPLE_RESULTS: [Option<crate::Output>; 2] =
    [Some(crate::Output::Int(3)), Some(crate::Output::Int(14))];
