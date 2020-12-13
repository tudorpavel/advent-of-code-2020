fn part1(lines: &[String]) -> usize {
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

fn part2(lines: &[String]) -> u64 {
    // Convert "17,x,13,19" to [17, 0, 13, 19]
    let bus_ids: Vec<u64> = lines[1]
        .split(',')
        .map(|s| s.parse::<u64>().unwrap_or(0))
        .collect();

    let mut current_solution = 0;
    let mut step_size: u64 = 1;

    // Insight here is that each previously found pattern
    // repeats itself every Least common multiple (LCM) steps
    // and LCM of primes is their product
    for (offset, bus_id) in bus_ids.iter().enumerate() {
        if *bus_id == 0 {
            continue;
        }

        for timestamp in (current_solution..u64::MAX).step_by(step_size as usize) {
            if (timestamp + offset as u64) % bus_id == 0 {
                current_solution = timestamp;
                step_size *= bus_id;
                break;
            }
        }
    }

    current_solution
}

fn main() {
    let lines = utils::read_lines();
    println!("Part 1: {}", part1(&lines));
    println!("Part 2: {}", part2(&lines));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_works() {
        let input: Vec<String> = vec!["939".to_string(), "7,13,x,x,59,x,31,19".to_string()];

        assert_eq!(part1(&input), 295);
    }

    #[test]
    fn part2_works_with_initial_example() {
        let input: Vec<String> = vec!["0".to_string(), "7,13,x,x,59,x,31,19".to_string()];

        assert_eq!(part2(&input), 1068781);
    }

    #[test]
    fn part2_works_with_other_examples() {
        let input: Vec<String> = vec!["0".to_string(), "17,x,13,19".to_string()];
        assert_eq!(part2(&input), 3417);

        let input: Vec<String> = vec!["0".to_string(), "67,7,59,61".to_string()];
        assert_eq!(part2(&input), 754018);

        let input: Vec<String> = vec!["0".to_string(), "67,x,7,59,61".to_string()];
        assert_eq!(part2(&input), 779210);

        let input: Vec<String> = vec!["0".to_string(), "67,7,x,59,61".to_string()];
        assert_eq!(part2(&input), 1261476);

        let input: Vec<String> = vec!["0".to_string(), "1789,37,47,1889".to_string()];
        assert_eq!(part2(&input), 1202161486);
    }
}
