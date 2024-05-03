#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Pipe {
    Vertical,
    Horizontal,
    NE,
    NW,
    SE,
    SW,
}

impl TryFrom<char> for Pipe {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            '|' => Self::Vertical,
            '-' => Self::Horizontal,
            'L' => Self::NE,
            'J' => Self::NW,
            '7' => Self::SW,
            'F' => Self::SE,
            _ => return Err("unknown tile".to_string()),
        })
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Tile {
    Pipe(Pipe),
    Startpoint,
    Ground,
}

impl TryFrom<char> for Tile {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            '.' => Self::Ground,
            'S' => Self::Startpoint,
            _ => Self::Pipe(Pipe::try_from(value)?),
        })
    }
}

#[derive(Debug)]
struct Grid(Vec<Vec<Tile>>);

impl Grid {
    fn width(&self) -> usize {
        self.0[0].len()
    }

    fn height(&self) -> usize {
        self.0.len()
    }

    fn get_path(&self) -> Result<Vec<Point>, String> {
        let mut start_point = None;
        for position in self.iter() {
            if self[position] == Tile::Startpoint {
                start_point = Some(position);
            }
        }

        let Some(start_point) = start_point else {
            panic!("no start point found")
        };

        let mut path: Vec<Point> = vec![start_point];

        let connected_pipes = self.connected_pipes(start_point);
        assert!(connected_pipes.len() == 2);

        let mut current_point = connected_pipes[0];
        let mut came_from = start_point;

        loop {
            path.push(current_point);
            let connected_pipes = self.connected_pipes(current_point);
            assert!(connected_pipes.len() == 2);

            let next_point = connected_pipes
                .into_iter()
                .filter(|pos| *pos != came_from)
                .collect::<Vec<Point>>()[0];
            came_from = current_point;
            current_point = next_point;

            if self[current_point] == Tile::Startpoint {
                break Ok(path);
            }
        }
    }

    fn parse(input: &str) -> Result<Self, String> {
        Ok(Self(
            input
                .lines()
                .map(|s| {
                    s.chars()
                        .map(Tile::try_from)
                        .collect::<Result<Vec<Tile>, String>>()
                })
                .collect::<Result<Vec<Vec<Tile>>, String>>()?,
        ))
    }

    fn iter(&self) -> impl Iterator<Item = Point> {
        let height = self.height();
        let width = self.width();

        (0..height)
            .map(|y| {
                (0..width)
                    .map(move |x| Point { y, x })
                    .collect::<Vec<Point>>()
            })
            .collect::<Vec<Vec<Point>>>()
            .into_iter()
            .flatten()
    }

    fn connected_pipes(&self, point: Point) -> Vec<Point> {
        let mut adjacent_points = vec![];

        if point.x > 0
            && matches!(
                self[point],
                Tile::Startpoint | Tile::Pipe(Pipe::Horizontal | Pipe::NW | Pipe::SW)
            )
        {
            let point = Point {
                x: point.x - 1,
                y: point.y,
            };
            let tile = self[point];
            if tile == Tile::Startpoint {
                adjacent_points.push(point);
            } else if let Tile::Pipe(pipe) = tile {
                if pipe == Pipe::Horizontal || pipe == Pipe::NE || pipe == Pipe::SE {
                    adjacent_points.push(point);
                }
            }
        }

        if point.y > 0
            && matches!(
                self[point],
                Tile::Startpoint | Tile::Pipe(Pipe::Vertical | Pipe::NE | Pipe::NW)
            )
        {
            let point = Point {
                x: point.x,
                y: point.y - 1,
            };
            let tile = self[point];
            if tile == Tile::Startpoint {
                adjacent_points.push(point);
            } else if let Tile::Pipe(pipe) = tile {
                if pipe == Pipe::Vertical || pipe == Pipe::SE || pipe == Pipe::SW {
                    adjacent_points.push(point);
                }
            }
        }

        if point.x < self.width() - 1
            && matches!(
                self[point],
                Tile::Startpoint | Tile::Pipe(Pipe::Horizontal | Pipe::SE | Pipe::NE)
            )
        {
            let point = Point {
                x: point.x + 1,
                y: point.y,
            };
            let tile = self[point];
            if tile == Tile::Startpoint {
                adjacent_points.push(point);
            } else if let Tile::Pipe(pipe) = tile {
                if pipe == Pipe::Horizontal || pipe == Pipe::NW || pipe == Pipe::SW {
                    adjacent_points.push(point);
                }
            }
        }

        if point.y < self.height() - 1
            && matches!(
                self[point],
                Tile::Startpoint | Tile::Pipe(Pipe::Vertical | Pipe::SW | Pipe::SE)
            )
        {
            let point = Point {
                x: point.x,
                y: point.y + 1,
            };
            let tile = self[point];
            if tile == Tile::Startpoint {
                adjacent_points.push(point);
            } else if let Tile::Pipe(pipe) = tile {
                if pipe == Pipe::Vertical || pipe == Pipe::NW || pipe == Pipe::NE {
                    adjacent_points.push(point);
                }
            }
        }
        adjacent_points
    }
}

