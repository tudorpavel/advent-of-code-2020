use std::collections::HashMap;

fn prepare_bitmasks(input_mask: &str) -> (u64, u64) {
    let mut and_mask = 0;
    let mut or_mask = 0;

    for (i, bit) in input_mask.chars().rev().enumerate() {
        let power = 2_u64.pow(i as u32);

        match bit {
            'X' => and_mask += power,
            '1' => {
                and_mask += power;
                or_mask += power;
            }
            _ => (),
        }
    }

    (and_mask, or_mask)
}

fn solve(lines: Vec<String>) -> u64 {
    let mut mem: HashMap<u64, u64> = HashMap::new();
    let mut and_mask: u64 = 1;
    let mut or_mask: u64 = 0;

    for line in lines {
        let pair: Vec<&str> = line.split(" = ").collect();

        if pair[0] == "mask" {
            let (and, or) = prepare_bitmasks(pair[1]);
            and_mask = and;
            or_mask = or;
        } else {
            let address: u64 = pair[0].split('[').collect::<Vec<&str>>()[1]
                .split(']')
                .collect::<Vec<&str>>()[0]
                .parse()
                .unwrap();
            let value: u64 = pair[1].parse().unwrap();
            let value = value & and_mask;
            let value = value | or_mask;

            mem.insert(address, value);
        }
    }

    mem.iter().map(|(_, value)| value).sum()
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
                    "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X",
                    "mem[8] = 11",
                    "mem[7] = 101",
                    "mem[8] = 0"
                ]
                .into_iter()
                .map(String::from)
                .collect()
            ),
            165
        );
    }
}
