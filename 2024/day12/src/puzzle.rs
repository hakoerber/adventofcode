use crate::helpers::{Grid, Point};

#[derive(Debug, Clone)]
pub struct Input {
    grid: Grid<char>,
}

pub fn parse(input: &str) -> Input {
    Input {
        grid: Grid::from_str(input),
    }
}

#[derive(Debug, Clone)]
pub struct Region {
    #[allow(dead_code)]
    id: char,
    points: Vec<Point>,
}

impl Region {
    fn area(&self) -> usize {
        self.points.len()
    }

    fn perimeter(&self) -> usize {
        let mut acc = 0;
        for point in &self.points {
            if point.x == 0 {
                acc += 1;
            } else if !self.points.iter().any(|p| {
                *p == Point {
                    x: point.x - 1,
                    y: point.y,
                }
            }) {
                acc += 1;
            }
            if !self.points.iter().any(|p| {
                *p == Point {
                    x: point.x + 1,
                    y: point.y,
                }
            }) {
                acc += 1;
            }
            if point.y == 0 {
                acc += 1;
            } else if !self.points.iter().any(|p| {
                *p == Point {
                    x: point.x,
                    y: point.y - 1,
                }
            }) {
                acc += 1;
            }
            if !self.points.iter().any(|p| {
                *p == Point {
                    x: point.x,
                    y: point.y + 1,
                }
            }) {
                acc += 1;
            }
        }
        acc
    }

    fn sides(&self) -> usize {
        let mut sides = 0;

        let min_x = self.points.iter().map(|p| p.x).min().unwrap();
        let max_x = self.points.iter().map(|p| p.x).max().unwrap();
        let min_y = self.points.iter().map(|p| p.y).min().unwrap();
        let max_y = self.points.iter().map(|p| p.y).max().unwrap();

        // rows
        for row in min_y..=max_y {
            // above
            let mut inside_run = false;
            for col in min_x..=max_x {
                if !self.points.iter().any(|p| *p == Point { y: row, x: col }) {
                    if inside_run {
                        inside_run = false;
                        sides += 1;
                    }
                    continue;
                }
                let has_neighbor = if row == min_y {
                    false
                } else {
                    self.points
                        .iter()
                        .any(|p| row > 0 && *p == Point { y: row - 1, x: col })
                };

                if has_neighbor {
                    if inside_run {
                        sides += 1;
                    }
                    inside_run = false;
                } else if !inside_run {
                    inside_run = true;
                }
            }
            if inside_run {
                sides += 1;
            }

            // below
            let mut inside_run = false;
            for col in min_x..=max_x {
                if !self.points.iter().any(|p| *p == Point { y: row, x: col }) {
                    if inside_run {
                        inside_run = false;
                        sides += 1;
                    }
                    continue;
                }
                let has_neighbor = self
                    .points
                    .iter()
                    .any(|p| *p == Point { y: row + 1, x: col });

                if has_neighbor {
                    if inside_run {
                        sides += 1;
                    }
                    inside_run = false;
                } else if !inside_run {
                    inside_run = true;
                }
            }

            if inside_run {
                sides += 1;
            }
        }

        // cols
        for col in min_x..=max_x {
            // above
            let mut inside_run = false;
            for row in min_y..=max_y {
                if !self.points.iter().any(|p| *p == Point { y: row, x: col }) {
                    if inside_run {
                        inside_run = false;
                        sides += 1;
                    }
                    continue;
                }
                let has_neighbor = if col == min_x {
                    false
                } else {
                    self.points
                        .iter()
                        .any(|p| col > 0 && *p == Point { y: row, x: col - 1 })
                };

                if has_neighbor {
                    if inside_run {
                        sides += 1;
                    }
                    inside_run = false;
                } else {
                    inside_run = true;
                }
            }

            if inside_run {
                sides += 1;
            }

            // below
            let mut inside_run = false;
            for row in min_y..=max_y {
                if !self.points.iter().any(|p| *p == Point { y: row, x: col }) {
                    if inside_run {
                        inside_run = false;
                        sides += 1;
                    }
                    continue;
                }
                let has_neighbor = self
                    .points
                    .iter()
                    .any(|p| *p == Point { y: row, x: col + 1 });

                if has_neighbor {
                    if inside_run {
                        sides += 1;
                    }
                    inside_run = false;
                } else {
                    inside_run = true;
                }
            }

            if inside_run {
                sides += 1;
            }
        }

        sides
    }
}

fn find_regions(grid: &Grid<char>) -> Vec<Region> {
    let mut regions = Vec::new();

    // use None to represent a plot in a region that is already accounted for
    let mut grid = grid.clone().transform(Some);

    fn find_region_from(grid: &mut Grid<Option<char>>, point: &Point, id: char) -> Vec<Point> {
        assert_eq!(
            grid.get(point)
                .expect("must be in grid")
                .expect("must not be already visited"),
            id
        );
        let mut points = vec![point.clone()];
        let id = grid.get_mut(point).unwrap().take().unwrap();

        let neighbors = grid
            .neighbors_of(point)
            .map(|gridpoint| gridpoint.point)
            .collect::<Vec<Point>>();

        for neighbor in neighbors {
            if grid[&neighbor].is_some_and(|c| c == id) {
                points.extend(find_region_from(grid, &neighbor, id));
            }
        }

        points
    }

    loop {
        let next = grid.iter().find(|(_point, elem)| elem.is_some());

        if let Some((point, elem)) = next {
            let elem = elem.unwrap();
            regions.push(Region {
                id: elem,
                points: find_region_from(&mut grid, &point, elem),
            });
        } else {
            break;
        }
    }

    regions
}

pub fn part_1(input: &Input) -> crate::Output {
    find_regions(&input.grid)
        .into_iter()
        .map(|region| region.area() * region.perimeter())
        .sum::<usize>()
        .into()
}

pub fn part_2(input: &Input) -> crate::Output {
    find_regions(&input.grid)
        .into_iter()
        .map(|region| region.area() * region.sides())
        .sum::<usize>()
        .into()
}

#[cfg(test)]
pub const EXAMPLE_RESULTS: [Option<crate::Output>; 2] = [
    Some(crate::Output::Int(1930)),
    Some(crate::Output::Int(1206)),
];
