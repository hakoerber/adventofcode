use nom::{
    bytes::complete::tag,
    character::complete::{char, digit1, multispace0, newline},
    combinator::map,
    multi::{many1, separated_list1},
    sequence::{delimited, preceded, separated_pair, tuple},
    IResult,
};

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

fn part1(input: &str) -> Result<usize, String> {
    let (rest, racesheet) = RaceSheet::parse(input).map_err(|e| e.to_string())?;
    if !rest.is_empty() {
        eprintln!("parsing rest found: {rest}");
        panic!();
    }
    let result = racesheet
        .races
        .into_iter()
        .map(|race| {
            (0..=race.time)
                .filter(|hold_time| {
                    let time_travelled = race.time - hold_time;
                    let speed = hold_time;
                    let distance_travelled = time_travelled * speed;

                    distance_travelled > race.distance
                })
                .collect::<Vec<_>>()
                .len()
        })
        .product();
    Ok(result)
}

fn part2(input: &str) -> Result<usize, String> {
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

    let result = (0..=time)
        .filter(|hold_time| {
            let time_travelled = time - hold_time;
            let speed = hold_time;
            let distance_travelled = time_travelled * speed;

            distance_travelled > distance
        })
        .collect::<Vec<_>>()
        .len();

    Ok(result)
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
        panic!("unknown part")
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

        assert_eq!(part1(&input).unwrap(), 288);
    }

    #[test]
    fn example_02() {
        let input = indoc! {"
            Time:      7  15   30
            Distance:  9  40  200
        "};

        assert_eq!(part2(&input).unwrap(), 71503);
    }
}
