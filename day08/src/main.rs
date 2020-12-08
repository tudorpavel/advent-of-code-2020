use std::collections::HashSet;

fn solve(lines: Vec<String>) -> i32 {
    let instructions: Vec<(&str, i32)> = lines
        .iter()
        .map(|l| {
            let parts: Vec<&str> = l.split(" ").collect();
            let operation = parts[0];
            let argument = parts[1].parse().unwrap();

            (operation, argument)
        })
        .collect();
    let mut accumulator = 0;
    let mut current = 0;
    let mut visited = HashSet::new();

    while current < instructions.len() {
        if visited.contains(&current) {
            break;
        }

        visited.insert(current);

        match instructions[current] {
            ("nop", _) => current += 1,
            ("acc", argument) => {
                accumulator += argument;
                current += 1;
            }
            ("jmp", argument) => current = (current as i32 + argument) as usize,
            (operation, _) => panic!("Unknown operation: {}", operation),
        }
    }

    accumulator
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
                vec![
                    "nop +0", "acc +1", "jmp +4", "acc +3", "jmp -3", "acc -99", "acc +1",
                    "jmp -4", "acc +6",
                ]
                .into_iter()
                .map(|s| String::from(s))
                .collect()
            ),
            5
        );
    }
}
