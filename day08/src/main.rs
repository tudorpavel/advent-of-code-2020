use std::collections::HashSet;

fn run_program(instructions: &[(&str, i32)]) -> Result<i32, i32> {
    let mut accumulator = 0;
    let mut current = 0;
    let mut visited = HashSet::new();

    while current < instructions.len() {
        if visited.contains(&current) {
            return Err(accumulator);
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

    Ok(accumulator)
}

fn swap_operations(instruction: &mut (&str, i32)) {
    match instruction {
        ("nop", _) => {
            instruction.0 = "jmp";
        }
        ("jmp", _) => {
            instruction.0 = "nop";
        }
        _ => (),
    }
}

fn solve(lines: Vec<String>) -> (i32, i32) {
    let mut instructions: Vec<(&str, i32)> = lines
        .iter()
        .map(|l| {
            let parts: Vec<&str> = l.split(' ').collect();
            let operation = parts[0];
            let argument = parts[1].parse().unwrap();

            (operation, argument)
        })
        .collect();

    let part1 = match run_program(&instructions) {
        Ok(accumulator) => accumulator,
        Err(accumulator) => accumulator,
    };
    let mut part2 = 0;

    for i in 0..instructions.len() {
        swap_operations(&mut instructions[i]);

        if let Ok(accumulator) = run_program(&instructions) {
            part2 = accumulator;
            break;
        }

        swap_operations(&mut instructions[i]);
    }

    (part1, part2)
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
                vec![
                    "nop +0", "acc +1", "jmp +4", "acc +3", "jmp -3", "acc -99", "acc +1",
                    "jmp -4", "acc +6",
                ]
                .into_iter()
                .map(|s| String::from(s))
                .collect()
            ),
            (5, 8)
        );
    }
}
