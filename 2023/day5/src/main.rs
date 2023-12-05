use std::cmp::{max, min};
use std::{ops::Range, str::FromStr};

use nom::{
    bytes::complete::{tag, take_until},
    character::complete::{char, digit1, multispace0, multispace1},
    combinator::map,
    multi::{many1, separated_list1},
    sequence::{preceded, separated_pair, terminated, tuple},
    IResult,
};
use rayon::prelude::*;

#[derive(PartialEq, Eq)]
#[allow(dead_code)]
enum Approach {
    Reverse,
    BruteForce,
    Ranges,
}

impl FromStr for Approach {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "reverse" => Self::Reverse,
            "bruteforce" => Self::BruteForce,
            "ranges" => Self::Ranges,
            _ => return Err("unknown approach"),
        })
    }
}

#[allow(dead_code)]
impl Approach {
    fn values() -> Vec<Self> {
        vec![Self::Reverse, Self::BruteForce, Self::Ranges]
    }
}

fn number(i: &str) -> IResult<&str, usize> {
    map(digit1, |f: &str| f.parse::<usize>().unwrap())(i)
}

fn spaces(i: &str) -> IResult<&str, Vec<char>> {
    many1(char(' '))(i)
}

fn ident(i: &str) -> IResult<&str, &str> {
    take_until(" ")(i)
}

#[derive(Debug)]
struct Almanac<S: Seeds> {
    seeds: Box<S>,
    map_lists: Vec<MapList>,
}

impl<S: Seeds<Out = S>> Almanac<S> {
    fn parse(s: &str) -> IResult<&str, Self> {
        let map_lists = separated_list1(multispace1, MapList::parse);
        let (rest, (seeds, map_lists)) = terminated(
            separated_pair(<S as Seeds>::parse, multispace1, map_lists),
            multispace0,
        )(s)?;
        Ok((rest, Self { seeds, map_lists }))
    }
}

#[derive(Debug)]
struct SeedList(Vec<usize>);

#[derive(Debug)]
struct SeedRanges(Vec<Range<usize>>);

trait Seeds {
    type Out: Seeds;
    fn parse(s: &str) -> IResult<&str, Box<Self::Out>>;
}

impl Seeds for SeedList {
    type Out = Self;
    fn parse(s: &str) -> IResult<&str, Box<Self::Out>> {
        let (rest, ids) = preceded(tag("seeds: "), separated_list1(char(' '), number))(s)?;
        Ok((rest, Box::new(Self(ids))))
    }
}

impl Seeds for SeedRanges {
    type Out = Self;
    fn parse(s: &str) -> IResult<&str, Box<Self::Out>> {
        let (rest, range) = preceded(
            tag("seeds: "),
            separated_list1(char(' '), separated_pair(number, spaces, number)),
        )(s)?;
        Ok((
            rest,
            Box::new(Self(
                range
                    .into_iter()
                    .map(|(start, length)| start..start + length)
                    .collect(),
            )),
        ))
    }
}

#[derive(Debug)]
struct Map {
    source_range: Range<usize>,
    destination_range: Range<usize>,
}

impl Map {
    fn parse(s: &str) -> IResult<&str, Self> {
        let (rest, (destination_start, source_start, length)) = tuple((
            terminated(number, spaces),
            terminated(number, spaces),
            number,
        ))(s)?;
        Ok((
            rest,
            Self {
                source_range: source_start..(source_start + length),
                destination_range: destination_start..(destination_start + length),
            },
        ))
    }

    fn map(&self, value: usize) -> Option<usize> {
        if self.source_range.contains(&value) {
            let index = value - self.source_range.start;
            // `nth()` is implemented via a simple addition, there is no actual calling
            // of `next()`
            Some(self.destination_range.clone().nth(index).unwrap())
        } else {
            None
        }
    }

    fn map_reverse(&self, value: usize) -> Option<usize> {
        if self.destination_range.contains(&value) {
            let index = value - self.destination_range.start;
            Some(self.source_range.clone().nth(index).unwrap())
        } else {
            None
        }
    }
}

