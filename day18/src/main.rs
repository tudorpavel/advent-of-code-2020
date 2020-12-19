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

fn evaluate(line: &String) -> u64 {
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

fn part1(lines: &[String]) -> u64 {
    lines.iter().map(evaluate).sum()
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

        assert_eq!(part1(&lines), 71 + 51 + 26 + 437 + 12240 + 13632);
    }
}
