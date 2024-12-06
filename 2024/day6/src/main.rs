mod helpers;
mod output;
use output::Output;

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Guard {
    direction: Direction,
    position: Position,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Tile {
    Free,
    Obstacle,
    Guard(Guard),
}

#[derive(Debug, Clone)]
struct Input {
    grid: Vec<Vec<Tile>>,
}

fn parse(input: &str) -> Input {
    Input {
        grid: input
            .lines()
            .enumerate()
            .map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(|(x, c)| match c {
                        '.' => Tile::Free,
                        '#' => Tile::Obstacle,
                        '^' => Tile::Guard(Guard {
                            direction: Direction::Up,
                            position: Position { x, y },
                        }),
                        _ => panic!("invalid input"),
                    })
                    .collect()
            })
            .collect(),
    }
}

fn part_1(input: &Input) -> Output {
    let Tile::Guard(guard) = input
        .grid
        .iter()
        .find_map(|line| line.iter().find(|tile| matches!(tile, Tile::Guard(_))))
        .unwrap()
    else {
        unreachable!()
    };

    let max_x = input.grid[0].len() - 1;
    let max_y = input.grid.len() - 1;

    let mut state = guard.clone();

    let mut steps: Vec<Position> = vec![guard.position.clone()];

    loop {
        match state.direction {
            Direction::Up => {
                if state.position.y == 0 {
                    break;
                } else {
                    if input.grid[state.position.y - 1][state.position.x] == Tile::Obstacle {
                        state.direction = match state.direction {
                            Direction::Up => Direction::Right,
                            Direction::Down => Direction::Left,
                            Direction::Right => Direction::Down,
                            Direction::Left => Direction::Up,
                        };
                        continue;
                    }
                    state.position.y -= 1;
                    if steps.iter().all(|pos| *pos != state.position) {
                        steps.push(state.position.clone());
                    }
                }
            }
            Direction::Down => {
                if state.position.y == max_y {
                    break;
                } else {
                    if input.grid[state.position.y + 1][state.position.x] == Tile::Obstacle {
                        state.direction = match state.direction {
                            Direction::Up => Direction::Right,
                            Direction::Down => Direction::Left,
                            Direction::Right => Direction::Down,
                            Direction::Left => Direction::Up,
                        };
                        continue;
                    }
                    state.position.y += 1;
                    if steps.iter().all(|pos| *pos != state.position) {
                        steps.push(state.position.clone());
                    }
                }
            }
            Direction::Right => {
                if state.position.x == max_x {
                    break;
                } else {
                    if input.grid[state.position.y][state.position.x + 1] == Tile::Obstacle {
                        state.direction = match state.direction {
                            Direction::Up => Direction::Right,
                            Direction::Down => Direction::Left,
                            Direction::Right => Direction::Down,
                            Direction::Left => Direction::Up,
                        };
                        continue;
                    }
                    state.position.x += 1;
                    if steps.iter().all(|pos| *pos != state.position) {
                        steps.push(state.position.clone());
                    }
                }
            }
            Direction::Left => {
                if state.position.x == 0 {
                    break;
                } else {
                    if input.grid[state.position.y][state.position.x - 1] == Tile::Obstacle {
                        state.direction = match state.direction {
                            Direction::Up => Direction::Right,
                            Direction::Down => Direction::Left,
                            Direction::Right => Direction::Down,
                            Direction::Left => Direction::Up,
                        };
                        continue;
                    }
                    state.position.x -= 1;
                    if steps.iter().all(|pos| *pos != state.position) {
                        steps.push(state.position.clone());
                    }
                }
            }
        }
    }

    steps.len().into()
}

