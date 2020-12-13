fn solve(lines: Vec<String>) -> usize {
    let earliest_timestamp: usize = lines[0].parse().unwrap();

    let (bus_id, min_wait) = lines[1]
        .split(',')
        .filter(|s| s != &"x")
        .map(|id| id.parse::<usize>().unwrap())
        .map(|id| (id, id - (earliest_timestamp % id)))
        .min_by_key(|pair| pair.1)
        .unwrap();

    bus_id * min_wait
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
                vec!["939", "7,13,x,x,59,x,31,19"]
                    .into_iter()
                    .map(String::from)
                    .collect()
            ),
            295
        );
    }
}
