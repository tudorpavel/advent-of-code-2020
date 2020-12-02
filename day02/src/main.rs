use regex::Regex;

fn parse_input(lines: Vec<String>) -> Vec<(usize, usize, char, String)> {
    let re = Regex::new(r"(\d+)-(\d+) (\D): (\D+)").unwrap();

    lines
        .iter()
        .map(|l| {
            let cap = re.captures(l).unwrap();

            (
                cap[1].parse().unwrap(),
                cap[2].parse().unwrap(),
                cap[3].parse().unwrap(),
                cap[4].to_string(),
            )
        })
        .collect()
}

fn part1(lines: Vec<String>) -> usize {
    let tuples = parse_input(lines);

    tuples
        .iter()
        .filter(|(min, max, letter, pass)| {
            let occurrence_count = pass.chars().filter(|c| c == letter).count();

            min <= &occurrence_count && &occurrence_count <= max
        })
        .count()
}

fn part2(lines: Vec<String>) -> usize {
    let tuples = parse_input(lines);

    tuples
        .iter()
        .filter(|(pos1, pos2, letter, pass)| {
            let match1 = &pass.chars().nth(pos1 - 1).unwrap() == letter;
            let match2 = &pass.chars().nth(pos2 - 1).unwrap() == letter;

            match1 && !match2 || !match1 && match2
        })
        .count()
}

fn main() {
    let lines = utils::read_lines();
    println!("Part 1: {}", part1(lines.clone()));
    println!("Part 1: {}", part2(lines));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_works() {
        assert_eq!(
            part1(vec![
                "1-3 a: abcde".to_string(),
                "1-3 b: cdefg".to_string(),
                "2-9 c: ccccccccc".to_string()
            ]),
            2
        );
    }

    #[test]
    fn part2_works() {
        assert_eq!(
            part2(vec![
                "1-3 a: abcde".to_string(),
                "1-3 b: cdefg".to_string(),
                "2-9 c: ccccccccc".to_string()
            ]),
            1
        );
    }
}
