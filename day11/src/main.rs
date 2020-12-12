fn adjacent_occupancy_count(layout: &Vec<Vec<char>>, i: usize, j: usize) -> usize {
    let i = i as i32;
    let j = j as i32;

    vec![
        (i - 1, j - 1),
        (i - 1, j),
        (i - 1, j + 1),
        (i, j - 1),
        (i, j + 1),
        (i + 1, j - 1),
        (i + 1, j),
        (i + 1, j + 1),
    ]
    .iter()
    .filter(|(k, l)| {
        (0..layout.len() as i32).contains(k) && (0..layout[0].len() as i32).contains(l)
    })
    .map(|(k, l)| layout[*k as usize][*l as usize])
    .filter(|c| *c == '#')
    .count()
}

fn total_occupancy_count(layout: &Vec<Vec<char>>) -> usize {
    layout.iter().flatten().filter(|c| **c == '#').count()
}

fn simulation_next(layout: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut result = layout.to_owned();

    for i in 0..layout.len() {
        for j in 0..layout[0].len() {
            if layout[i][j] == 'L' && adjacent_occupancy_count(&layout, i, j) == 0 {
                result[i][j] = '#';
            } else if layout[i][j] == '#' && adjacent_occupancy_count(&layout, i, j) >= 4 {
                result[i][j] = 'L';
            }
        }
    }

    result
}

fn solve(lines: Vec<String>) -> usize {
    let mut layout: Vec<Vec<char>> = lines.iter().map(|l| l.chars().collect()).collect();

    loop {
        let new_layout = simulation_next(&layout);

        if layout == new_layout {
            break;
        }

        layout = new_layout;
    }

    total_occupancy_count(&layout)
}

fn main() {
    let lines = utils::read_lines();
    let part1 = solve(lines);
    println!("Part 1: {}", part1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_works() {
        assert_eq!(
            solve(
                vec![
                    "L.LL.LL.LL",
                    "LLLLLLL.LL",
                    "L.L.L..L..",
                    "LLLL.LL.LL",
                    "L.LL.LL.LL",
                    "L.LLLLL.LL",
                    "..L.L.....",
                    "LLLLLLLLLL",
                    "L.LLLLLL.L",
                    "L.LLLLL.LL",
                ]
                .into_iter()
                .map(String::from)
                .collect()
            ),
            37
        );
    }
}
