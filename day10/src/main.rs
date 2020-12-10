fn solve(lines: Vec<String>) -> u32 {
    let mut nums: Vec<u32> = lines.iter().map(|l| l.parse().unwrap()).collect();

    nums.sort();

    let (mut diff1_count, mut diff3_count) =
        nums.windows(2)
            .fold((0, 0), |(mut diff1_count, mut diff3_count), pair| {
                match pair[1] - pair[0] {
                    1 => diff1_count += 1,
                    3 => diff3_count += 1,
                    _ => (),
                }

                (diff1_count, diff3_count)
            });

    // from charging outlet to first adapter
    match nums[0] {
        1 => diff1_count += 1,
        3 => diff3_count += 1,
        _ => (),
    }

    // from last adapter to my device
    diff3_count += 1;

    diff1_count * diff3_count
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
    fn solve_works_with_small_example() {
        assert_eq!(
            solve(
                vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4]
                    .into_iter()
                    .map(|n| n.to_string())
                    .collect()
            ),
            35
        );
    }

    #[test]
    fn solve_works_with_larger_example() {
        assert_eq!(
            solve(
                vec![
                    28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32,
                    25, 35, 8, 17, 7, 9, 4, 2, 34, 10, 3
                ]
                .into_iter()
                .map(|n| n.to_string())
                .collect()
            ),
            220
        );
    }
}
