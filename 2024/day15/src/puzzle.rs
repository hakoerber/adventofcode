use crate::helpers::{Grid, Point, Vector};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cell {
    Wall,
    Robot,
    Box,
    Empty,
}

impl From<char> for Cell {
    fn from(value: char) -> Self {
        match value {
            '#' => Self::Wall,
            '@' => Self::Robot,
            'O' => Self::Box,
            '.' => Self::Empty,
            _ => panic!("invalid cell input"),
        }
    }
}

impl std::fmt::Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Wall => '#',
                Self::Robot => '@',
                Self::Box => 'O',
                Self::Empty => '.',
            }
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum WideCell {
    Wall,
    Robot,
    BoxStart,
    BoxEnd,
    Empty,
}

impl std::fmt::Display for WideCell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Wall => '#',
                Self::Robot => '@',
                Self::BoxStart => '[',
                Self::BoxEnd => ']',
                Self::Empty => '.',
            }
        )
    }
}

#[derive(Debug, Clone, Copy)]
enum Move {
    Up,
    Down,
    Left,
    Right,
}

impl From<char> for Move {
    fn from(value: char) -> Self {
        match value {
            '<' => Self::Left,
            'v' => Self::Down,
            '>' => Self::Right,
            '^' => Self::Up,
            _ => panic!("invalid move input"),
        }
    }
}

impl From<Move> for Vector {
    fn from(value: Move) -> Self {
        match value {
            Move::Up => Self { x: 0, y: -1 },
            Move::Down => Self { x: 0, y: 1 },
            Move::Left => Self { x: -1, y: 0 },
            Move::Right => Self { x: 1, y: 0 },
        }
    }
}

#[derive(Debug, Clone)]
pub struct Input {
    grid: Grid<Cell>,
    moves: Vec<Move>,
}

pub fn parse(input: &str) -> Input {
    let fields: Vec<&str> = input.split("\n\n").collect();
    assert_eq!(fields.len(), 2);
    Input {
        grid: Grid::from_str(fields[0]),
        moves: fields[1]
            .chars()
            .filter(|c| !c.is_whitespace())
            .map(|c| c.into())
            .collect(),
    }
}

pub fn part_1(input: &Input) -> crate::Output {
    let mut grid = input.grid.clone();
    let mut robot = grid.iter().find(|(_p, c)| *c == Cell::Robot).unwrap().0;

    for m in &input.moves {
        let v: Vector = (*m).into();
        let mut boxes_to_move = vec![];
        let mut movable = false;
        for i in 1.. {
            let v = v.mul(i);
            let pos = robot.add(&v).unwrap();
            match grid.get(&pos).unwrap() {
                Cell::Wall => {
                    movable = false;
                    break;
                }
                Cell::Robot => panic!("wtf"),
                Cell::Box => {
                    boxes_to_move.push(pos);
                }
                Cell::Empty => {
                    movable = true;
                    break;
                }
            }
        }
        if movable {
            if !boxes_to_move.is_empty() {
                *grid
                    .get_mut(
                        &robot
                            .add(&v.mul(isize::try_from(boxes_to_move.len()).unwrap() + 1))
                            .unwrap(),
                    )
                    .unwrap() = Cell::Box;
            }
            *grid.get_mut(&robot).unwrap() = Cell::Empty;
            robot = robot.add(&v).unwrap();
            *grid.get_mut(&robot).unwrap() = Cell::Robot;
        }
    }
    grid.iter()
        .filter_map(|(pos, cell)| {
            if cell == Cell::Box {
                Some(100 * pos.y + pos.x)
            } else {
                None
            }
        })
        .sum::<usize>()
        .into()
}

fn can_push_box_line(grid: &Grid<WideCell>, pos: &[&Point], dir: Move) -> bool {
    for pos in pos {
        let next = &pos.add(&dir.into()).unwrap();
        let can_push: bool = match (dir, grid.get(next).unwrap()) {
            (_, WideCell::Wall) => false,
            (_, WideCell::Empty) => true,
            (_, WideCell::Robot) => panic!("wtf"),
            (Move::Up | Move::Down, WideCell::BoxStart) => {
                let box_end = next.add(&Vector { x: 1, y: 0 }).unwrap();
                assert!(matches!(*grid.get(&box_end).unwrap(), WideCell::BoxEnd));

                can_push_box_line(grid, &[next, &box_end], dir)
            }
            (Move::Up | Move::Down, WideCell::BoxEnd) => {
                let box_start = next.add(&Vector { x: -1, y: 0 }).unwrap();
                assert!(matches!(*grid.get(&box_start).unwrap(), WideCell::BoxStart));

                can_push_box_line(grid, &[&box_start, next], dir)
            }
            (Move::Left, WideCell::BoxStart) | (Move::Right, WideCell::BoxEnd) => {
                panic!("invalid push check horizontal")
            }
            (Move::Left, WideCell::BoxEnd) => {
                let box_start = next.add(&Vector { x: -1, y: 0 }).unwrap();
                assert!(matches!(*grid.get(&box_start).unwrap(), WideCell::BoxStart));
                can_push_box_line(grid, &[&box_start], dir)
            }
            (Move::Right, WideCell::BoxStart) => {
                let box_end = next.add(&Vector { x: 1, y: 0 }).unwrap();
                assert!(matches!(*grid.get(&box_end).unwrap(), WideCell::BoxEnd));
                can_push_box_line(grid, &[&box_end], dir)
            }
        };
        if !can_push {
            return false;
        }
    }
    true
}

