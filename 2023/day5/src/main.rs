#[derive(Debug, Default)]
struct RangeMap {
    dst: usize,
    src: usize,
    len: usize,
}

fn ranges_map(ranges: &[RangeMap], n: usize) -> usize {
    for range in ranges {
        if n >= range.src && n < range.src + range.len {
            return range.dst + (n - range.src);
        }
    }

    n
}

#[derive(Debug, Default)]
struct Game {
    seeds: Vec<usize>,
    seed_to_soil: Vec<RangeMap>,
    soil_to_fertilizer: Vec<RangeMap>,
    fertilizer_to_water: Vec<RangeMap>,
    water_to_light: Vec<RangeMap>,
    light_to_temperature: Vec<RangeMap>,
    temperature_to_humidity: Vec<RangeMap>,
    humidity_to_location: Vec<RangeMap>,
}

fn parse(input: &str) -> Game {
    let mut game = Game::default();

    let entries: Vec<_> = input.split("\n\n").collect();

    game.seeds = entries[0]
        .split_once(' ')
        .map(|(_, seeds)| {
            seeds
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect()
        })
        .unwrap();

    let ranges = |entry: &str| -> Vec<RangeMap> {
        entry
            .lines()
            .skip(1)
            .map(|l| {
                let range: Vec<_> = l
                    .split_whitespace()
                    .map(|n| n.parse::<usize>().unwrap())
                    .collect();
                RangeMap {
                    dst: range[0],
                    src: range[1],
                    len: range[2],
                }
            })
            .collect()
    };

    game.seed_to_soil = ranges(entries[1]);
    game.soil_to_fertilizer = ranges(entries[2]);
    game.fertilizer_to_water = ranges(entries[3]);
    game.water_to_light = ranges(entries[4]);
    game.light_to_temperature = ranges(entries[5]);
    game.temperature_to_humidity = ranges(entries[6]);
    game.humidity_to_location = ranges(entries[7]);

    game
}

fn part1(input: &str) -> usize {
    let game = parse(input);

    let pipeline = [
        &game.seed_to_soil,
        &game.soil_to_fertilizer,
        &game.fertilizer_to_water,
        &game.water_to_light,
        &game.light_to_temperature,
        &game.temperature_to_humidity,
        &game.humidity_to_location,
    ];

    game.seeds
        .into_iter()
        .map(|seed| {
            pipeline
                .iter()
                .fold(seed, |location, ranges| ranges_map(ranges, location))
        })
        .min()
        .unwrap()
}

fn part2(input: &str) -> usize {
    let game = parse(input);

    let pipeline = [
        &game.seed_to_soil,
        &game.soil_to_fertilizer,
        &game.fertilizer_to_water,
        &game.water_to_light,
        &game.light_to_temperature,
        &game.temperature_to_humidity,
        &game.humidity_to_location,
    ];

    game.seeds
        .chunks_exact(2)
        .flat_map(|chunk| match chunk {
            &[seed_start, len] => seed_start..seed_start + len,
            _ => panic!("wtf"),
        })
        .map(|seed| {
            pipeline
                .iter()
                .fold(seed, |location, ranges| ranges_map(ranges, location))
        })
        .min()
        .unwrap()
}

fn main() {
    println!("part1={}", part1(include_str!("../input.txt")));
    println!("part2={}", part2(include_str!("../input.txt")));
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 35);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 46);
    }
}
