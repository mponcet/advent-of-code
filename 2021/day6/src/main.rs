fn parse_input() -> [u64; 9] {
    let mut population = [0; 9];
    include_str!("../input.txt")
        .split(',')
        .map(|x| x.trim().parse::<u32>().unwrap())
        .for_each(|x| population[x as usize] += 1);

    population
}

fn count_fishes(population: &mut [u64; 9], days: usize) -> u64 {
    for day in 0..days {
        population[(day + 7) % 9] += population[day % 9];
    }

    population.iter().sum()
}

fn part1() -> u64 {
    let mut population = parse_input();
    count_fishes(&mut population, 80)
}

fn part2() -> u64 {
    let mut population = parse_input();
    count_fishes(&mut population, 256)
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
