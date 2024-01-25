#[derive(Clone, Debug)]
struct Tile {
    id: u64,
    pixels: Vec<Vec<char>>,
}

impl Tile {
    fn build(lines: &[String]) -> Tile {
        Tile {
            id: lines[0][5..=8].parse().unwrap(),
            pixels: lines[1..].iter().map(|l| l.chars().collect()).collect(),
        }
    }

    // mutably rotate Tile 90 degrees clockwise
    fn rotate(&mut self) {
        let mut result = self.pixels.clone();

        for (i, row) in self.pixels.iter().enumerate() {
            for (j, &pixel) in row.iter().enumerate() {
                result[j][row.len() - i - 1] = pixel;
            }
        }

        self.pixels = result;
    }

    // mutably flip Tile horizontally
    fn flip(&mut self) {
        for row in self.pixels.iter_mut() {
            row.reverse();
        }
    }

    fn print(&self) {
        println!("Tile {}:", self.id);

        for line in self.pixels.iter() {
            let s: String = line.iter().collect();
            println!("{}", s);
        }

        println!();
    }
}

struct Candidate {
    size: usize,
    i: usize,
    j: usize,
    tiles: Vec<Vec<Tile>>,
}

impl Candidate {
    fn new(size: usize) -> Candidate {
        Candidate {
            size,
            i: 0,
            j: 0,
            tiles: vec![vec![]],
        }
    }

    fn next_position(&mut self) {
        if self.j == self.size - 1 {
            self.i += 1;
            self.j = 0;
        } else {
            self.j += 1;
        };
    }

    fn replace(&mut self, tile: Tile) {
        self.tiles[self.i][self.j] = tile;
    }

    fn is_complete(&self) -> bool {
        self.i == self.size - 1 && self.j == self.size - 1
    }

    fn reject(&self) -> bool {
        for i in 0..=self.i {
            let last = if i == self.i { self.j } else { self.size - 1 };

            for j in 0..=last {
                // compare tiles and stuff
            }
        }

        false
    }
}

fn reject(candidate: &Candidate) -> bool {
    if candidate.is_complete() {
        return false;
    }

    false
}

fn backtracking(tiles: &[Tile], candidate: &mut Candidate) -> Option<Candidate> {
    if reject(&candidate) {
        return None;
    }
    // if accept(&candidate) {
    // }

    candidate.next_position();

    for (i, tile) in tiles.iter().enumerate() {
        let mut cloned_tile = tile.clone();
        let (left, right) = tiles.split_at(i);
        let rest = &[&left[..], &right[1..]].concat();

        for j in 0..8 {
            candidate.replace(cloned_tile.clone());

            if let Some(solution) = backtracking(rest, candidate) {
                return Some(solution);
            }

            if j == 3 {
                cloned_tile.flip();
            } else {
                cloned_tile.rotate();
            }
        }
    }

    None
}

fn part1(lines: &[String], size: usize) -> u64 {
    let tiles: Vec<Tile> = lines.split(|l| l == "").map(|v| Tile::build(v)).collect();
    let mut candidate = Candidate::new(size);

    if let Some(solution) = backtracking(&tiles, &mut candidate) {
        return solution.tiles[0][0].id
            * solution.tiles[0][size - 1].id
            * solution.tiles[size - 1][0].id
            * solution.tiles[size - 1][size - 1].id;
    }

    0
}

fn main() {
    let lines = utils::read_lines();
    println!("Part 1: {}", part1(&lines, 12));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_works() {
        let lines: Vec<String> = vec![
            "Tile 2311:",
            "..##.#..#.",
            "##..#.....",
            "#...##..#.",
            "####.#...#",
            "##.##.###.",
            "##...#.###",
            ".#.#.#..##",
            "..#....#..",
            "###...#.#.",
            "..###..###",
            "",
            "Tile 1951:",
            "#.##...##.",
            "#.####...#",
            ".....#..##",
            "#...######",
            ".##.#....#",
            ".###.#####",
            "###.##.##.",
            ".###....#.",
            "..#.#..#.#",
            "#...##.#..",
            "",
            "Tile 1171:",
            "####...##.",
            "#..##.#..#",
            "##.#..#.#.",
            ".###.####.",
            "..###.####",
            ".##....##.",
            ".#...####.",
            "#.##.####.",
            "####..#...",
            ".....##...",
            "",
            "Tile 1427:",
            "###.##.#..",
            ".#..#.##..",
            ".#.##.#..#",
            "#.#.#.##.#",
            "....#...##",
            "...##..##.",
            "...#.#####",
            ".#.####.#.",
            "..#..###.#",
            "..##.#..#.",
            "",
            "Tile 1489:",
            "##.#.#....",
            "..##...#..",
            ".##..##...",
            "..#...#...",
            "#####...#.",
            "#..#.#.#.#",
            "...#.#.#..",
            "##.#...##.",
            "..##.##.##",
            "###.##.#..",
            "",
            "Tile 2473:",
            "#....####.",
            "#..#.##...",
            "#.##..#...",
            "######.#.#",
            ".#...#.#.#",
            ".#########",
            ".###.#..#.",
            "########.#",
            "##...##.#.",
            "..###.#.#.",
            "",
            "Tile 2971:",
            "..#.#....#",
            "#...###...",
            "#.#.###...",
            "##.##..#..",
            ".#####..##",
            ".#..####.#",
            "#..#.#..#.",
            "..####.###",
            "..#.#.###.",
            "...#.#.#.#",
            "",
            "Tile 2729:",
            "...#.#.#.#",
            "####.#....",
            "..#.#.....",
            "....#..#.#",
            ".##..##.#.",
            ".#.####...",
            "####.#.#..",
            "##.####...",
            "##..#.##..",
            "#.##...##.",
            "",
            "Tile 3079:",
            "#.#.#####.",
            ".#..######",
            "..#.......",
            "######....",
            "####.#..#.",
            ".#...#.##.",
            "#.#####.##",
            "..#.###...",
            "..#.......",
            "..#.###...",
        ]
        .into_iter()
        .map(String::from)
        .collect();

        assert_eq!(part1(&lines, 3), 20899048083289);
    }
}
