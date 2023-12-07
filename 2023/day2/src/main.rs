use core::fmt;
use std::collections::HashMap;
use std::error::Error;

use nom::{
    bytes::complete::{tag, take_while1},
    character::complete::char,
    combinator::{cut, eof, map, map_res},
    error::{context, VerboseError},
    multi::{many0, separated_list1},
    sequence::{delimited, separated_pair, terminated, tuple},
    Err as NomErr, IResult,
};

#[derive(Debug)]
struct MyError(String);

impl Error for MyError {}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Color {
    Red,
    Green,
    Blue,
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Red => "Red",
                Self::Green => "Green",
                Self::Blue => "Blue",
            }
        )
    }
}

impl TryFrom<&str> for Color {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "red" => Ok(Self::Red),
            "green" => Ok(Self::Green),
            "blue" => Ok(Self::Blue),
            _ => Err(format!("invalid color \"{value}\"")),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct GameId(u32);

#[derive(Debug, PartialEq, Eq)]
struct Game {
    id: GameId,
    draws: Vec<HashMap<Color, u32>>,
}

impl Game {
    fn parse(line: &str) -> Result<Self, NomErr<VerboseError<&str>>> {
        fn number(i: &str) -> IResult<&str, u32, VerboseError<&str>> {
            context(
                "number parsing",
                map(
                    take_while1::<_, &str, VerboseError<_>>(|c: char| c.is_ascii_digit()),
                    |number| number.parse::<u32>().unwrap(),
                ),
            )(i)
        }

        fn parse_line(
            i: &str,
        ) -> IResult<&str, (GameId, Vec<HashMap<Color, u32>>), VerboseError<&str>> {
            let game_id = number;

            let prefix = context(
                "prefix",
                map(
                    delimited(
                        tuple((terminated(tag("Game"), char(' ')),)),
                        game_id,
                        tuple((char(':'), char(' '))),
                    ),
                    GameId,
                ),
            );

            let color_word = context(
                "color name",
                take_while1::<_, &str, VerboseError<_>>(|c: char| c.is_alphabetic()),
            );

            let color_name = context(
                "color name",
                cut(map_res(color_word, |s: &str| {
                    s.try_into()
                        .map_err(|s: String| nom::Err::Failure(format!("unknown color: {s}")))
                })),
            );

            let color_count = context(
                "color count",
                map(separated_pair(number, char(' '), color_name), |color| {
                    (color.1, color.0)
                }),
            );

            let draw = context(
                "draw",
                map_res(
                    separated_list1(tuple((char(','), many0(char(' ')))), color_count),
                    |colors| {
                        let mut hashmap = HashMap::new();
                        for (color_name, color_count) in colors {
                            if hashmap.insert(color_name, color_count).is_some() {
                                return Err(format!("duplicate color found: {}", color_name));
                            }
                        }
                        Ok(hashmap)
                    },
                ),
            );

            let draws = context(
                "draws",
                separated_list1(tuple((char(';'), many0(char(' ')))), draw),
            );

            context("main", terminated(tuple((prefix, draws)), eof))(i)
        }

        let (rest, (id, draws)) = parse_line(line)?;
        assert!(rest.is_empty());

        Ok(Self { draws, id })
    }

    fn minimum_cube_count(&self) -> HashMap<Color, u32> {
        let mut required_cubes: HashMap<Color, u32> = HashMap::new();
        for draw in &self.draws {
            for (color, draw_count) in draw {
                match required_cubes.get(color) {
                    Some(required_cube_count) if required_cube_count < draw_count => {
                        required_cubes.insert(*color, *draw_count);
                    }
                    None => {
                        required_cubes.insert(*color, *draw_count);
                    }
                    _ => (),
                }
            }
        }
        required_cubes
    }
}

fn parse_input(input: &str) -> Result<Vec<Game>, NomErr<VerboseError<&str>>> {
    input
        .lines()
        .map(|line| line.trim())
        .map(Game::parse)
        .collect::<Result<Vec<Game>, _>>()
}

fn count_possible_games(games: &[Game], limits: &HashMap<Color, u32>) -> Result<u32, String> {
    Ok(games
        .iter()
        .filter_map(|game| {
            game.draws
                .iter()
                .map(|draw| {
                    limits
                        .iter()
                        .map(|(limit_color, limit_count)| {
                            draw.get(limit_color).unwrap_or(&0) <= limit_count
                        })
                        .all(|possible| possible)
                })
                .all(|possible| possible)
                .then_some(game.id)
        })
        .map(|game_id| game_id.0)
        .sum())
}

fn minimum_cube_powered(games: &[Game]) -> Result<u32, String> {
    Ok(games
        .iter()
        .map(|game| game.minimum_cube_count())
        .map(|counts| counts.into_values().product::<u32>())
        .sum())
}

fn limits() -> HashMap<Color, u32> {
    HashMap::from([(Color::Red, 12), (Color::Green, 13), (Color::Blue, 14)])
}

fn main() -> Result<(), String> {
    let input = include_str!("../input");

    match parse_input(&input) {
        Ok(input) => {
            let count = count_possible_games(&input, &limits())?;

            println!("part 1 : {count}");

            let power = minimum_cube_powered(&input)?;

            println!("part 2 : {power}");
        }
        Err(e) => match e {
            NomErr::Incomplete(needed) => match needed {
                nom::Needed::Unknown => eprintln!("unknown data needed"),
                nom::Needed::Size(n) => eprintln!("needed {n} more bytes"),
            },
            NomErr::Error(e) | NomErr::Failure(e) => {
                eprintln!("{}", nom::error::convert_error(input, e))
            }
        },
    };
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn game_parsing() {
        assert_eq!(
            Game::parse("Game 1: 5 blue, 1 red, 4 green; 2 red; 9 green, 13 blue").unwrap(),
            Game {
                id: GameId(1),
                draws: vec![
                    HashMap::from([(Color::Blue, 5), (Color::Red, 1), (Color::Green, 4)]),
                    HashMap::from([(Color::Red, 2)]),
                    HashMap::from([(Color::Green, 9), (Color::Blue, 13)]),
                ]
            }
        );

        assert!(Game::parse("Game 1: 5 blue").is_ok());
        assert!(Game::parse("Game 1 5 blue").is_err());
        assert!(Game::parse("Game 1: 5 blue;").is_err());
        assert!(Game::parse("Gam 1: 5 blue").is_err());
        assert!(Game::parse("game 1: 5 blue").is_err());
        assert!(Game::parse("game a: 5 blue").is_err());
        assert!(Game::parse("Game 1: 5 blu").is_err());
        assert!(Game::parse("Game 1: x blue").is_err());
    }

    #[test]
    fn example_01() {
        let input = include_str!("../example_01");
        assert_eq!(
            count_possible_games(&parse_input(&input).unwrap(), &super::limits()).unwrap(),
            8
        );
    }

    #[test]
    fn example_02() {
        let input = include_str!("../example_02");
        assert_eq!(
            minimum_cube_powered(&parse_input(&input).unwrap()).unwrap(),
            2286
        );
    }
}
