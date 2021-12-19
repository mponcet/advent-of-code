#[derive(Default)]
struct Position {
    depth: u32,
    x: u32,
    aim: u32,
}

fn part1() -> u32 {
    let input = include_str!("../input.txt");
    let mut position = Position::default();

    for line in input.lines() {
        let (action, count) = line.split_once(" ").unwrap();
        let count: u32 = count.parse().unwrap();

        match action {
            "forward" => position.x = position.x.checked_add(count).unwrap(),
            "down" => position.depth = position.depth.checked_add(count).unwrap(),
            "up" => position.depth = position.depth.checked_sub(count).unwrap(),
            _ => panic!("Unexpected keyword"),
        }
    }

    position.depth * position.x
}

fn part2() -> u32 {
    let input = include_str!("../input.txt");
    let mut position = Position::default();

    for line in input.lines() {
        let (action, count) = line.split_once(" ").unwrap();
        let count: u32 = count.parse().unwrap();

        match action {
            "forward" => {
                position.x = position.x.checked_add(count).unwrap();
                position.depth = position.depth.checked_add(position.aim * count).unwrap();
            }
            "down" => position.aim = position.aim.checked_add(count).unwrap(),
            "up" => position.aim = position.aim.checked_sub(count).unwrap(),
            _ => panic!("Unexpected keyword"),
        }
    }

    position.depth * position.x
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        println!("answer: {}", part1());
    }

    #[test]
    fn test_part2() {
        println!("answer: {}", part2());
    }
}
