use std::fmt;

use crate::helpers::{Grid, Point};

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Paper => "@",
                Self::Empty => " ",
            }
        )
    }
}

impl From<char> for Cell {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::Empty,
            '@' => Self::Paper,
            _ => panic!(),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Cell {
    Paper,
    Empty,
}

impl Cell {
    fn is_paper(self) -> bool {
        self == Self::Paper
    }
}

#[derive(Debug, Clone)]
pub struct Input {
    grid: Grid<Cell>,
}

pub fn parse(input: &str) -> Input {
    Input {
        grid: Grid::from_str(input),
    }
}

pub fn part_1(input: &Input) -> crate::Output {
    input
        .grid
        .filter_points_copy(Cell::is_paper)
        .fold(0, |mut acc, point| {
            if input
                .grid
                .adjacents_of_copy(&point)
                .filter(|cell| cell.value.is_paper())
                .count()
                < 4
            {
                acc += 1;
            }
            acc
        })
        .into()
}

pub fn part_2(input: &Input) -> crate::Output {
    let mut acc: usize = 0;

    let mut grid = input.grid.clone();

    loop {
        let to_remove: Vec<Point> =
            grid.filter_points_copy(Cell::is_paper)
                .fold(vec![], |mut to_remove, point| {
                    if grid
                        .adjacents_of_copy(&point)
                        .filter(|cell| cell.value.is_paper())
                        .count()
                        < 4
                    {
                        to_remove.push(point);
                    }
                    to_remove
                });

        if to_remove.is_empty() {
            break;
        }
        acc += to_remove.len();
        for p in to_remove {
            grid[&p] = Cell::Empty;
        }
    }

    acc.into()
}

#[cfg(test)]
pub const EXAMPLE_RESULTS: [Option<crate::Output>; 2] =
    [Some(crate::Output::Int(13)), Some(crate::Output::Int(43))];
