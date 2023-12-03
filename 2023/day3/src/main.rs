use std::cmp::min;

#[derive(Debug)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug, Eq, PartialEq)]
struct Number {
    line: usize,
    range: (usize, usize),
}

impl Number {
    fn value(&self, lines: &[&str]) -> usize {
        let mut accum: usize = 0;
        for (i, x) in (self.range.0..=self.range.1).rev().enumerate() {
            let val = lines[self.line].chars().collect::<Vec<char>>()[x]
                .to_digit(10)
                .unwrap() as usize;
            accum += val * 10_usize.pow(i as u32);
        }
        accum
    }

    fn contains(&self, point: &Point) -> bool {
        point.y == self.line && point.x >= self.range.0 && point.x <= self.range.1
    }
}

fn find_numbers(lines: &[&str]) -> Vec<Number> {
    let mut numbers: Vec<Number> = vec![];

    for (y, line) in lines.iter().enumerate() {
        let mut in_number = false;
        let mut start = 0;
        for (x, c) in line.chars().enumerate() {
            if c.is_ascii_digit() {
                if !in_number {
                    start = x;
                    in_number = true;
                }
            } else if in_number {
                let end = x - 1;
                in_number = false;
                numbers.push(Number {
                    line: y,
                    range: (start, end),
                })
            }
        }
        if in_number {
            numbers.push(Number {
                line: y,
                range: (start, line.len() - 1),
            })
        }
    }

    numbers
}

fn part1(input: &str) -> usize {
    let lines: Vec<&str> = input.lines().collect();
    let numbers = find_numbers(&lines);

    let mut numbers_with_adjacent_symbol = vec![];

    for number in &numbers {
        let mut has_adjacent_symbol = false;

        let bounding_box: (Point, Point) = (
            Point {
                // this would underflow on the first column
                x: number.range.0.saturating_sub(1),
                // this would underflow on the first line
                y: number.line.saturating_sub(1),
            },
            Point {
                x: min(number.range.1 + 1, lines[number.line].len() - 1),
                // this would overflow on the last line
                y: min(number.line + 1, lines.len() - 1),
            },
        );

        'outer: for line in lines
            .iter()
            .skip(bounding_box.0.y)
            .take(bounding_box.1.y + 1)
        {
            for x in bounding_box.0.x..=bounding_box.1.x {
                let c = line.chars().collect::<Vec<char>>()[x];
                if !c.is_ascii_digit() && c != '.' {
                    has_adjacent_symbol = true;
                    break 'outer;
                }
            }
        }

        if has_adjacent_symbol {
            numbers_with_adjacent_symbol.push(number);
        }
    }

    let sum: usize = numbers_with_adjacent_symbol
        .into_iter()
        .map(|n| n.value(&lines))
        .sum();

    sum
}

fn part2(input: &str) -> usize {
    let mut gears: Vec<Point> = vec![];

    let lines: Vec<&str> = input.lines().collect();

    let numbers = find_numbers(&lines);

    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '*' {
                gears.push(Point { x, y });
            }
        }
    }

    let mut gear_ratios: Vec<Vec<&Number>> = vec![];
    for gear in &gears {
        let mut matching_numbers: Vec<&Number> = vec![];
        for number in &numbers {
            for x in gear.x.saturating_sub(1)..=min(gear.x + 1, lines[0].len()) {
                for y in gear.y.saturating_sub(1)..=min(gear.y + 1, lines.len()) {
                    let point = Point { x, y };
                    if number.contains(&point) && !matching_numbers.contains(&number) {
                        matching_numbers.push(number);
                    }
                }
            }
        }
        if matching_numbers.len() >= 2 {
            gear_ratios.push(matching_numbers);
        }
    }

    gear_ratios
        .into_iter()
        .map(|gear_numbers| {
            gear_numbers
                .into_iter()
                .map(|n| n.value(&lines))
                .product::<usize>()
        })
        .sum()
}

fn main() {
    let input = std::fs::read_to_string("./input").unwrap();

    println!("Part 1 : {}", part1(&input));
    println!("Part 2 : {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn example_01() {
        let input = indoc! {"
            467..114..
            ...*......
            ..35..633.
            ......#...
            617*......
            .....+.58.
            ..592.....
            ......755.
            ...$.*....
            .664.598..
        "};

        assert_eq!(part1(input), 4361);
    }

    #[test]
    fn example_02() {
        let input = indoc! {"
                467..114..
                ...*......
                ..35..633.
                ......#...
                617*......
                .....+.58.
                ..592.....
                ......755.
                ...$.*....
                .664.598..
            "};

        assert_eq!(part2(input), 467835);
    }
}
