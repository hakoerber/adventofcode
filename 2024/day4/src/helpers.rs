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
            )
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
    grid.iter()
        .enumerate()
        .map(|(y, line)| {
            let i = line
                .as_ref()
                .iter()
                .enumerate()
                .map(move |(x, _elem)| (y, x));

            i
        })
        .flatten()
        .into_iter()
}
