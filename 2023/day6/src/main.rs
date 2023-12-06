use nom::{
    bytes::complete::tag,
    character::complete::{char, digit1, multispace0, newline},
    combinator::map,
    multi::{many1, separated_list1},
    sequence::{delimited, preceded, separated_pair, tuple},
    IResult,
};
use std::str::FromStr;

#[derive(PartialEq, Eq, Clone, Copy)]
#[allow(dead_code)]
enum Approach {
    BruteForce,
    QuadraticFormula,
    RangeReduction,
}

impl FromStr for Approach {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "bruteforce" => Self::BruteForce,
            "quadraticformula" => Self::QuadraticFormula,
            "rangereduction" => Self::RangeReduction,
            _ => return Err("unknown approach"),
        })
    }
}

#[allow(dead_code)]
impl Approach {
    fn values() -> Vec<Self> {
        vec![
            Self::BruteForce,
            Self::QuadraticFormula,
            Self::RangeReduction,
        ]
    }
}

fn number(i: &str) -> IResult<&str, usize> {
    map(digit1, |f: &str| f.parse::<usize>().unwrap())(i)
}

fn spaces(i: &str) -> IResult<&str, Vec<char>> {
    many1(char(' '))(i)
}

#[derive(Debug)]
struct Race {
    time: usize,
    distance: usize,
}

impl Race {
    fn wins(&self, approach: Approach) -> usize {
        match approach {
            Approach::BruteForce => (0..=self.time)
                .filter(|hold_time| {
                    let time_travelled = self.time - hold_time;
                    let speed = hold_time;
                    let distance_travelled = time_travelled * speed;

                    distance_travelled > self.distance
                })
                .collect::<Vec<_>>()
                .len(),
            Approach::QuadraticFormula => {
                // the races form a quadratic function:
                //
                // T = the total time of the race
                //
                // t = charging time, in [0..=self.time]
                // y = resulting distance
                //
                // y = (T - t) x t
                // y = -t^2 + Tt
                //
                // By substracing the winning time, we know that all values of x *above*
                // y = 0 are winings
                //
                // D = winning distance
                //
                // y = -t^2 + Tt - D
                //
                // We need to calculate the roots of that function using the quadratic formula,
                // with
                //
                // a = -1
                // b = T
                // c = -D

                // use isizes to enable negation and pow/sqrt
                let a: f64 = -1.0;
                let b: f64 = self.time as f64;
                let c: f64 = -(self.distance as f64);

                let roots: Option<(f64, f64)> = {
                    let discriminant = (b.powi(2)) - 4.0 * a * c;
                    if !discriminant.is_sign_positive() {
                        None
                    } else {
                        Some((
                            (-b + f64::sqrt(discriminant)) / (2.0 * a),
                            (-b - f64::sqrt(discriminant)) / (2.0 * a),
                        ))
                    }
                };

                if let Some((x1, x2)) = roots {
                    // Sort the roots by size. The order is determined by the sign of a, so it's
                    // constant for our input, but this solution is more general
                    let (x1, x2) = if x1 > x2 { (x2, x1) } else { (x1, x2) };
                    assert!(x1 < x2);

                    // We actually found two roots. All whole integers that lie *between* the
                    // roots are races that we win. Exact matches are a tie and do not win.
                    (x2.ceil() as usize - 1) - (x1.floor() as usize + 1) + 1
                } else {
                    // No roots, there is no way for us to win the race.
                    0
                }
            }
            Approach::RangeReduction => {
                let limit: isize = self.distance.try_into().unwrap();
                let time: isize = self.time.try_into().unwrap();
                let f = |x: isize| -x * x + time * x;

                // we know that charging for 0 seconds always loses, as the result will be 0
                let lower_bound = 0;
                assert!(f(lower_bound) == 0);

                // we know that charging for the hold time of the game always loses, as the
                // result will be 0
                let upper_bound = self.time.try_into().unwrap();
                assert!(f(upper_bound) == 0);

                // linear search, this could be optimized to a binary search
                let first_win = (lower_bound..upper_bound).find(|x| f(*x) > limit);
                let last_win = (lower_bound..upper_bound).rev().find(|x| f(*x) > limit);

                // merge the options into one,
                let wins = first_win.zip(last_win);

                if let Some((first, last)) = wins {
                    (last - first + 1).try_into().unwrap()
                } else {
                    0
                }
            }
        }
    }
}

#[derive(Debug)]
struct RaceSheet {
    races: Vec<Race>,
}

impl RaceSheet {
    fn parse(s: &str) -> IResult<&str, Self> {
        let (rest, (times, distances)) = delimited(
            multispace0,
            separated_pair(
                preceded(
                    tuple((tag("Time:"), spaces)),
                    separated_list1(spaces, number),
                ),
                newline,
                preceded(
                    tuple((tag("Distance:"), spaces)),
                    separated_list1(spaces, number),
                ),
            ),
            multispace0,
        )(s)?;

        let races = times
            .into_iter()
            .zip(distances)
            .map(|(time, distance)| Race { time, distance })
            .collect::<Vec<Race>>();

        Ok((rest, Self { races }))
    }
}

fn part1(input: &str, approach: Approach) -> Result<usize, String> {
    let (rest, racesheet) = RaceSheet::parse(input).map_err(|e| e.to_string())?;
    if !rest.is_empty() {
        eprintln!("parsing rest found: {rest}");
        panic!();
    }
    let result = racesheet
        .races
        .into_iter()
        .map(|race| race.wins(approach))
        .product();
    Ok(result)
}

fn part2(input: &str, approach: Approach) -> Result<usize, String> {
    let (rest, racesheet) = RaceSheet::parse(input).map_err(|e| e.to_string())?;
    if !rest.is_empty() {
        eprintln!("parsing rest found: {rest}");
        panic!();
    }

    let time = racesheet
        .races
        .iter()
        .map(|race| race.time.to_string())
        .collect::<String>()
        .parse::<usize>()
        .unwrap();

    let distance = racesheet
        .races
        .iter()
        .map(|race| race.distance.to_string())
        .collect::<String>()
        .parse::<usize>()
        .unwrap();

    let race = Race { time, distance };

    Ok(race.wins(approach))
}

fn main() -> Result<(), String> {
    let input = include_str!("../input");

    let args = std::env::args().skip(1).collect::<Vec<String>>();
    let part = args[0].parse::<usize>().unwrap();
    let approach = args[1].parse()?;

    if part == 1 {
        println!("Part 1 : {}", part1(input, approach)?);
    } else if part == 2 {
        println!("Part 2 : {}", part2(input, approach)?);
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
    fn example_01() {
        let input = indoc! {"
            Time:      7  15   30
            Distance:  9  40  200
        "};

        for approach in Approach::values() {
            assert_eq!(part1(&input, approach).unwrap(), 288);
        }
    }

    #[test]
    fn example_02() {
        let input = indoc! {"
            Time:      7  15   30
            Distance:  9  40  200
        "};

        for approach in Approach::values() {
            assert_eq!(part2(&input, approach).unwrap(), 71503);
        }
    }
}
