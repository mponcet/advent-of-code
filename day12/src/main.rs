use std::collections::HashMap;

struct Edge {
    start: String,
    end: String,
}

fn is_big_cave(cave: &str) -> bool {
    cave.chars().any(|c| c.is_uppercase())
}

fn is_small_cave(cave: &str) -> bool {
    cave.chars().any(|c| c.is_lowercase())
}

fn parse_input() -> Vec<Edge> {
    include_str!("../input.txt")
        .lines()
        .flat_map(|l| {
            let (s1, s2) = l.split_once('-').unwrap();
            [
                Edge {
                    start: s1.to_owned(),
                    end: s2.to_owned(),
                },
                Edge {
                    start: s2.to_owned(),
                    end: s1.to_owned(),
                },
            ]
            .into_iter()
        })
        .collect::<Vec<Edge>>()
}

fn dfs(edges: &[Edge], start: &str, visited: &mut HashMap<String, u32>, max_visits: u32) -> u32 {
    edges
        .iter()
        .filter(|edge| edge.start == start)
        .map(|edge| match &edge.end[..] {
            "start" => 0,
            "end" => 1,
            _ => {
                let mut sum = 0;
                if is_big_cave(&edge.end) || *visited.get(&edge.end).unwrap_or(&0) < max_visits {
                    let counter = visited.entry(edge.end.to_owned()).or_insert(0);
                    *counter += 1;
                    if is_small_cave(&edge.end) && *counter == max_visits {
                        sum = dfs(edges, &edge.end, visited, 1);
                    } else {
                        sum = dfs(edges, &edge.end, visited, max_visits);
                    }
                    *visited.get_mut(&edge.end).unwrap() -= 1;
                }
                sum
            }
        })
        .sum()
}

fn part1() -> u32 {
    let edges = parse_input();
    dfs(&edges, "start", &mut HashMap::new(), 1)
}

fn part2() -> u32 {
    let edges = parse_input();
    dfs(&edges, "start", &mut HashMap::new(), 2)
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
