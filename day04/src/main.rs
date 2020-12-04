const REQUIRED_FIELDS: [&str; 7] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

fn parse_input(lines: &[String]) -> Vec<String> {
    lines.split(|l| l == "").map(|v| v.join(" ")).collect()
}

fn valid_passport(passport: &str) -> bool {
    REQUIRED_FIELDS.iter().all(|s| passport.contains(s))
}

fn part1(lines: &[String]) -> usize {
    let passports = parse_input(lines);

    passports.into_iter().filter(|p| valid_passport(p)).count()
}

fn main() {
    let lines = utils::read_lines();
    println!("Part 1: {}", part1(&lines));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> Vec<String> {
        vec![
            "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd",
            "byr:1937 iyr:2017 cid:147 hgt:183cm",
            "",
            "iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884",
            "hcl:#cfa07d byr:1929",
            "",
            "hcl:#ae17e1 iyr:2013",
            "eyr:2024",
            "ecl:brn pid:760753108 byr:1931",
            "hgt:179cm",
            "",
            "hcl:#cfa07d eyr:2025 pid:166559648",
            "iyr:2011 ecl:brn hgt:59in",
        ]
        .into_iter()
        .map(|s| String::from(s))
        .collect()
    }

    #[test]
    fn part1_works() {
        assert_eq!(part1(&input()), 2);
    }
}
