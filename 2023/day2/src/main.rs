#[derive(Debug, Default)]
struct GameSet {
    red: usize,
    green: usize,
    blue: usize,
}

type Game = Vec<GameSet>;

fn parse(input: &str) -> Vec<Game> {
    input
        .lines()
        .map(|l| {
            l[(l.find(':').unwrap() + 2)..]
                .split("; ")
                .map(|set| {
                    // x red, y green, z blue
                    set.split(", ")
                        .fold(GameSet::default(), |mut gameset, rgb| {
                            match rgb.split_once(' ').unwrap() {
                                (count, "red") => gameset.red = count.parse().unwrap(),
                                (count, "green") => gameset.green = count.parse().unwrap(),
                                (count, "blue") => gameset.blue = count.parse().unwrap(),
                                _ => panic!("wtf"),
                            }
                            gameset
                        })
                })
                .collect()
        })
        .collect()
}

fn part1(input: &str) -> usize {
    let games = parse(input);
    games
        .into_iter()
        .enumerate()
        .filter(|(_, game)| {
            game.iter()
                .all(|set| set.red <= 12 && set.green <= 13 && set.blue <= 14)
        })
        .map(|(idx, _)| idx + 1)
        .sum()
}

fn part2(input: &str) -> usize {
    let games = parse(input);

    games
        .into_iter()
        .map(|game| {
            let min = game.iter().fold(GameSet::default(), |mut min, gameset| {
                min.red = min.red.max(gameset.red);
                min.green = min.green.max(gameset.green);
                min.blue = min.blue.max(gameset.blue);
                min
            });
            min.red * min.green * min.blue
        })
        .sum()
}

fn main() {
    println!("part1={}", part1(include_str!("../input.txt")));
    println!("part2={}", part2(include_str!("../input.txt")));
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 8);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 2286);
    }
}
