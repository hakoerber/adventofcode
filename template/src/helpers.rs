#![allow(dead_code)]

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