impl std::ops::Index<usize> for Grid {
    type Output = Vec<Tile>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl std::ops::IndexMut<usize> for Grid {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl std::ops::Index<Point> for Grid {
    type Output = Tile;

    fn index(&self, point: Point) -> &Self::Output {
        &self.0[point.y][point.x]
    }
}

impl std::ops::IndexMut<Point> for Grid {
    fn index_mut(&mut self, point: Point) -> &mut Self::Output {
        &mut self.0[point.y][point.x]
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
struct Point {
    y: usize,
    x: usize,
}

impl Point {}

fn part1(input: &str) -> Result<usize, String> {
    let grid = Grid::parse(input)?;

    Ok(grid.get_path()?.len() / 2)
}

fn part2(input: &str) -> Result<usize, String> {
    let mut grid = Grid::parse(input)?;

    #[derive(Debug)]
    enum State {
        Inside(usize),
        OnPipe(bool, Pipe),
        Outside,
    }

    let path = {
        let mut path = grid.get_path()?;

        for p in &path {
            if grid[*p] == Tile::Startpoint {
                let mut connected = grid.connected_pipes(*p);
                connected.sort();

                assert!(connected.len() == 2);

                let (mut left, mut right, mut top, mut bottom) = (false, false, false, false);

                if p.y > 0 && (connected[0].y == p.y - 1 || connected[1].y == p.y - 1) {
                    top = true;
                }

                if p.x > 0 && (connected[0].x == p.x - 1 || connected[1].x == p.x - 1) {
                    left = true;
                }

                if connected[0].x == p.x + 1 || connected[1].x == p.x + 1 {
                    right = true;
                }

                if connected[0].y == p.y + 1 || connected[1].y == p.y + 1 {
                    bottom = true;
                }

                grid[*p] = if top && left {
                    Tile::Pipe(Pipe::NW)
                } else if top && bottom {
                    Tile::Pipe(Pipe::Vertical)
                } else if top && right {
                    Tile::Pipe(Pipe::NE)
                } else if left && right {
                    Tile::Pipe(Pipe::Horizontal)
                } else if left && bottom {
                    Tile::Pipe(Pipe::SW)
                } else if right && bottom {
                    Tile::Pipe(Pipe::SE)
                } else {
                    panic!("invalid startpoint")
                }
            }
        }

        path.sort();

        path
    };

    let mut sum = 0;

    for line in path.chunk_by(|left, right| left.y == right.y) {
        let mut state = State::Outside;

        for cell in line {
            state = match grid[*cell] {
                Tile::Ground => panic!("ground in path?"),
                Tile::Startpoint => panic!("startpoint still exists"),
                Tile::Pipe(pipe) => match pipe {
                    Pipe::Horizontal => state,
                    Pipe::Vertical => match state {
                        State::Inside(since) => {
                            sum += cell.x - since - 1;
                            State::Outside
                        }
                        State::OnPipe(inside, tile) => State::OnPipe(inside, tile),
                        State::Outside => State::Inside(cell.x),
                    },
                    Pipe::NE => match state {
                        State::Inside(since) => {
                            sum += cell.x - since - 1;
                            State::OnPipe(true, Pipe::NE)
                        }
                        State::Outside => State::OnPipe(false, Pipe::NE),
                        State::OnPipe(inside, last_pipe) => match last_pipe {
                            Pipe::Vertical => panic!(),
                            Pipe::Horizontal => panic!(),
                            Pipe::NE => panic!(),
                            Pipe::NW => {
                                if inside {
                                    State::Inside(cell.x)
                                } else {
                                    State::Outside
                                }
                            }
                            Pipe::SW => {
                                if inside {
                                    State::Outside
                                } else {
                                    State::Inside(cell.x)
                                }
                            }
                            Pipe::SE => panic!(),
                        },
                    },
                    Pipe::NW => match state {
                        State::Inside(since) => {
                            sum += cell.x - since - 1;
                            State::OnPipe(true, Pipe::NW)
                        }
                        State::Outside => State::OnPipe(false, Pipe::NW),
                        State::OnPipe(inside, last_pipe) => match last_pipe {
                            Pipe::Vertical => panic!(),
                            Pipe::Horizontal => panic!(),
                            Pipe::NW => panic!(),
                            Pipe::NE => {
                                if inside {
                                    State::Inside(cell.x)
                                } else {
                                    State::Outside
                                }
                            }
                            Pipe::SE => {
                                if inside {
                                    State::Outside
                                } else {
                                    State::Inside(cell.x)
                                }
                            }
                            Pipe::SW => panic!(),
                        },
                    },
                    Pipe::SW => match state {
                        State::Inside(since) => {
                            sum += cell.x - since - 1;
                            State::OnPipe(true, Pipe::SW)
                        }
                        State::Outside => State::OnPipe(false, Pipe::SW),
                        State::OnPipe(inside, last_pipe) => match last_pipe {
                            Pipe::Vertical => panic!(),
                            Pipe::Horizontal => panic!(),
                            Pipe::SW => panic!(),
                            Pipe::SE => {
                                if inside {
                                    State::Inside(cell.x)
                                } else {
                                    State::Outside
                                }
                            }
                            Pipe::NE => {
                                if inside {
                                    State::Outside
                                } else {
                                    State::Inside(cell.x)
                                }
                            }
                            Pipe::NW => panic!(),
                        },
                    },
                    Pipe::SE => match state {
                        State::Inside(since) => {
                            sum += cell.x - since - 1;
                            State::OnPipe(true, Pipe::SE)
                        }
                        State::Outside => State::OnPipe(false, Pipe::SE),
                        State::OnPipe(inside, last_pipe) => match last_pipe {
                            Pipe::Vertical => panic!(),
                            Pipe::Horizontal => panic!(),
                            Pipe::SE => panic!(),
                            Pipe::SW => {
                                if inside {
                                    State::Inside(cell.x)
                                } else {
                                    State::Outside
                                }
                            }
                            Pipe::NW => {
                                if inside {
                                    State::Outside
                                } else {
                                    State::Inside(cell.x)
                                }
                            }
                            Pipe::NE => panic!(),
                        },
                    },
                },
            };
        }
    }

    Ok(sum)
}

fn main() -> Result<(), String> {
    println!("part1: {}", part1(include_str!("../input"))?);
    println!("part2: {}", part2(include_str!("../input"))?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn part_1_example_1() {
        let input = indoc! {"
            ..F7.
            .FJ|.
            SJ.L7
            |F--J
            LJ...
        "};
        assert_eq!(part1(input).unwrap(), 8);
    }

    #[test]
    fn part_2_example_1() {
        let input = indoc! {"
            .F----7F7F7F7F-7....
            .|F--7||||||||FJ....
            .||.FJ||||||||L7....
            FJL7L7LJLJ||LJ.L-7..
            L--J.L7...LJS7F-7L7.
            ....F-J..F7FJ|L7L7L7
            ....L7.F7||L7|.L7L7|
            .....|FJLJ|FJ|F7|.LJ
            ....FJL-7.||.||||...
            ....L---J.LJ.LJLJ...
        "};
        assert_eq!(part2(input).unwrap(), 8);
    }

    #[test]
    fn part_2_example_2() {
        let input = indoc! {"
            FF7FSF7F7F7F7F7F---7
            L|LJ||||||||||||F--J
            FL-7LJLJ||||||LJL-77
            F--JF--7||LJLJ7F7FJ-
            L---JF-JLJ.||-FJLJJ7
            |F|F-JF---7F7-L7L|7|
            |FFJF7L7F-JF7|JL---7
            7-L-JL7||F7|L7F-7F7|
            L.L7LFJ|||||FJL7||LJ
            L7JLJL-JLJLJL--JLJ.L
        "};
        assert_eq!(part2(input).unwrap(), 10);
    }
}
