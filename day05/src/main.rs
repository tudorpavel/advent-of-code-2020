use std::collections::HashSet;
use std::ops::Range;

fn binary_search(path: &str, search_range: Range<usize>, left_char: char) -> usize {
    *path
        .chars()
        .fold(search_range.collect::<Vec<_>>(), |range, c| {
            let (left, right) = range.split_at(range.len() / 2);

            if c == left_char {
                left.to_vec()
            } else {
                right.to_vec()
            }
        })
        .first()
        .unwrap()
}

fn solve(lines: Vec<String>) -> (usize, usize) {
    let ids: Vec<usize> = lines
        .iter()
        .map(|l| {
            let (row_path, col_path) = l.split_at(7);
            let row = binary_search(row_path, 0..128, 'F');
            let col = binary_search(col_path, 0..8, 'L');

            row * 8 + col
        })
        .collect();

    let min = *ids.iter().min().unwrap();
    let max = *ids.iter().max().unwrap();

    let all_seats: HashSet<_> = (min..max + 1).collect();
    let tickets: HashSet<_> = ids.iter().cloned().collect();
    let my_id = **all_seats
        .difference(&tickets)
        .collect::<Vec<_>>()
        .first()
        .unwrap();

    (max, my_id)
}

fn main() {
    let lines = utils::read_lines();
    let (max, my_id) = solve(lines);
    println!("Part 1: {}", max);
    println!("Part 2: {}", my_id);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_works() {
        assert_eq!(
            solve(
                vec!["BBFFBBFLLR", "BBFFBBFLRL", "BBFFBBFRLL", "BBFFBBFRLR"]
                    .into_iter()
                    .map(|s| String::from(s))
                    .collect()
            ),
            (821, 819)
        );
    }
}
