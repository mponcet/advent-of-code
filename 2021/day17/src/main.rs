type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn parse_input() -> Result<((i32, i32), (i32, i32))> {
    let coords: Vec<_> = include_str!("../input.txt")
        .split(' ')
        .skip(2)
        .filter_map(|s| s[2..].trim_end_matches(",").split_once(".."))
        .filter_map(
            |(p1, p2)| match (p1.trim().parse::<i32>(), p2.trim().parse::<i32>()) {
                (Ok(p1), Ok(p2)) => Some((p1, p2)),
                _ => None,
            },
        )
        .collect();

    if coords.len() == 2 {
        Ok((coords[0], coords[1]))
    } else {
        Err("parsing failed".into())
    }
}

fn hit_area(mut vx: i32, mut vy: i32, target: ((i32, i32), (i32, i32))) -> bool {
    let ((x1, x2), (y1, y2)) = target;
    let (mut x, mut y) = (0, 0);

    while x <= x2 && y >= y1 {
        x += vx;
        y += vy;
        if (x1..=x2).contains(&x) && (y1..=y2).contains(&y) {
            return true;
        }
        vx = (vx - 1).max(0);
        vy -= 1;
    }

    false
}

fn part1() -> Result<i32> {
    let ((_, _), (y1, _)) = parse_input()?;
    let vy_max = -y1 - 1;
    let y_max = vy_max * (vy_max + 1) / 2;
    Ok(y_max)
}

fn part2() -> Result<usize> {
    let hitbox = parse_input()?;
    let ((_, x2), (y1, _)) = hitbox;
    Ok((1..=x2)
        .flat_map(|vx| (y1..=-y1 - 1).map(move |vy| (vx, vy)))
        .filter(|&(vx, vy)| hit_area(vx, vy, hitbox))
        .count())
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        println!("answer: {}", part1().unwrap());
    }

    #[test]
    fn test_part2() {
        println!("answer: {}", part2().unwrap());
    }
}
