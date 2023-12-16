fn parse(input: &str) -> Vec<&[u8]> {
    input
        .trim_end()
        .split(',')
        .map(|seq| seq.as_bytes())
        .collect()
}

fn hash(seq: &[u8]) -> usize {
    seq.iter().fold(0, |mut acc, c| {
        acc += *c as usize;
        acc *= 17;
        acc %= 256;
        acc
    })
}

fn part1(input: &str) -> usize {
    parse(input).into_iter().map(hash).sum()
}

#[derive(Debug)]
struct Slot {
    label: Vec<u8>,
    focal: usize,
}

fn part2(input: &str) -> usize {
    let seqs = parse(input);
    const INIT: Option<Vec<Slot>> = None;
    let mut boxes: [Option<Vec<Slot>>; 256] = [INIT; 256];

    for seq in seqs {
        let sep = seq.iter().position(|c| *c == b'-' || *c == b'=').unwrap();
        let label = &seq[0..sep];
        let box_id = hash(label);
        let op = &seq[sep];
        let focal = seq.get(sep + 1).map(|&f| f - b'0').unwrap_or(0) as usize;

        match op {
            b'-' => {
                if let Some(ref mut bx) = boxes[box_id] {
                    if let Some(pos) = bx.iter().position(|slot| slot.label == label) {
                        bx.remove(pos);
                    }
                }
            }
            b'=' => {
                if boxes[box_id].is_none() {
                    boxes[box_id] = Some(Vec::new());
                }
                let bx = boxes[box_id].as_mut().unwrap(); //.insert(label, focal);
                if let Some(pos) = bx.iter().position(|slot| slot.label == label) {
                    bx[pos] = Slot {
                        label: label.to_vec(),
                        focal,
                    };
                } else {
                    bx.push(Slot {
                        label: label.to_vec(),
                        focal,
                    });
                }
            }
            _ => unreachable!(),
        }
    }

    boxes
        .iter()
        .enumerate()
        .filter(|(_, bx)| bx.is_some())
        .flat_map(|(bx_id, slots)| {
            slots
                .as_ref()
                .unwrap()
                .iter()
                .enumerate()
                .map(move |(slot_id, slot)| (bx_id + 1) * (slot_id + 1) * slot.focal)
        })
        .sum::<usize>()
}

fn main() {
    println!("part1={}", part1(include_str!("../input.txt")));
    println!("part2={}", part2(include_str!("../input.txt")));
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 1320);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 145);
    }
}
