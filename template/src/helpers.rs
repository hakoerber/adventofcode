#![allow(dead_code)]

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Point {
    pub y: usize,
    pub x: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Vector {
    pub y: isize,
    pub x: isize,
}

impl Vector {
    pub fn rev(&self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }

    pub fn mul(&self, m: isize) -> Self {
        Self {
            x: self.x * m,
            y: self.y * m,
        }
    }
}

impl Point {
    pub fn add(&self, v: Vector) -> Option<Self> {
        Some(Self {
            x: usize::try_from(isize::try_from(self.x).unwrap().checked_add(v.x).unwrap()).ok()?,
            y: usize::try_from(isize::try_from(self.y).unwrap().checked_add(v.y).unwrap()).ok()?,
        })
    }

    pub fn vector_to(&self, p: Self) -> Vector {
        Vector {
            x: isize::try_from(p.x).unwrap() - isize::try_from(self.x).unwrap(),
            y: isize::try_from(p.y).unwrap() - isize::try_from(self.y).unwrap(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Grid<T> {
    inner: Vec<T>,
    width: usize,
}

impl<T> std::iter::FromIterator<Vec<T>> for Grid<T> {
    fn from_iter<I: IntoIterator<Item = Vec<T>>>(iter: I) -> Self {
        let mut iter = iter.into_iter();
        let mut r = iter.next().unwrap();
        let width = r.len();
        for v in iter {
            assert_eq!(v.len(), width, "width differs between vecs");
            r.extend(v);
        }
        Self { inner: r, width }
    }
}

impl<T> Grid<T> {
    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.inner.len() / self.width
    }

    pub fn lines(&self) -> impl Iterator<Item = &[T]> {
        self.inner.chunks(self.width)
    }

    pub fn contains(&self, point: Point) -> bool {
        point.y < self.height() && point.x < self.width()
    }

    pub fn get(&self, point: Point) -> Option<&T> {
        if self.contains(point) {
            Some(&self.inner[point.y * self.width + point.x])
        } else {
            None
        }
    }
}

impl<T> Grid<T>
where
    T: Copy,
{
    pub fn iter(&self) -> GridIter<T> {
        GridIter {
            point: Point { x: 0, y: 0 },
            grid: self,
        }
    }
}

impl<T> Grid<T>
where
    T: From<char>,
{
    pub fn from_str(input: &str) -> Self {
        input
            .lines()
            .map(|line| line.chars().map(|c| c.into()).collect())
            .collect()
    }
}

impl<T> std::ops::Index<Point> for Grid<T>
where
    T: Copy,
{
    type Output = T;

    fn index(&self, index: Point) -> &Self::Output {
        self.get(index).unwrap()
    }
}

pub struct GridIter<'a, T> {
    point: Point,
    grid: &'a Grid<T>,
}

impl<'a, T> Iterator for GridIter<'a, T>
where
    T: Copy,
{
    type Item = (Point, T);

    fn next(&mut self) -> Option<Self::Item> {
        if self.point.x == self.grid.width() - 1 {
            self.point = Point {
                x: 0,
                y: self.point.y + 1,
            }
        } else {
            self.point = Point {
                x: self.point.x + 1,
                y: self.point.y,
            }
        }

        assert!(self.point.y <= self.grid.height());
        self.grid.get(self.point).map(|e| (self.point, *e))
    }
}

impl<T> IntoIterator for Grid<T> {
    type Item = (Point, T);

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        todo!()
    }
}

pub fn parse_into_fields<T, const LEN: usize, const SEP: char>(input: &str) -> [T; LEN]
where
    T: Copy + std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    let split = input.split(SEP).collect::<Vec<&str>>();
    assert_eq!(split.len(), LEN, "field count did not match");
    split
        .into_iter()
        .map(|e| e.parse::<T>().unwrap())
        .collect::<Vec<T>>()
        .as_slice()
        .try_into()
        .unwrap()
}

pub fn parse_into_vec<T, const SEP: char>(input: &str) -> Vec<T>
where
    T: Copy + std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    input
        .split(SEP)
        .map(|e| e.parse::<T>().unwrap())
        .collect::<Vec<T>>()
}

pub fn subgrids<T, U>(grid: &[U], size_y: usize, size_x: usize) -> Vec<Vec<&[T]>>
where
    T: Copy,
    U: AsRef<[T]>,
{
    let mut subgrids = Vec::new();
    for y in 0..=(grid.len() - size_y) {
        for x in 0..=(grid[y].as_ref().len() - size_x) {
            subgrids.push(
                grid[y..(y + size_y)]
                    .iter()
                    .map(|line| &line.as_ref()[x..(x + size_x)])
                    .collect::<Vec<_>>(),
            );
        }
    }

    subgrids
}

pub fn find_in_grid<T, U>(grid: &[U], needle: T) -> Vec<(usize, usize)>
where
    U: AsRef<[T]>,
    T: PartialEq<T> + Copy,
{
    iter_grid(grid)
        .filter_map(move |(y, x)| {
            if grid[y].as_ref()[x] == needle {
                Some((y, x))
            } else {
                None
            }
        })
        .collect()
}

pub fn iter_grid<'a, T, U>(grid: &'a [U]) -> impl Iterator<Item = (usize, usize)> + use<'a, T, U>
where
    U: AsRef<[T]>,
    T: PartialEq<T> + Copy + 'a,
{
    grid.iter().enumerate().flat_map(|(y, line)| {
        let i = line
            .as_ref()
            .iter()
            .enumerate()
            .map(move |(x, _elem)| (y, x));

        i
    })
}
