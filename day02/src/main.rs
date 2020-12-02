use regex::Regex;

fn part1(lines: Vec<String>) -> i32 {
    let mut valid_count = 0;
    let re = Regex::new(r"(\d+)-(\d+) (\D): (\D+)").unwrap();

    for line in lines {
        for cap in re.captures_iter(&line) {
            let min: usize = cap[1].parse().unwrap();
            let max: usize = cap[2].parse().unwrap();
            let letter: char = cap[3].parse().unwrap();
            let pass: String = cap[4].to_string();

            let occurrence_count = pass.chars().filter(|c| c == &letter).count();

            if min <= occurrence_count && occurrence_count <= max {
                valid_count += 1;
            }
        }
    }

    valid_count
}

fn main() {
    let lines = utils::read_lines();
    println!("Part 1: {}", part1(lines));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_works() {
        assert_eq!(
            part1(vec!["1-3 a: abcde".to_string(),
                       "1-3 b: cdefg".to_string(),
                       "2-9 c: ccccccccc".to_string()]),
            2
        );
    }
}
