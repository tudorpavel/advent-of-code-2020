use std::collections::HashSet;

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct GeoMap {
    width: usize,
    height: usize,
    trees: HashSet<Point>,
}

impl GeoMap {
    fn build(lines: &[String]) -> GeoMap {
        let mut trees = HashSet::new();

        for (y, line) in lines.iter().enumerate() {
            for (x, c) in line.chars().enumerate() {
                if c == '#' {
                    trees.insert(Point { x, y });
                }
            }
        }

        GeoMap {
            width: lines.first().unwrap_or(&String::from("")).len(),
            height: lines.len(),
            trees,
        }
    }

    fn tree_count(&self, slope: &Point) -> usize {
        let mut current = slope.clone();
        let mut count = 0;

        while current.y < self.height {
            if self.trees.contains(&current) {
                count += 1;
            }

            current.x = (current.x + slope.x) % self.width;
            current.y += slope.y;
        }

        count
    }
}

fn part1(lines: &[String]) -> usize {
    let geo_map = GeoMap::build(lines);

    geo_map.tree_count(&Point { x: 3, y: 1 })
}

fn part2(lines: &[String]) -> usize {
    let geo_map = GeoMap::build(lines);

    vec![
        Point { x: 1, y: 1 },
        Point { x: 3, y: 1 },
        Point { x: 5, y: 1 },
        Point { x: 7, y: 1 },
        Point { x: 1, y: 2 },
    ]
    .iter()
    .map(|p| geo_map.tree_count(p))
    .product()
}

fn main() {
    let lines = utils::read_lines();
    println!("Part 1: {}", part1(&lines));
    println!("Part 1: {}", part2(&lines));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> Vec<String> {
        vec![
            "..##.......",
            "#...#...#..",
            ".#....#..#.",
            "..#.#...#.#",
            ".#...##..#.",
            "..#.##.....",
            ".#.#.#....#",
            ".#........#",
            "#.##...#...",
            "#...##....#",
            ".#..#...#.#",
        ]
        .into_iter()
        .map(|s| String::from(s))
        .collect()
    }

    #[test]
    fn part1_works() {
        assert_eq!(part1(&input()), 7);
    }

    #[test]
    fn part2_works() {
        assert_eq!(part2(&input()), 336);
    }
}
