type Layout = Vec<Vec<char>>;

fn adjacent_occupancy_count(layout: &Layout, i: usize, j: usize) -> usize {
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

fn find_occupied(layout: &Layout, i: i32, j: i32, (di, dj): (i32, i32)) -> usize {
    let i = i + di;
    let j = j + dj;

    // out of bounds check
    if !(0..layout.len() as i32).contains(&i) || !(0..layout[0].len() as i32).contains(&j) {
        return 0;
    }

    // stop recursion at first seat
    match layout[i as usize][j as usize] {
        'L' => {
            return 0;
        }
        '#' => {
            return 1;
        }
        _ => (),
    }

    find_occupied(layout, i, j, (di, dj))
}

fn visible_occupancy_count(layout: &Layout, i: usize, j: usize) -> usize {
    let i = i as i32;
    let j = j as i32;

    vec![
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ]
    .iter()
    .map(|dir| find_occupied(layout, i, j, *dir))
    .sum()
}

fn total_occupancy_count(layout: &Layout) -> usize {
    layout.iter().flatten().filter(|c| **c == '#').count()
}

fn part1_simulation_next(layout: &Layout) -> Layout {
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

fn part2_simulation_next(layout: &Layout) -> Layout {
    let mut result = layout.to_owned();

    for i in 0..layout.len() {
        for j in 0..layout[0].len() {
            if layout[i][j] == 'L' && visible_occupancy_count(&layout, i, j) == 0 {
                result[i][j] = '#';
            } else if layout[i][j] == '#' && visible_occupancy_count(&layout, i, j) >= 5 {
                result[i][j] = 'L';
            }
        }
    }

    result
}

fn run_simulation(lines: &[String], simulation_next: &dyn Fn(&Layout) -> Layout) -> usize {
    let mut layout: Layout = lines.iter().map(|l| l.chars().collect()).collect();

    loop {
        let new_layout = simulation_next(&layout);

        if layout == new_layout {
            break;
        }

        layout = new_layout;
    }

    total_occupancy_count(&layout)
}

fn solve(lines: Vec<String>) -> (usize, usize) {
    (
        run_simulation(&lines, &part1_simulation_next),
        run_simulation(&lines, &part2_simulation_next),
    )
}

fn main() {
    let lines = utils::read_lines();
    let (part1, part2) = solve(lines);
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
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
            (37, 26)
        );
    }
}
