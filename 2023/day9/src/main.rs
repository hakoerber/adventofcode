use std::num::ParseIntError;

fn part1(input: &str) -> Result<usize, String> {
    let mut sum = 0;
    for line in input.lines() {
        let mut map: Vec<Vec<isize>> = vec![];

        map.push(
            line.split_whitespace()
                .map(str::parse::<isize>)
                .collect::<Result<Vec<isize>, ParseIntError>>()
                .unwrap(),
        );

        for i in 0.. {
            let reduced = map[i]
                .windows(2)
                .map(|w| w[1] - w[0])
                .collect::<Vec<isize>>();

            let all_zero = reduced.iter().all(|e| *e == 0);

            map.push(reduced);

            if all_zero {
                break;
            }
        }

        let len = map.len();

        map[len - 1].push(0);

        for i in (0..map.len() - 1).rev() {
            let j = map[i].len();
            let left = map[i][j - 1];
            let below = map[i + 1][j - 1];
            map[i].push(left + below);
        }

        sum += map[0].last().unwrap();
    }

    Ok(sum as usize)
}

fn part2(input: &str) -> Result<usize, String> {
    let mut sum = 0;
    for line in input.lines() {
        let mut map: Vec<Vec<isize>> = vec![];

        map.push(
            line.split_whitespace()
                .map(str::parse::<isize>)
                .collect::<Result<Vec<isize>, ParseIntError>>()
                .unwrap(),
        );

        for i in 0.. {
            let reduced = map[i]
                .windows(2)
                .map(|w| w[1] - w[0])
                .collect::<Vec<isize>>();

            let all_zero = reduced.iter().all(|e| *e == 0);

            map.push(reduced);

            if all_zero {
                break;
            }
        }

        let len = map.len();

        map[len - 1].insert(0, 0);

        for i in (0..map.len() - 1).rev() {
            let right = map[i][0];
            let below = map[i + 1][0];
            map[i].insert(0, right - below);
        }

        sum += map[0].first().unwrap();
    }

    Ok(sum as usize)
}

fn main() -> Result<(), String> {
    let input = include_str!("../input");

    let args = std::env::args().skip(1).collect::<Vec<String>>();
    let part = args[0].parse::<usize>().unwrap();

    if part == 1 {
        println!("Part 1 : {}", part1(input)?);
    } else if part == 2 {
        println!("Part 2 : {}", part2(input)?);
    } else {
        return Err("unknown part".into());
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn part_1_example_1() {
        let input = indoc! {"
            0 3 6 9 12 15
            1 3 6 10 15 21
            10 13 16 21 30 45
        "};
        assert_eq!(part1(input).unwrap(), 114);
    }

    #[test]
    fn part_2_example_1() {
        let input = indoc! {"
            0 3 6 9 12 15
            1 3 6 10 15 21
            10 13 16 21 30 45
        "};
        assert_eq!(part2(input).unwrap(), 2);
    }
}