#[derive(Debug)]
struct MapList {
    #[allow(dead_code)]
    header: String,
    maps: Vec<Map>,
}

impl MapList {
    fn parse(s: &str) -> IResult<&str, Self> {
        let maps = separated_list1(multispace1, Map::parse);
        let header = terminated(ident, tag(" map:"));

        let (rest, (header, maps)) = separated_pair(header, multispace1, maps)(s)?;

        Ok((
            rest,
            Self {
                header: header.to_owned(),
                maps,
            },
        ))
    }

    fn map(&self, value: usize) -> usize {
        for map in &self.maps {
            if let Some(mapped_value) = map.map(value) {
                return mapped_value;
            }
        }
        value
    }

    fn map_reverse(&self, value: usize) -> usize {
        for map in &self.maps {
            if let Some(mapped_value) = map.map_reverse(value) {
                return mapped_value;
            }
        }
        value
    }
}

fn part1(input: &str) -> Result<usize, String> {
    let (rest, almanac) = Almanac::<SeedList>::parse(input).map_err(|e| e.to_string())?;
    if !rest.is_empty() {
        println!("parsing rest found: {rest}");
        panic!();
    }
    let mut lowest_location = usize::MAX;
    for seed in &almanac.seeds.0 {
        let mut mapped_value = *seed;
        for map_list in &almanac.map_lists {
            mapped_value = map_list.map(mapped_value);
        }
        if mapped_value < lowest_location {
            lowest_location = mapped_value;
        }
    }
    Ok(lowest_location)
}

fn part2(input: &str, approach: Approach) -> Result<usize, String> {
    let (rest, almanac) = Almanac::<SeedRanges>::parse(input).map_err(|e| e.to_string())?;
    if !rest.is_empty() {
        println!("parsing rest found: {rest}");
        panic!();
    }

    let lowest_location = match approach {
        Approach::Ranges => {
            let current_ranges = almanac.seeds.0;

            // As each range can be partially mapped, the mapped ranges need to be marked properly. They
            // are skipped on further mappings, as mappings are only done *once* per block (they are not
            // iterative). Only unmapped ranges are mapped.
            #[derive(PartialEq, Eq)]
            enum State {
                Unmapped,
                Mapped,
            }

            #[derive(PartialEq, Eq)]
            struct RangeToMap {
                range: Range<usize>,
                state: State,
            }

            let mut current_ranges: Vec<RangeToMap> = current_ranges
                .into_iter()
                .map(|range| RangeToMap {
                    range,
                    state: State::Unmapped,
                })
                .collect();

            // There is one potential optimization that could be done after each mapping: Merge overlapping or adjacent ranges.
            // This would cut down on the number of ranges. For the puzzle input, it does not make sense, as the number of ranges
            // stays manageable (exactly 100 for my input). As number may grow by as much as three times for each mapping line,
            // this optimization would become prudent for longer input with many half-matching ranges.
            for map_list in almanac.map_lists.into_iter() {
                // Mark all ranges as unmapped again
                for range in &mut current_ranges {
                    range.state = State::Unmapped;
                }
                for map in map_list.maps {
                    for i in 0..current_ranges.len() {
                        if current_ranges[i].state == State::Mapped {
                            continue;
                        }

                        if current_ranges[i].range.end <= map.source_range.start {
                            // does not overlap, nothing to do
                            continue;
                        }
                        if current_ranges[i].range.start >= map.source_range.end {
                            // does not overlap, nothing to do
                            continue;
                        }

                        let overlap_start =
                            max(current_ranges[i].range.start, map.source_range.start);

                        let overlap_end = min(current_ranges[i].range.end, map.source_range.end);

                        let overlapping_range = (overlap_start + map.destination_range.start
                            - map.source_range.start)
                            ..(overlap_end + map.destination_range.start - map.source_range.start);

                        assert!(!overlapping_range.is_empty());

                        let range_before = current_ranges[i].range.start..overlap_start;
                        let range_after = overlap_end..current_ranges[i].range.end;

                        if !range_before.is_empty() {
                            current_ranges.push(RangeToMap {
                                range: range_before,
                                state: State::Unmapped,
                            });
                        }
                        if !range_after.is_empty() {
                            current_ranges.push(RangeToMap {
                                range: range_after,
                                state: State::Unmapped,
                            });
                        }
                        current_ranges[i] = RangeToMap {
                            range: overlapping_range,
                            state: State::Mapped,
                        };
                    }
                }
            }
            current_ranges
                .into_iter()
                .map(|range| range.range.start)
                .min()
                .unwrap()
        }
        Approach::Reverse => {
            let mut i = 0;
            loop {
                let mut reverse_val = i;
                for map_list in almanac.map_lists.iter().rev() {
                    reverse_val = map_list.map_reverse(reverse_val);
                }
                if almanac
                    .seeds
                    .0
                    .iter()
                    .any(|seed| seed.contains(&reverse_val))
                {
                    break i;
                }
                i += 1;
            }
        }
        Approach::BruteForce => almanac
            .seeds
            .0
            .into_par_iter()
            .map(|seed_range| {
                println!("{seed_range:?}");
                // let seeds = seed_range.collect::<Vec<usize>>();
                let result = seed_range
                    .clone()
                    .map(|seed| {
                        let mut mapped_value = seed;
                        for map_list in &almanac.map_lists {
                            mapped_value = map_list.map(mapped_value);
                        }
                        mapped_value
                    })
                    .min()
                    .unwrap();
                println!("{seed_range:?} => {result}");
                result
            })
            .min()
            .unwrap(),
    };

    Ok(lowest_location)
}

