use std::io::{self, BufRead};

fn lines_to_nums(lines: Vec<String>) -> Vec<i32> {
    lines.iter().map(|s| s.parse().unwrap()).collect()
}

pub fn read_lines() -> Vec<String> {
    let mut lines: Vec<String> = Vec::new();

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        lines.push(line.unwrap());
    }

    lines
}

pub fn read_nums() -> Vec<i32> {
    lines_to_nums(read_lines())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lines_to_nums_works() {
        let lines: Vec<String> = vec!["1721".to_string(), "1456".to_string()];

        assert_eq!(lines_to_nums(lines), vec![1721, 1456]);
    }
}