fn push_column(
    grid: &mut Grid<WideCell>,
    pos: &Point,
    dir: Move,
    moved: &mut Vec<Point>,
    start: WideCell,
) {
    let basev: Vector = dir.into();

    if moved.iter().any(|moved| moved == pos) {
        return;
    }

    let mut cells = Vec::new();
    for i in 0.. {
        let posi = pos.add(&basev.mul(i)).unwrap();
        let cell = *grid.get(&posi).unwrap();
        moved.push(posi.clone());
        if !matches!(cell, WideCell::BoxStart | WideCell::BoxEnd) {
            break;
        }
        match cell {
            WideCell::BoxStart => {
                let box_end = posi.add(&Vector { x: 1, y: 0 }).unwrap();
                push_column(grid, &box_end, dir, moved, WideCell::Empty);
            }
            WideCell::BoxEnd => {
                let box_start = posi.add(&Vector { x: -1, y: 0 }).unwrap();
                push_column(grid, &box_start, dir, moved, WideCell::Empty);
            }
            _ => panic!("wtf"),
        }
        cells.push(cell);
    }

    *grid.get_mut(pos).unwrap() = start;
    for (i, cell) in cells.iter().enumerate() {
        *grid
            .get_mut(
                &pos.add(&basev.mul(isize::try_from(i).unwrap().checked_add(1).unwrap()))
                    .unwrap(),
            )
            .unwrap() = *cell;
    }
}

fn push_box_vert(grid: &mut Grid<WideCell>, pos: &Point, dir: Move) {
    let start = grid.get(pos).unwrap();
    assert_eq!(*start, WideCell::Robot);

    match dir {
        Move::Left | Move::Right => panic!("invalid vert push direction"),
        Move::Up | Move::Down => match grid.get(pos).unwrap() {
            WideCell::Robot => {
                push_column(
                    grid,
                    &pos.add(&dir.into()).unwrap(),
                    dir,
                    &mut Vec::new(),
                    WideCell::Robot,
                );
            }
            _ => panic!("invalid vert box push cell found"),
        },
    }
}

fn push_box(grid: &mut Grid<WideCell>, pos: &Point, dir: Move, current: WideCell) {
    let next = &pos.add(&dir.into()).unwrap();
    match (dir, grid.get(next).unwrap()) {
        (_, WideCell::Wall) => panic!("cannot push"),
        (_, WideCell::Robot) => panic!("wtf"),
        (Move::Up | Move::Down, WideCell::BoxStart | WideCell::BoxEnd) => {
            push_box_vert(grid, pos, dir);
        }
        (_, WideCell::Empty) => {
            *grid.get_mut(next).unwrap() = current;
        }
        (Move::Left, WideCell::BoxStart) | (Move::Right, WideCell::BoxEnd) => {
            panic!("invalid push horizontal with box")
        }
        (Move::Left, WideCell::BoxEnd) => {
            let box_start = next.add(&Vector { x: -1, y: 0 }).unwrap();
            assert!(matches!(*grid.get(&box_start).unwrap(), WideCell::BoxStart));
            push_box(grid, &box_start, dir, WideCell::BoxStart);
            *grid.get_mut(&box_start).unwrap() = WideCell::BoxEnd;
            *grid.get_mut(next).unwrap() = current;
        }
        (Move::Right, WideCell::BoxStart) => {
            let box_end = next.add(&Vector { x: 1, y: 0 }).unwrap();
            assert!(matches!(*grid.get(&box_end).unwrap(), WideCell::BoxEnd));
            push_box(grid, &box_end, dir, WideCell::BoxEnd);
            *grid.get_mut(&box_end).unwrap() = WideCell::BoxStart;
            *grid.get_mut(next).unwrap() = current;
        }
    }
}

pub fn part_2(input: &Input) -> crate::Output {
    let mut grid = input.grid.flat_transform(|cell| match cell {
        Cell::Wall => [WideCell::Wall, WideCell::Wall],
        Cell::Robot => [WideCell::Robot, WideCell::Empty],
        Cell::Box => [WideCell::BoxStart, WideCell::BoxEnd],
        Cell::Empty => [WideCell::Empty, WideCell::Empty],
    });

    let mut robot = grid.iter().find(|(_p, c)| *c == WideCell::Robot).unwrap().0;

    for m in &input.moves {
        let movable = can_push_box_line(&grid, &[&robot], *m);
        if movable {
            push_box(&mut grid, &robot, *m, WideCell::Robot);
            *grid.get_mut(&robot).unwrap() = WideCell::Empty;
            robot = robot.add(&(*m).into()).unwrap();
            *grid.get_mut(&robot).unwrap() = WideCell::Robot;
        }
    }

    grid.iter()
        .filter_map(|(pos, cell)| {
            if cell == WideCell::BoxStart {
                Some(100 * pos.y + pos.x)
            } else {
                None
            }
        })
        .sum::<usize>()
        .into()
}

#[cfg(test)]
pub const EXAMPLE_RESULTS: [Option<crate::Output>; 2] = [
    Some(crate::Output::Int(10092)),
    Some(crate::Output::Int(9021)),
];
