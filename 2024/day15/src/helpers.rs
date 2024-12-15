#![allow(dead_code)]

use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

#[derive(Debug, Clone)]
pub struct PointCloud(Vec<Point>);

impl PointCloud {
    pub fn from(v: Vec<Point>) -> Self {
        Self(v)
    }

    pub fn area(&self) -> usize {
        self.0.len()
    }

    pub fn iter(&self) -> impl Iterator<Item = &Point> {
        self.0.iter()
    }

    pub fn into_iter(self) -> impl Iterator<Item = Point> {
        self.0.into_iter()
    }

    pub fn extend(&mut self, other: Self) {
        self.0.extend(other.0);
    }

    pub fn contains(&self, point: &Point) -> bool {
        self.0.iter().any(|p| p == point)
    }

    pub fn into_bool_grid(self) -> Grid<bool> {
        let mut points = self.0;
        points.sort_by_key(|p| (p.y, p.x));
        let min_x = points.iter().map(|p| p.x).min().unwrap();
        let min_y = points.iter().map(|p| p.y).min().unwrap();
        let max_x = points.iter().map(|p| p.x).max().unwrap();
        let max_y = points.iter().map(|p| p.y).max().unwrap();

        let mut points = points.into_iter();
        let mut next = points.next();
        let mut v = Vec::new();
        for y in min_y..=max_y {
            for x in min_x..=max_x {
                if next.as_ref().is_some_and(|p| *p == Point { x, y }) {
                    v.push(true);
                    next = points.next();
                } else {
                    v.push(false);
                }
            }
        }
        Grid {
            inner: v,
            width: max_x - min_x + 1,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Vector {
    pub x: isize,
    pub y: isize,
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
    pub fn add(&self, v: &Vector) -> Option<Self> {
        Some(Self {
            x: usize::try_from(isize::try_from(self.x).unwrap().checked_add(v.x).unwrap()).ok()?,
            y: usize::try_from(isize::try_from(self.y).unwrap().checked_add(v.y).unwrap()).ok()?,
        })
    }

    pub fn vector_to(&self, p: &Self) -> Vector {
        Vector {
            x: isize::try_from(p.x).unwrap() - isize::try_from(self.x).unwrap(),
            y: isize::try_from(p.y).unwrap() - isize::try_from(self.y).unwrap(),
        }
    }

    pub fn neighbors(&self) -> impl Iterator<Item = Self> {
        self.all_neighbors().flatten()
    }

    pub fn all_neighbors(&self) -> impl Iterator<Item = Option<Self>> {
        let mut v = Vec::new();
        for (x, y) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
            v.push(self.add(&Vector { x, y }));
        }
        v.into_iter()
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

#[derive(Debug, Clone)]
pub struct GridRow<'a, T> {
    pub cells: &'a [T],
    pub y: usize,
}

impl<'a, T> GridRow<'a, T>
where
    T: Copy,
{
    pub fn points(&'a self) -> impl Iterator<Item = (Point, T)> + use<'a, T> {
        self.cells
            .iter()
            .enumerate()
            .map(|(x, cell)| (Point { y: self.y, x }, *cell))
    }
}

#[derive(Debug, Clone)]
pub struct GridCol<T> {
    pub cells: Vec<T>,
    pub x: usize,
}

impl<T> GridCol<T>
where
    T: Copy,
{
    pub fn points(&self) -> impl Iterator<Item = (Point, T)> + use<'_, T> {
        self.cells
            .iter()
            .enumerate()
            .map(|(y, cell)| (Point { y, x: self.x }, *cell))
    }
}

impl<T> Grid<T> {
    pub fn width(&self) -> usize {
        self.width
    }

    pub fn transform<U>(self, f: impl FnMut(T) -> U) -> Grid<U> {
        Grid {
            inner: self.inner.into_iter().map(f).collect(),
            width: self.width,
        }
    }

    pub fn flat_transform<const W: usize, U>(&self, f: impl FnMut(&T) -> [U; W]) -> Grid<U> {
        Grid {
            inner: self.inner.iter().flat_map(f).collect(),
            width: self.width * W,
        }
    }

    pub fn height(&self) -> usize {
        self.inner.len() / self.width
    }

    pub fn rows(&self) -> impl Iterator<Item = GridRow<T>> {
        self.inner
            .chunks(self.width)
            .enumerate()
            .map(|(y, cells)| GridRow { cells, y })
    }

    pub fn contains(&self, point: &Point) -> bool {
        point.y < self.height() && point.x < self.width()
    }

    pub fn get(&self, point: &Point) -> Option<&T> {
        if self.contains(point) {
            Some(&self.inner[point.y * self.width + point.x])
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, point: &Point) -> Option<&mut T> {
        if self.contains(point) {
            Some(&mut self.inner[point.y * self.width + point.x])
        } else {
            None
        }
    }

    pub fn neighbors_of<'a>(&'a self, point: &Point) -> impl Iterator<Item = GridPoint<'a, T>> {
        let mut v = Vec::new();
        for point in point.neighbors() {
            if let Some(value) = self.get(&point) {
                v.push(GridPoint { value, point });
            }
        }
        v.into_iter()
    }
}

impl<T> Display for Grid<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in self.rows() {
            for cell in line.cells {
                write!(f, "{cell}")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct GridPoint<'a, T> {
    pub value: &'a T,
    pub point: Point,
}

#[derive(Debug)]
pub struct GridPointOwned<T> {
    pub value: T,
    pub point: Point,
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

    pub fn neighbors_of_copy(&self, point: &Point) -> impl Iterator<Item = GridPointOwned<T>> {
        let mut v = Vec::new();
        for point in point.neighbors() {
            if let Some(value) = self.get(&point) {
                v.push(GridPointOwned {
                    value: *value,
                    point,
                });
            }
        }
        v.into_iter()
    }

    pub fn cols(&self) -> impl Iterator<Item = GridCol<T>> + use<'_, T> {
        (0..self.width()).map(|x| GridCol {
            x,
            cells: self
                .inner
                .iter()
                .skip(x)
                .step_by(self.width())
                .copied()
                .collect(),
        })
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

impl<T> Grid<T> {
    pub fn from_str_with<F: Fn(char) -> T + Copy>(input: &str, f: F) -> Self {
        input
            .lines()
            .map(|line| line.chars().map(f).collect())
            .collect()
    }
}

impl Grid<u8> {
    pub fn from_str_as_digits(input: &str) -> Self {
        Self::from_str_with(input, |c| u8::try_from(c.to_digit(10).unwrap()).unwrap())
    }
}

impl<T> std::ops::Index<&Point> for Grid<T>
where
    T: Copy,
{
    type Output = T;

    fn index(&self, index: &Point) -> &Self::Output {
        self.get(index).unwrap()
    }
}

pub struct GridIter<'a, T> {
    point: Point,
    grid: &'a Grid<T>,
}

impl<T> Iterator for GridIter<'_, T>
where
    T: Copy,
{
    type Item = (Point, T);

    fn next(&mut self) -> Option<Self::Item> {
        assert!(self.point.y <= self.grid.height());
        let elem = self.grid.get(&self.point).map(|e| (self.point.clone(), *e));

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

        elem
    }
}

impl<T> IntoIterator for Grid<T> {
    type Item = (Point, T);

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        todo!()
    }
}

#[derive(Debug, Clone)]
pub struct Path(pub Vec<Point>);

impl Display for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, point) in self.0.iter().enumerate() {
            write!(f, "({},{})", point.y, point.x)?;
            if i != self.0.len() - 1 {
                write!(f, " -> ")?;
            }
        }
        Ok(())
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

pub fn number_of_digits(num: usize) -> u32 {
    num.ilog10() + 1
}

pub fn unique<T>(v: impl Iterator<Item = T>) -> Vec<T>
where
    T: PartialEq<T> + Clone,
{
    let mut result = Vec::new();
    for elem in v {
        if !result.iter().any(|e| *e == elem) {
            result.push(elem.clone());
        }
    }
    result
}

pub fn whole_div(a: isize, b: isize) -> Option<isize> {
    if b == 0 || a % b != 0 {
        None
    } else {
        Some(a / b)
    }
}
