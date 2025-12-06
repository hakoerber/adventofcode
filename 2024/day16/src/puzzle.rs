use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};

use crate::helpers::{Grid, Point, Vector};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cell {
    Empty,
    Wall,
    Start,
    End,
}

impl From<char> for Cell {
    fn from(value: char) -> Self {
        match value {
            '#' => Self::Wall,
            '.' => Self::Empty,
            'S' => Self::Start,
            'E' => Self::End,
            _ => panic!("invalid input"),
        }
    }
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Empty => '.',
                Self::Wall => '#',
                Self::Start => 'S',
                Self::End => 'E',
            }
        )
    }
}

#[derive(Debug, Clone, Copy)]
enum Command {
    TurnLeft,
    TurnRight,
    Forward,
}

#[derive(Debug, Clone)]
struct Path(Vec<(Command, Point)>);

impl Path {
    fn cost(&self) -> usize {
        self.0
            .iter()
            .map(|(step, _point)| match step {
                Command::TurnLeft | Command::TurnRight => 1000,
                Command::Forward => 1,
            })
            .sum()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn left(self) -> Self {
        match self {
            Self::North => Self::West,
            Self::East => Self::North,
            Self::South => Self::East,
            Self::West => Self::South,
        }
    }
    fn right(self) -> Self {
        match self {
            Self::North => Self::East,
            Self::East => Self::South,
            Self::South => Self::West,
            Self::West => Self::North,
        }
    }
}

impl From<Direction> for Vector {
    fn from(value: Direction) -> Self {
        match value {
            Direction::North => Self { x: 0, y: -1 },
            Direction::East => Self { x: 1, y: 0 },
            Direction::South => Self { x: 0, y: 1 },
            Direction::West => Self { x: -1, y: 0 },
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Cost(usize);

impl Cost {
    fn add(self, value: usize) -> Self {
        Self(self.0 + value)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct State {
    position: Point,
    direction: Direction,
    cost: Cost,
}

impl State {
    fn turn_left(self) -> Self {
        Self {
            position: self.position.clone(),
            direction: self.direction.left(),
            cost: self.cost.add(1000),
        }
    }

    fn turn_right(self) -> Self {
        Self {
            position: self.position.clone(),
            direction: self.direction.right(),
            cost: self.cost.add(1000),
        }
    }

    fn forward(self) -> Self {
        Self {
            position: self.position.add(&self.direction.into()).unwrap(),
            direction: self.direction,
            cost: self.cost.add(1),
        }
    }
}

type Maze = Grid<Cell>;

#[expect(
    clippy::match_same_arms,
    reason = "so we can properly document via comments"
)]
fn find_cheapest_path(
    maze: &Maze,
    state: &State,
    known_points: &mut HashMap<(Point, Direction), Cost>,
) -> Option<Vec<Path>> {
    let mut paths: Vec<Path> = Vec::new();

    // try forward
    let new_position = state.position.add(&state.direction.into()).unwrap();
    match maze.get(&new_position).unwrap() {
        Cell::Empty => {
            let new_state = state.clone().forward();
            if known_points
                .get(&(new_state.position.clone(), new_state.direction))
                .is_none_or(|cost| *cost >= new_state.cost)
            {
                known_points.insert(
                    (new_state.position.clone(), new_state.direction),
                    new_state.cost,
                );
                if let Some(cheapest_paths) = find_cheapest_path(maze, &new_state, known_points) {
                    for cheapest_path in cheapest_paths {
                        let path = Path(
                            [
                                &[(Command::Forward, new_position.clone())],
                                cheapest_path.0.as_slice(),
                            ]
                            .concat(),
                        );
                        paths.push(path);
                    }
                }
            }
        }
        Cell::Wall => {
            // Stepping forward not possible
        }
        Cell::Start => {
            // Getting back to the start is always more expensive than just turning at the
            // very beginning
        }
        Cell::End => {
            return Some(vec![Path(vec![(Command::Forward, new_position.clone())])]);
        }
    }

    // left + forward
    let new_position = state.position.add(&state.direction.left().into()).unwrap();
    match maze.get(&new_position).unwrap() {
        Cell::Empty => {
            let new_state = state.clone().turn_left().forward();
            if known_points
                .get(&(new_state.position.clone(), new_state.direction))
                .is_none_or(|cost| *cost >= new_state.cost)
            {
                known_points.insert(
                    (new_state.position.clone(), new_state.direction),
                    new_state.cost,
                );
                if let Some(cheapest_paths) = find_cheapest_path(maze, &new_state, known_points) {
                    for cheapest_path in cheapest_paths {
                        let path = Path(
                            [
                                &[
                                    (Command::TurnLeft, new_position.clone()),
                                    (Command::Forward, new_position.clone()),
                                ],
                                cheapest_path.0.as_slice(),
                            ]
                            .concat(),
                        );
                        paths.push(path);
                    }
                }
            }
        }
        Cell::Wall => {
            // Stepping forward not possible
        }
        Cell::Start => {
            // Getting back to the start is always more expensive than just turning at the
            // very beginning
        }
        Cell::End => {
            return Some(vec![Path(vec![
                (Command::TurnLeft, new_position.clone()),
                (Command::Forward, new_position.clone()),
            ])]);
        }
    }

    // right + forward
    let new_position = state.position.add(&state.direction.right().into()).unwrap();
    match maze.get(&new_position).unwrap() {
        Cell::Empty => {
            let new_state = state.clone().turn_right().forward();
            if known_points
                .get(&(new_state.position.clone(), new_state.direction))
                .is_none_or(|cost| *cost >= new_state.cost)
            {
                known_points.insert(
                    (new_state.position.clone(), new_state.direction),
                    new_state.cost,
                );
                if let Some(cheapest_paths) = find_cheapest_path(maze, &new_state, known_points) {
                    for cheapest_path in cheapest_paths {
                        let path = Path(
                            [
                                &[
                                    (Command::TurnRight, new_position.clone()),
                                    (Command::Forward, new_position.clone()),
                                ],
                                cheapest_path.0.as_slice(),
                            ]
                            .concat(),
                        );
                        paths.push(path);
                    }
                }
            }
        }
        Cell::Wall => {
            // Stepping forward not possible
        }
        Cell::Start => {
            // Getting back to the start is always more expensive than just turning at the
            // very beginning
        }
        Cell::End => {
            return Some(vec![Path(vec![
                (Command::TurnRight, new_position.clone()),
                (Command::Forward, new_position.clone()),
            ])]);
        }
    }

    if paths.is_empty() {
        None
    } else {
        let min_cost = paths.iter().map(|path| path.cost()).min().unwrap();
        Some(
            paths
                .into_iter()
                .filter(|path| path.cost() == min_cost)
                .collect(),
        )
    }
}

#[derive(Debug, Clone)]
pub struct Input {
    maze: Maze,
}

pub fn parse(input: &str) -> Input {
    Input {
        maze: Grid::from_str(input),
    }
}

pub fn part_1(input: &Input) -> crate::Output {
    let start = input
        .maze
        .iter()
        .find_map(|(p, c)| (c == Cell::Start).then_some(p))
        .unwrap();

    let paths = find_cheapest_path(
        &input.maze,
        &State {
            position: start,
            direction: Direction::East,
            cost: Cost(0),
        },
        &mut HashMap::new(),
    )
    .expect("no path found");

    paths[0].cost().into()
}

pub fn part_2(input: &Input) -> crate::Output {
    let start = input
        .maze
        .iter()
        .find_map(|(p, c)| (c == Cell::Start).then_some(p))
        .unwrap();

    let paths = find_cheapest_path(
        &input.maze,
        &State {
            position: start.clone(),
            direction: Direction::East,
            cost: Cost(0),
        },
        &mut HashMap::new(),
    )
    .expect("no path found");

    let mut points: HashSet<Point> = HashSet::new();
    points.insert(start);

    for path in paths {
        for (_command, point) in path.0 {
            points.insert(point);
        }
    }

    points.len().into()
}

#[cfg(test)]
pub const EXAMPLE_RESULTS: [Option<&[crate::Output]>; 2] = [
    Some(&[crate::Output::Int(7036), crate::Output::Int(11048)]),
    Some(&[crate::Output::Int(45), crate::Output::Int(64)]),
];
