fn has_valid_sum(prev_nums: &[u64], reference: u64) -> bool {
    for (i, a) in prev_nums[..prev_nums.len()].iter().enumerate() {
        for b in prev_nums[i + 1..].iter() {
            if a + b == reference {
                return true;
            }
        }
    }

    false
}

fn solve(lines: Vec<String>, preamble_size: usize) -> (u64, u64) {
    let nums: Vec<u64> = lines.iter().map(|l| l.parse().unwrap()).collect();
    let mut part1 = 0;

    // Part 1
    for window in nums.windows(preamble_size + 1) {
        part1 = *window.last().unwrap();

        if !has_valid_sum(&window[..preamble_size], part1) {
            break;
        }
    }

    // Part 2
    for window_size in 2..=lines.len() {
        for window in nums.windows(window_size) {
            if window.iter().sum::<u64>() == part1 {
                return (
                    part1,
                    window.iter().min().unwrap() + window.iter().max().unwrap(),
                );
            }
        }
    }

    (part1, 0)
}

fn main() {
    let lines = utils::read_lines();
    let (part1, part2) = solve(lines, 25);
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
                    "35", "20", "15", "25", "47", "40", "62", "55", "65", "95", "102", "117",
                    "150", "182", "127", "219", "299", "277", "309", "576"
                ]
                .into_iter()
                .map(String::from)
                .collect(),
                5
            ),
            (127, 62)
        );
    }
}
