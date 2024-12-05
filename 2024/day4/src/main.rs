mod helpers;

#[derive(Debug, Clone)]
struct Input {
    grid: Vec<Vec<char>>,
}

fn parse(input: &str) -> Input {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    Input { grid }
}

fn part_1(input: &Input) -> usize {
    let mut count = 0;
    let grid = &input.grid;
    let height = grid.len();
    let width = grid[0].len();
    for (y, x) in helpers::find_in_grid(grid, 'X') {
        let space_right = x <= width - 4;
        let space_left = x >= 3;
        let space_top = y >= 3;
        let space_bottom = y <= height - 4;

        if space_right {
            if grid[y][x + 1] == 'M' && grid[y][x + 2] == 'A' && grid[y][x + 3] == 'S' {
                count += 1;
            }
        }
        if space_right && space_bottom {
            if grid[y + 1][x + 1] == 'M' && grid[y + 2][x + 2] == 'A' && grid[y + 3][x + 3] == 'S' {
                count += 1;
            }
        }
        if space_bottom {
            if grid[y + 1][x] == 'M' && grid[y + 2][x] == 'A' && grid[y + 3][x] == 'S' {
                count += 1;
            }
        }
        if space_bottom && space_left {
            if grid[y + 1][x - 1] == 'M' && grid[y + 2][x - 2] == 'A' && grid[y + 3][x - 3] == 'S' {
                count += 1;
            }
        }
        if space_left {
            if grid[y][x - 1] == 'M' && grid[y][x - 2] == 'A' && grid[y][x - 3] == 'S' {
                count += 1;
            }
        }
        if space_left && space_top {
            if grid[y - 1][x - 1] == 'M' && grid[y - 2][x - 2] == 'A' && grid[y - 3][x - 3] == 'S' {
                count += 1;
            }
        }
        if space_top {
            if grid[y - 1][x] == 'M' && grid[y - 2][x] == 'A' && grid[y - 3][x] == 'S' {
                count += 1;
            }
        }
        if space_top && space_right {
            if grid[y - 1][x + 1] == 'M' && grid[y - 2][x + 2] == 'A' && grid[y - 3][x + 3] == 'S' {
                count += 1;
            }
        }
    }
    count
}

fn part_2(input: &Input) -> usize {
    let mut count = 0;
    let grid = &input.grid;
    for subgrid in helpers::subgrids(grid, 3, 3) {
        if subgrid[1][1] == 'A' {
            let top_left = subgrid[0][0];
            let top_right = subgrid[0][2];
            let bottom_left = subgrid[2][0];
            let bottom_right = subgrid[2][2];

            if ((top_left == 'M' && bottom_right == 'S')
                || (top_left == 'S' && bottom_right == 'M'))
                && ((top_right == 'M' && bottom_left == 'S')
                    || (top_right == 'S' && bottom_left == 'M'))
            {
                count += 1;
            }
        }
    }
    count
}

fn main() {
    let input = parse(&std::fs::read_to_string("input").expect("input could not be read"));

    match std::env::args().nth(1) {
        Some(s) if s == "1" => println!("part 1: {}", part_1(&input)),
        Some(s) if s == "2" => println!("part 2: {}", part_2(&input)),
        _ => panic!("specify part"),
    }
}

#[cfg(test)]
mod test {
    const EXAMPLE_RESULT_PART_1: usize = 18;
    const EXAMPLE_RESULT_PART_2: usize = 9;

    use super::*;

    fn example_input() -> Input {
        parse(&std::fs::read_to_string("example").unwrap())
    }

    #[test]
    fn example_part_1() {
        assert_eq!(part_1(&example_input()), EXAMPLE_RESULT_PART_1)
    }

    #[test]
    fn example_part_2() {
        assert_eq!(part_2(&example_input()), EXAMPLE_RESULT_PART_2)
    }
}
