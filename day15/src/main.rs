const PART1: usize = 2020;
const PART2: usize = 30_000_000;

fn solve(lines: &[String], target_turn: usize) -> usize {
    let starting_numbers: Vec<usize> = lines[0].split(',').map(|s| s.parse().unwrap()).collect();

    // Vec where index is the spoken number and value is the turn when it was last spoken.
    let mut history = vec![0; target_turn];
    let mut number_just_spoken = 0;

    if let Some((last, already_seen)) = starting_numbers.split_last() {
        number_just_spoken = *last;

        for (i, n) in already_seen.iter().enumerate() {
            history[*n] = i + 1;
        }
    }

    let mut turn = starting_numbers.len();

    loop {
        let next_number = if history[number_just_spoken] != 0 {
            turn - history[number_just_spoken]
        } else {
            0
        };

        history[number_just_spoken] = turn;
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
        assert_eq!(solve(&vec!["0,3,6".to_string()], PART2), 175_594);
    }
}
