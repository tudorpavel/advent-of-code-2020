fn convert_answers(answers: &str) -> u32 {
    answers
        .chars()
        .fold(0, |acc, c| acc + 2_u32.pow(c as u32 - b'a' as u32))
}

fn count_1_bits(mut num: u32) -> u32 {
    let mut count = 0;

    while num != 0 {
        count += num & 1;
        num >>= 1;
    }

    count
}

fn solve(lines: Vec<String>) -> (u32, u32) {
    lines
        .split(|l| l == "")
        .map(|group| {
            let bitstrings = group.iter().map(|s| convert_answers(&s));

            (
                bitstrings.clone().fold(0, |acc, bs| acc | bs),
                bitstrings.fold(2_u32.pow(26) - 1, |acc, bs| acc & bs),
            )
        })
        .map(|(bs1, bs2)| (count_1_bits(bs1), count_1_bits(bs2)))
        .fold((0, 0), |(acc1, acc2), (res1, res2)| {
            (acc1 + res1, acc2 + res2)
        })
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
                vec!["abc", "", "a", "b", "c", "", "ab", "ac", "", "a", "a", "a", "a", "", "b",]
                    .into_iter()
                    .map(|s| String::from(s))
                    .collect()
            ),
            (11, 6)
        );
    }
}
