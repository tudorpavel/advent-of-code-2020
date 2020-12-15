use std::collections::HashMap;

const PART1: u32 = 2020;
const PART2: u32 = 30000000;

fn solve(lines: &[String], target_turn: u32) -> u32 {
    let starting_numbers: Vec<u32> = lines[0].split(',').map(|s| s.parse().unwrap()).collect();

    // HashMap<K, V>, where number K was last spoken at turn V
    let mut history: HashMap<u32, u32> = HashMap::new();
    let mut number_just_spoken = 0;

    if let Some((last, already_seen)) = starting_numbers.split_last() {
        number_just_spoken = *last;

        for (i, n) in already_seen.iter().enumerate() {
            history.insert(*n, (i + 1) as u32);
        }
    }

    let mut turn = starting_numbers.len() as u32;

    loop {
        let next_number = match history.get(&number_just_spoken) {
            Some(last_seen_at) => turn - last_seen_at,
            None => 0,
        };

        history.insert(number_just_spoken, turn);
        number_just_spoken = next_number;
        turn += 1;

        if turn == target_turn {
            break;
        }
    }

    number_just_spoken
}

fn main() {
    let lines = utils::read_lines();
    println!("Part 1: {}", solve(&lines, PART1));
    println!("Part 2: {}", solve(&lines, PART2));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_works_with_initial_example() {
        assert_eq!(solve(&vec!["0,3,6".to_string()], PART1), 436);
    }

    #[test]
    fn part1_works_with_other_examples() {
        assert_eq!(solve(&vec!["1,3,2".to_string()], PART1), 1);
        assert_eq!(solve(&vec!["2,1,3".to_string()], PART1), 10);
        assert_eq!(solve(&vec!["1,2,3".to_string()], PART1), 27);
        assert_eq!(solve(&vec!["2,3,1".to_string()], PART1), 78);
        assert_eq!(solve(&vec!["3,2,1".to_string()], PART1), 438);
        assert_eq!(solve(&vec!["3,1,2".to_string()], PART1), 1836);
    }

    #[test]
    fn part2_works_with_initial_example() {
        assert_eq!(solve(&vec!["0,3,6".to_string()], PART2), 175594);
    }
}
