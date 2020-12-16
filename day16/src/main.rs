use std::ops::RangeInclusive;

fn part1(lines: &[String]) -> usize {
    let groups: Vec<&[String]> = lines.split(|l| l == "").collect();
    let all_ranges: Vec<RangeInclusive<usize>> = groups[0]
        .iter()
        .map(|line| {
            let rule: Vec<&str> = line.split(": ").collect();
            let field_ranges: Vec<&str> = rule[1].split(" or ").collect();
            let field_ranges: Vec<RangeInclusive<usize>> = field_ranges
                .iter()
                .map(|s| {
                    let nums: Vec<&str> = s.split('-').collect();

                    nums[0].parse().unwrap()..=nums[1].parse().unwrap()
                })
                .collect();

            field_ranges
        })
        .flatten()
        .collect();

    let all_nearby_ticket_values: Vec<usize> = groups[2][1..groups[2].len()]
        .iter()
        .map(|line| {
            line.split(',')
                .map(|s| s.parse().unwrap())
                .collect::<Vec<usize>>()
        })
        .flatten()
        .collect();

    all_nearby_ticket_values
        .iter()
        .filter(|value| all_ranges.iter().all(|r| !r.contains(value)))
        .sum()
}

fn main() {
    let lines = utils::read_lines();
    println!("Part 1: {}", part1(&lines));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_works() {
        let lines: Vec<String> = vec![
            "class: 1-3 or 5-7",
            "row: 6-11 or 33-44",
            "seat: 13-40 or 45-50",
            "",
            "your ticket:",
            "7,1,14",
            "",
            "nearby tickets:",
            "7,3,47",
            "40,4,50",
            "55,2,20",
            "38,6,12",
        ]
        .into_iter()
        .map(String::from)
        .collect();

        assert_eq!(part1(&lines), 71);
    }
}
