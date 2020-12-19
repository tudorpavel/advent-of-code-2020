mod part1 {
    // gradually performs operations by popping values from the stack and stops at first ')'
    // For example:
    // `1 + 2 * 3` becomes `9`
    // `1 + 2 * 3) + 4` becomes `9 + 4`
    fn reduce(stack: &mut Vec<String>) {
        let mut result: u64 = stack.pop().unwrap().parse().unwrap();

        while let Some(operator) = stack.pop() {
            match operator.as_str() {
                ")" => break,
                _ => {
                    if let Some(operand) = stack.pop() {
                        let operand: u64 = operand.parse().unwrap();

                        if operator == "*" {
                            result *= operand;
                        } else {
                            result += operand;
                        }
                    }
                }
            }
        }

        stack.push(result.to_string());
    }

    // iterate expression from right to left push values onto a stack until reaching
    // a '(' at which point delegate to reduce procedure to compute expression for these parens
    pub fn evaluate(line: &String) -> u64 {
        let mut stack: Vec<String> = vec![];

        for c in line.chars().rev() {
            let s = c.to_string();

            match s.as_str() {
                "(" => reduce(&mut stack),
                " " => continue,
                _ => stack.push(s),
            }
        }

        reduce(&mut stack);

        stack.pop().unwrap().parse().unwrap()
    }
}

mod part2 {
    // gradually performs addition by popping values from the stack and stops at
    // first ')' or '*' and puts it back together with the result
    // For example:
    // `1 + 2 * 3` becomes `3 * 3`
    // `1 + 2) + 5` becomes `3) + 5`
    fn reduce_addition(stack: &mut Vec<String>) {
        let mut result: u64 = stack.pop().unwrap().parse().unwrap();

        while let Some(operator) = stack.pop() {
            match operator.as_str() {
                ")" | "*" => {
                    stack.push(operator);
                    break;
                }
                _ => {
                    if let Some(operand) = stack.pop() {
                        let operand: u64 = operand.parse().unwrap();

                        result += operand;
                    }
                }
            }
        }

        stack.push(result.to_string());
    }

    // gradually performs multiplication until the first ')'
    // For example:
    // `1 * 2 * 3` becomes `6`
    // `1 * 2 * 3) + 4` becomes `6 + 4`
    fn reduce_parens(stack: &mut Vec<String>) {
        let mut result: u64 = stack.pop().unwrap().parse().unwrap();

        while let Some(operator) = stack.pop() {
            match operator.as_str() {
                ")" => break,
                _ => {
                    if let Some(operand) = stack.pop() {
                        let operand: u64 = operand.parse().unwrap();

                        result *= operand;
                    }
                }
            }
        }

        stack.push(result.to_string());
    }

    // iterate expression from right to left push values onto a stack until reaching either:
    //   - a '*' at which point it delegates to reduce_addition to compute any additions it finds
    //   - a '(' at which point it delegates first to reduce_addition and then to reduce_parens
    pub fn evaluate(line: &String) -> u64 {
        let mut stack: Vec<String> = vec![];

        for c in line.chars().rev() {
            let s = c.to_string();

            match s.as_str() {
                "*" => {
                    reduce_addition(&mut stack);
                    stack.push(s);
                }
                "(" => {
                    reduce_addition(&mut stack);
                    reduce_parens(&mut stack);
                }
                " " => continue,
                _ => stack.push(s),
            }
        }

        reduce_addition(&mut stack);
        reduce_parens(&mut stack);

        stack.pop().unwrap().parse().unwrap()
    }
}

fn solve(lines: &[String]) -> (u64, u64) {
    (
        lines.iter().map(crate::part1::evaluate).sum(),
        lines.iter().map(crate::part2::evaluate).sum(),
    )
}

fn main() {
    let lines = utils::read_lines();
    let (part1, part2) = solve(&lines);
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_works() {
        let lines: Vec<String> = vec![
            "1 + 2 * 3 + 4 * 5 + 6",
            "1 + (2 * 3) + (4 * (5 + 6))",
            "2 * 3 + (4 * 5)",
            "5 + (8 * 3 + 9 + 3 * 4 * 3)",
            "5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))",
            "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2",
        ]
        .into_iter()
        .map(String::from)
        .collect();

        assert_eq!(
            solve(&lines),
            (
                71 + 51 + 26 + 437 + 12240 + 13632,
                231 + 51 + 46 + 1445 + 669060 + 23340
            )
        );
    }
}
