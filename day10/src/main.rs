fn part1(nums: &Vec<u64>) -> u64 {
    let mut diff1_count = 0;
    let mut diff3_count = 0;

    for window in nums.windows(2) {
        match window[1] - window[0] {
            1 => diff1_count += 1,
            3 => diff3_count += 1,
            _ => (),
        }
    }

    diff1_count * diff3_count
}

fn part2(nums: &Vec<u64>) -> u64 {
    let mut slices = vec![];
    let mut current_slice = vec![];

    // generate slices of consecutive 1-diff elements, for example:
    // input:  [0, 1, 4, 5, 6, 9]
    // result: [[0, 1], [4, 5, 6], [9]]
    for window in nums.windows(2) {
        match window[1] - window[0] {
            1 => current_slice.push(window[0]),
            3 => {
                current_slice.push(window[0]);
                slices.push(current_slice);
                current_slice = vec![];
            }
            _ => (),
        }
    }

    slices
        .iter()
        .map(|slice| match slice.len() {
            1 => 1,
            2 => 1,
            3 => 2,
            4 => 4,
            5 => 7,
            _ => panic!("unexpected slice of size N > 5 consecutive 1-diff elements"),
        })
        .product()
}

fn solve(lines: Vec<String>) -> (u64, u64) {
    let mut nums: Vec<u64> = lines.iter().map(|l| l.parse().unwrap()).collect();

    nums.sort();

    // add charging outlet before first adapter
    nums.insert(0, 0);

    // add my device after last adapter
    nums.push(nums.last().unwrap() + 3);

    (part1(&nums), part2(&nums))
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
    fn solve_works_with_small_example() {
        assert_eq!(
            solve(
                vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4]
                    .into_iter()
                    .map(|n| n.to_string())
                    .collect()
            ),
            (35, 8)
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
            (220, 19208)
        );
    }
}
