#[derive(Clone, Copy, Debug)]
enum Block {
    File { id: u32, len: u32 },
    Free { free: u32 },
}

fn parse(input: &str) -> Vec<Block> {
    input
        .trim_end()
        .chars()
        .enumerate()
        .filter_map(|(id, c)| {
            let c = c.to_digit(10).unwrap();
            if id % 2 == 0 {
                Some(Block::File {
                    id: id as u32 / 2,
                    len: c,
                })
            } else if c != 0 {
                Some(Block::Free { free: c })
            } else {
                None
            }
        })
        .collect()
}

fn checksum(blocks: &[Block]) -> usize {
    let mut pos = 0;
    blocks
        .iter()
        .map(|block| match *block {
            Block::File { id, len } => {
                let s = (pos..pos + len).map(|i| i * id).sum::<u32>() as usize;
                pos += len;
                s
            }
            Block::Free { free } => {
                pos += free;
                0
            }
        })
        .sum()
}

fn part1(input: &str) -> usize {
    let mut blocks = parse(input);

    loop {
        let Some(position_first_free_block) = blocks
            .iter()
            .position(|block| matches!(block, Block::Free { .. }))
        else {
            break;
        };

        let Some(position_last_file_block) = blocks
            .iter()
            .enumerate()
            .rfind(|&(_, block)| matches!(block, Block::File { .. }))
            .map(|(i, _)| i)
        else {
            break;
        };

        if position_first_free_block < position_last_file_block {
            let file_block = *blocks.get(position_last_file_block).unwrap();
            let free_block = *blocks.get(position_first_free_block).unwrap();

            match (free_block, file_block) {
                (Block::Free { free }, Block::File { len, .. }) if free == len => {
                    blocks.swap(position_first_free_block, position_last_file_block);
                }
                (Block::Free { free }, Block::File { id, len }) if free < len => {
                    let free_block = blocks.get_mut(position_first_free_block).unwrap();
                    *free_block = Block::File { id, len: free };
                    let file_block = blocks.get_mut(position_last_file_block).unwrap();
                    *file_block = Block::File {
                        id,
                        len: len - free,
                    };
                }
                (Block::Free { free }, Block::File { id, len }) if free > len => {
                    let free_block = blocks.get_mut(position_first_free_block).unwrap();
                    *free_block = Block::File { id, len };
                    let file_block = blocks.get_mut(position_last_file_block).unwrap();
                    *file_block = Block::Free { free: len };
                    blocks.insert(
                        position_first_free_block + 1,
                        Block::Free { free: free - len },
                    );
                }
                _ => break,
            }
        } else {
            break;
        }
    }

    checksum(&blocks)
}

fn part2(input: &str) -> usize {
    let mut blocks = parse(input);
    let Block::File { id, .. } = blocks
        .iter()
        .rfind(|block| matches!(block, Block::File { .. }))
        .unwrap()
    else {
        panic!("");
    };

    let mut last_id = *id;

    while last_id > 0 {
        let Some(position_file_block) = blocks
            .iter()
            .position(|block| matches!(block, Block::File { id, .. } if *id == last_id))
        else {
            break;
        };

        last_id -= 1;

        let file_block = *blocks.get(position_file_block).unwrap();
        let Some(position_free_block) = blocks
            .iter()
            .position(|block| matches!((block, file_block), (Block::Free { free }, Block::File { len, ..}) if *free >= len))
        else {
            continue;
        };

        if position_free_block < position_file_block {
            let file_block = *blocks.get(position_file_block).unwrap();
            let free_block = *blocks.get(position_free_block).unwrap();

            match (free_block, file_block) {
                (Block::Free { free }, Block::File { len, .. }) if free == len => {
                    blocks.swap(position_free_block, position_file_block);
                }
                (Block::Free { free }, Block::File { id, len }) if free > len => {
                    let free_block = blocks.get_mut(position_free_block).unwrap();
                    *free_block = Block::File { id, len };
                    let file_block = blocks.get_mut(position_file_block).unwrap();
                    *file_block = Block::Free { free: len };
                    blocks.insert(position_free_block + 1, Block::Free { free: free - len });
                }
                _ => continue,
            }
        }
    }

    checksum(&blocks)
}

fn main() {
    println!("part1={}", part1(include_str!("../input.txt")));
    println!("part2={}", part2(include_str!("../input.txt")));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "2333133121414131402";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 1928);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 2858);
    }
}
