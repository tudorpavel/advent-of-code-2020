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
    fn build(lines: Vec<String>) -> GeoMap {
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

    fn tree_count(&self, slope: Point) -> usize {
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

fn part1(lines: Vec<String>) -> usize {
    let geo_map = GeoMap::build(lines);
    geo_map.tree_count(Point { x: 3, y: 1 })
}

fn main() {
    let lines = utils::read_lines();
    println!("Part 1: {}", part1(lines));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_works() {
        assert_eq!(
            part1(
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
            ),
            7
        );
    }
}
