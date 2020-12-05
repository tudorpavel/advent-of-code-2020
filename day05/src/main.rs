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

fn solve(lines: Vec<String>) -> usize {
    lines
        .iter()
        .map(|l| {
            let (row_path, col_path) = l.split_at(7);
            let row = binary_search(row_path, 0..128, 'F');
            let col = binary_search(col_path, 0..8, 'L');

            row * 8 + col
        })
        .max()
        .unwrap()
}

fn main() {
    let lines = utils::read_lines();
    println!("Part 1: {}", solve(lines));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_works() {
        assert_eq!(
            solve(
                vec!["FBFBBFFRLR", "BFFFBBFRRR", "FFFBBBFRRR", "BBFFBBFRLL"]
                    .into_iter()
                    .map(|s| String::from(s))
                    .collect()
            ),
            820
        );
    }
}
