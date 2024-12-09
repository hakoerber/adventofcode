mod helpers;
mod output;
use std::cmp;

use output::Output;

#[derive(Debug, Clone)]
struct Input(Vec<Entry>);

fn parse(input: &str) -> Input {
    Input(
        input
            .trim_end()
            .chars()
            .map(|c| u8::try_from(c.to_digit(10).unwrap()).unwrap())
            .collect::<Vec<u8>>()
            .chunks(2)
            .enumerate()
            .flat_map(|(id, chunk)| {
                let file = Entry::File {
                    blocks: chunk[0],
                    id,
                };
                if chunk.len() == 1 {
                    vec![file]
                } else {
                    vec![file, Entry::Free { blocks: chunk[1] }]
                }
            })
            .collect(),
    )
}

#[derive(Clone, Copy)]
enum Entry {
    File { blocks: u8, id: usize },
    Free { blocks: u8 },
}

impl std::fmt::Debug for Entry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::File { blocks, id } => write!(f, "F[{id}]<{blocks}>"),
            Self::Free { blocks } => write!(f, "X<{blocks}>"),
        }
    }
}

impl std::fmt::Display for Input {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for entry in &self.0 {
            match entry {
                Entry::File { blocks, id } => {
                    for _ in 0..*blocks {
                        write!(f, "{id}")?;
                    }
                }
                Entry::Free { blocks } => {
                    for _ in 0..*blocks {
                        write!(f, ".")?;
                    }
                }
            }
        }
        Ok(())
    }
}

fn part_1(input: &Input) -> Output {
    let mut entries = input.0.clone();

    let mut last_file_index = entries
        .iter()
        .enumerate()
        .rev()
        .find(|(_i, entry)| matches!(entry, Entry::File { .. }))
        .unwrap()
        .0;

    'out: for mut i in 0..entries.len() {
        if let Entry::Free { blocks } = entries[i] {
            let mut required_blocks = blocks;
            if i >= last_file_index {
                break;
            }
            // first, we take as much as needed from the last block
            let Entry::File {
                blocks: last_file_blocks,
                id: last_file_id,
            } = entries[last_file_index]
            else {
                panic!("last block is not actually a file")
            };

            let blocks_to_take = cmp::min(required_blocks, last_file_blocks);

            entries[i] = Entry::File {
                blocks: blocks_to_take,
                id: last_file_id,
            };

            // update the last file block
            assert!(blocks_to_take <= required_blocks);
            if blocks_to_take == last_file_blocks {
                // we exhausted the last file completely
                entries[last_file_index] = Entry::Free {
                    blocks: last_file_blocks,
                };

                for j in (0..(last_file_index - 1)).rev() {
                    if j <= i {
                        break 'out;
                    }
                    if matches!(entries[j], Entry::File { .. }) {
                        last_file_index = j;
                        break;
                    }
                }
            } else {
                entries[last_file_index] = Entry::File {
                    blocks: last_file_blocks - blocks_to_take,
                    id: last_file_id,
                };
            }

            required_blocks -= blocks_to_take;

            // if we still need more blocks, we iterate
            while required_blocks > 0 {
                // first, we take as much as needed from the last block
                let Entry::File {
                    blocks: last_file_blocks,
                    id: last_file_id,
                } = entries[last_file_index]
                else {
                    panic!("last block is not actually a file")
                };

                let blocks_to_take = cmp::min(required_blocks, last_file_blocks);

                entries.insert(
                    i + 1,
                    Entry::File {
                        blocks: blocks_to_take,
                        id: last_file_id,
                    },
                );

                // update indices, as we shifted elements via insert()
                i += 1;
                last_file_index += 1;

                // update the last file block
                if blocks_to_take == last_file_blocks {
                    // we exhausted the last file completely
                    entries[last_file_index] = Entry::Free {
                        blocks: last_file_blocks,
                    };

                    for j in (0..(last_file_index - 1)).rev() {
                        if j <= i {
                            break 'out;
                        }
                        if matches!(entries[j], Entry::File { .. }) {
                            last_file_index = j;
                            break;
                        }
                    }
                } else {
                    entries[last_file_index] = Entry::File {
                        blocks: last_file_blocks - blocks_to_take,
                        id: last_file_id,
                    };
                }

                required_blocks -= blocks_to_take;
            }
        }
    }

    let mut acc = 0;
    let mut i = 0;
    for entry in entries {
        if let Entry::File { blocks, id } = entry {
            for _j in 0..blocks {
                acc += i * id;
                i += 1;
            }
        }
    }

    acc.into()
}

fn part_2(input: &Input) -> Output {
    let mut entries = input.0.clone();

    for i in (0..entries.len()).rev() {
        let mut last_file_index = 0;
        for j in (0..=i).rev() {
            if matches!(entries[j], Entry::File { .. }) {
                last_file_index = j;
                break;
            }
        }
        let Entry::File {
            blocks: file_to_place_blocks,
            id: _,
        } = entries[last_file_index]
        else {
            panic!("last block is not actually a file")
        };

        let mut placement_index = None;
        for (i, entry) in entries.iter().enumerate() {
            if let Entry::Free { blocks } = entry {
                if *blocks >= file_to_place_blocks {
                    placement_index = Some(i);
                    break;
                }
            }
        }

        if let Some(placement_index) = placement_index {
            if placement_index < last_file_index {
                let Entry::File {
                    blocks: file_to_place_blocks,
                    id: file_to_place_id,
                } = entries[last_file_index]
                else {
                    panic!("last block is not actually a file")
                };

                let Entry::Free {
                    blocks: placement_blocks,
                } = entries[placement_index]
                else {
                    panic!("last block is not actually a file")
                };

                assert!(placement_blocks >= file_to_place_blocks);

                if placement_blocks == file_to_place_blocks {
                    // it fits perfectly, awesome!
                    entries.swap(placement_index, last_file_index);
                } else {
                    // we have to account for the remaining free space
                    entries[placement_index] = Entry::File {
                        blocks: file_to_place_blocks,
                        id: file_to_place_id,
                    };

                    entries[last_file_index] = Entry::Free {
                        blocks: file_to_place_blocks,
                    };
                    let remaining_free_space = placement_blocks - file_to_place_blocks;
                    entries.insert(
                        placement_index + 1,
                        Entry::Free {
                            blocks: remaining_free_space,
                        },
                    );
                }
            }
        }
    }

    let mut acc = 0;
    let mut i = 0_usize;
    for entry in &entries {
        match entry {
            Entry::File { blocks, id } => {
                for _j in 0..*blocks {
                    acc += i * id;
                    i += 1;
                }
            }
            Entry::Free { blocks } => i += *blocks as usize,
        }
    }

    acc.into()
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
    const EXAMPLE_RESULT_PART_1: Output = Output::Int(1928);
    const EXAMPLE_RESULT_PART_2: Output = Output::Int(2858);

    use super::*;

    fn example_input() -> Input {
        parse(&std::fs::read_to_string("example").unwrap())
    }

    #[test]
    fn example_part_1() {
        assert_eq!(part_1(&example_input()), EXAMPLE_RESULT_PART_1);
    }

    #[test]
    fn example_part_2() {
        assert_eq!(part_2(&example_input()), EXAMPLE_RESULT_PART_2);
    }
}
