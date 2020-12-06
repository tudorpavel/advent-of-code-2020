use std::collections::HashSet;

fn solve(lines: Vec<String>) -> usize {
    lines
        .split(|l| l == "")
        .map(|vec| vec.join(""))
        .map(|group| {
            group.chars().fold(HashSet::new(), |mut set, c| {
                set.insert(c);
                set
            })
        })
        .map(|set| set.len())
        .sum()
}

fn main() {
    let lines = utils::read_lines();
    let sum = solve(lines);
    println!("Part 1: {}", sum);
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
            11
        );
    }
}
