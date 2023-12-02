use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Color {
    Red,
    Green,
    Blue,
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

#[derive(Debug)]
struct Game {
    id: u32,
    draws: Vec<HashMap<Color, u32>>,
}

impl Game {
    fn parse(line: &str) -> Result<Self, String> {
        let mut list_of_draws: Vec<HashMap<Color, u32>> = vec![];

        let (gameinfo, draws) = line
            .split_once(':')
            .ok_or("line did not contain : delimiter")?;

        let id = {
            let (game, id) = gameinfo
                .split_once(' ')
                .ok_or("gameinfo did not contain a space")?;
            if game != "Game" {
                return Err(format!("did not contain \"Game\" prefix, found \"{game}\""));
            }
            id.parse::<u32>()
                .map_err(|e| format!("could not parse game id \"{id}\": {e}"))?
        };

        for draw in draws.split(';').map(|s| s.trim()) {
            let mut colors: HashMap<Color, u32> = HashMap::new();
            for color in draw.split(',').map(|s| s.trim()) {
                let (count, color) = color
                    .split_once(' ')
                    .ok_or("count and color were not separated by space")?;
                let color: Color = color.try_into()?;
                let count = count
                    .parse::<u32>()
                    .map_err(|e| format!("could not parse color count \"{count}\": {e}"))?;

                // insert returns Some(old_value) if the value as already present, this
                // is treated as an error
                if colors.insert(color, count).is_some() {
                    return Err("color seen more than once".into());
                }
            }
            list_of_draws.push(colors);
        }
        Ok(Self {
            draws: list_of_draws,
            id,
        })
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

fn parse_input(input: &str) -> Result<Vec<Game>, String> {
    input
        .lines()
        .map(|line| line.trim())
        .map(Game::parse)
        .collect::<Result<Vec<Game>, String>>()
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
    let input = std::fs::read_to_string("./input").unwrap();

    let input = parse_input(&input)?;
    let count = count_possible_games(&input, &limits())?;

    println!("part 1 : {count}");

    let power = minimum_cube_powered(&input)?;

    println!("part 2 : {power}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_01() {
        let input = std::fs::read_to_string("./example_01").unwrap();
        assert_eq!(
            count_possible_games(&parse_input(&input).unwrap(), &super::limits()).unwrap(),
            8
        );
    }

    #[test]
    fn example_02() {
        let input = std::fs::read_to_string("./example_02").unwrap();
        assert_eq!(
            minimum_cube_powered(&parse_input(&input).unwrap()).unwrap(),
            2286
        );
    }
}