fn main() -> Result<(), String> {
    let input = include_str!("../input");

    let args = std::env::args().skip(1).collect::<Vec<String>>();
    let part = args[0].parse::<usize>().unwrap();

    if part == 1 {
        println!("Part 1 : {}", part1(input)?);
    } else if part == 2 {
        let approach = args[1].parse()?;

        println!("Part 2 : {}", part2(input, approach)?);
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
            seeds: 79 14 55 13

            seed-to-soil map:
            50 98 2
            52 50 48

            soil-to-fertilizer map:
            0 15 37
            37 52 2
            39 0 15

            fertilizer-to-water map:
            49 53 8
            0 11 42
            42 0 7
            57 7 4

            water-to-light map:
            88 18 7
            18 25 70

            light-to-temperature map:
            45 77 23
            81 45 19
            68 64 13

            temperature-to-humidity map:
            0 69 1
            1 0 69

            humidity-to-location map:
            60 56 37
            56 93 4
        "};

        assert_eq!(part1(&input).unwrap(), 35);
    }

    #[test]
    fn test_map() {
        let input = indoc! {"
            50 98 2
        "};

        let (_rest, map) = Map::parse(input).unwrap();
        assert_eq!(map.map(97), None);
        assert_eq!(map.map(98), Some(50));
        assert_eq!(map.map(99), Some(51));
        assert_eq!(map.map(100), None);
        assert_eq!(map.map(101), None);
    }

    #[test]
    fn test_map_list() {
        let input = indoc! {"
            seed-to-soil map:
            50 98 2
            52 50 48
        "};

        let (_rest, maplist) = MapList::parse(input).unwrap();
        assert_eq!(maplist.map(79), 81);
        assert_eq!(maplist.map(14), 14);
        assert_eq!(maplist.map(55), 57);
        assert_eq!(maplist.map(13), 13);
    }

    #[test]
    fn example_02() {
        let input = indoc! {"
            seeds: 79 14 55 13

            seed-to-soil map:
            50 98 2
            52 50 48

            soil-to-fertilizer map:
            0 15 37
            37 52 2
            39 0 15

            fertilizer-to-water map:
            49 53 8
            0 11 42
            42 0 7
            57 7 4

            water-to-light map:
            88 18 7
            18 25 70

            light-to-temperature map:
            45 77 23
            81 45 19
            68 64 13

            temperature-to-humidity map:
            0 69 1
            1 0 69

            humidity-to-location map:
            60 56 37
            56 93 4
        "};

        for approach in Approach::values() {
            assert_eq!(part2(&input, approach).unwrap(), 46);
        }
    }
}