fn has_loop(grid: &[Vec<Tile>]) -> bool {
    let guard = grid
        .iter()
        .find_map(|line| {
            line.iter().find_map(|tile| match tile {
                Tile::Guard(g) => Some(g),
                _ => None,
            })
        })
        .unwrap();

    let max_x = grid[0].len() - 1;
    let max_y = grid.len() - 1;

    let mut state = guard.clone();

    let mut steps: Vec<(Position, Direction)> = vec![(guard.position.clone(), guard.direction)];

    loop {
        match state.direction {
            Direction::Up => {
                if state.position.y == 0 {
                    break false;
                } else {
                    if grid[state.position.y - 1][state.position.x] == Tile::Obstacle {
                        state.direction = match state.direction {
                            Direction::Up => Direction::Right,
                            Direction::Down => Direction::Left,
                            Direction::Right => Direction::Down,
                            Direction::Left => Direction::Up,
                        };
                        continue;
                    }
                    state.position.y -= 1;
                    if steps
                        .iter()
                        .any(|(pos, dir)| *pos == state.position && *dir == state.direction)
                    {
                        break true;
                    }
                    steps.push((state.position.clone(), state.direction));
                }
            }
            Direction::Down => {
                if state.position.y == max_y {
                    break false;
                } else {
                    if grid[state.position.y + 1][state.position.x] == Tile::Obstacle {
                        state.direction = match state.direction {
                            Direction::Up => Direction::Right,
                            Direction::Down => Direction::Left,
                            Direction::Right => Direction::Down,
                            Direction::Left => Direction::Up,
                        };
                        continue;
                    }
                    state.position.y += 1;
                    if steps
                        .iter()
                        .any(|(pos, dir)| *pos == state.position && *dir == state.direction)
                    {
                        break true;
                    }
                    steps.push((state.position.clone(), state.direction));
                }
            }
            Direction::Right => {
                if state.position.x == max_x {
                    break false;
                } else {
                    if grid[state.position.y][state.position.x + 1] == Tile::Obstacle {
                        state.direction = match state.direction {
                            Direction::Up => Direction::Right,
                            Direction::Down => Direction::Left,
                            Direction::Right => Direction::Down,
                            Direction::Left => Direction::Up,
                        };
                        continue;
                    }
                    state.position.x += 1;
                    if steps
                        .iter()
                        .any(|(pos, dir)| *pos == state.position && *dir == state.direction)
                    {
                        break true;
                    }
                    steps.push((state.position.clone(), state.direction));
                }
            }
            Direction::Left => {
                if state.position.x == 0 {
                    break false;
                } else {
                    if grid[state.position.y][state.position.x - 1] == Tile::Obstacle {
                        state.direction = match state.direction {
                            Direction::Up => Direction::Right,
                            Direction::Down => Direction::Left,
                            Direction::Right => Direction::Down,
                            Direction::Left => Direction::Up,
                        };
                        continue;
                    }
                    state.position.x -= 1;
                    if steps
                        .iter()
                        .any(|(pos, dir)| *pos == state.position && *dir == state.direction)
                    {
                        break true;
                    }
                    steps.push((state.position.clone(), state.direction));
                }
            }
        }
    }
}

fn part_2(input: &Input) -> Output {
    let mut possible_positions = 0;
    for y in 0..input.grid.len() {
        println!("{y} {possible_positions}");
        for x in 0..input.grid[0].len() {
            if input.grid[y][x] == Tile::Free {
                let mut mutation = input.grid.clone();
                mutation[y][x] = Tile::Obstacle;
                if has_loop(&mutation) {
                    possible_positions += 1;
                }
            }
        }
    }
    possible_positions.into()
}

fn main() {
    let input = parse(&std::fs::read_to_string("input").expect("input could not be read"));

    match std::env::args().nth(1) {
        Some(s) if s == "1" => println!("part 1: {}", part_1(&input)),
        Some(s) if s == "2" => println!("part 2: {}", part_2(&input)),
        _ => panic!("specify part"),
    }
}

#[cfg(test)]
mod test {
    const EXAMPLE_RESULT_PART_1: Output = Output::Int(41);
    const EXAMPLE_RESULT_PART_2: Output = Output::Int(6);

    use super::*;

    fn example_input() -> Input {
        parse(&std::fs::read_to_string("example").unwrap())
    }

    #[test]
    fn example_part_1() {
        assert_eq!(part_1(&example_input()), EXAMPLE_RESULT_PART_1)
    }

    #[test]
    fn example_part_2() {
        let input = example_input();
        assert_eq!(has_loop(&input.grid), false);
        assert_eq!(part_2(&input), EXAMPLE_RESULT_PART_2);
    }
}
