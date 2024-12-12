use crate::helpers::{Grid, Point, PointCloud, Vector};

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
    points: PointCloud,
}

impl Region {
    fn perimeter(&self) -> usize {
        self.points
            .iter()
            .map(|point| {
                point
                    .all_neighbors()
                    .filter(|neighbor| {
                        neighbor
                            .as_ref()
                            .map_or(true, |neighbor| !self.points.contains(neighbor))
                    })
                    .count()
            })
            .sum()
    }

    fn sides(&self) -> usize {
        let mut sides = 0;

        let grid = self.points.clone().into_bool_grid();

        // rows
        for row in grid.rows() {
            for y in [-1, 1] {
                let mut inside_run = false;
                for (point, occupied) in row.points() {
                    if occupied {
                        let is_border = !point
                            .add(&Vector { y, x: 0 })
                            .and_then(|p| grid.get(&p))
                            .unwrap_or(&false);

                        if !is_border && inside_run {
                            sides += 1;
                        }
                        inside_run = is_border;
                    } else if inside_run {
                        inside_run = false;
                        sides += 1;
                    }
                }
                if inside_run {
                    sides += 1;
                }
            }
        }

        // rows
        for col in grid.cols() {
            for x in [-1, 1] {
                let mut inside_run = false;
                for (point, occupied) in col.points() {
                    if occupied {
                        let is_border = !point
                            .add(&Vector { y: 0, x })
                            .and_then(|p| grid.get(&p))
                            .unwrap_or(&false);

                        if !is_border && inside_run {
                            sides += 1;
                        }
                        inside_run = is_border;
                    } else if inside_run {
                        inside_run = false;
                        sides += 1;
                    }
                }
                if inside_run {
                    sides += 1;
                }
            }
        }

        sides
    }
}

fn find_region_from(grid: &mut Grid<Option<char>>, point: &Point, id: char) -> PointCloud {
    assert_eq!(
        grid.get(point)
            .expect("must be in grid")
            .expect("must not be already visited"),
        id
    );
    let mut points = PointCloud::from(vec![point.clone()]);
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

fn find_regions(grid: &Grid<char>) -> impl Iterator<Item = Region> {
    let mut regions = Vec::new();

    // use None to represent a plot in a region that is already accounted for
    let mut grid = grid.clone().transform(Some);

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

    regions.into_iter()
}

pub fn part_1(input: &Input) -> crate::Output {
    find_regions(&input.grid)
        .map(|region| region.points.area() * region.perimeter())
        .sum::<usize>()
        .into()
}

pub fn part_2(input: &Input) -> crate::Output {
    find_regions(&input.grid)
        .map(|region| region.points.area() * region.sides())
        .sum::<usize>()
        .into()
}

#[cfg(test)]
pub const EXAMPLE_RESULTS: [Option<crate::Output>; 2] = [
    Some(crate::Output::Int(1930)),
    Some(crate::Output::Int(1206)),
];
