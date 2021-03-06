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

fn part1(lines: &[String]) -> u64 {
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

fn insert_at_floating_addresses(mem: &mut HashMap<u64, u64>, mask: &str, address: u64, value: u64) {
    let mut exponents = vec![];
    let mut address = address;

    for (i, c) in mask.chars().rev().enumerate() {
        if c == 'X' {
            let power = 2_u64.pow(i as u32);

            // set floating bits to 0 in address, leaving non-floating bits intact
            // so we can add each floating mask to the address below
            address &= power ^ (2_u64.pow(37) - 1_u64);
            exponents.push(power);
        }
    }

    for mut combination in 0..2_u64.pow(exponents.len() as u32) {
        // decompose each binary number in bits and place bits at desired exponent positions
        // this is similar to how you would handle extracting digits from decimal numbers
        let floating_mask = exponents.iter().fold(0, |acc, e| {
            let digit = combination % 2;
            combination /= 2;
            digit * e + acc
        });

        mem.insert(address + floating_mask, value);
    }
}

fn part2(lines: &[String]) -> u64 {
    let mut mem: HashMap<u64, u64> = HashMap::new();
    let mut mask = "";
    let mut or_mask: u64 = 0;

    for line in lines {
        let pair: Vec<&str> = line.split(" = ").collect();

        if pair[0] == "mask" {
            mask = pair[1];
            let (_, or) = prepare_bitmasks(mask);

            or_mask = or;
        } else {
            let address: u64 = pair[0].split('[').collect::<Vec<&str>>()[1]
                .split(']')
                .collect::<Vec<&str>>()[0]
                .parse()
                .unwrap();
            let value: u64 = pair[1].parse().unwrap();
            // overwrite bits in address when corresponding 1 in bitmask
            let address = address | or_mask;

            insert_at_floating_addresses(&mut mem, mask, address, value);
        }
    }

    mem.iter().map(|(_, value)| value).sum()
}

fn main() {
    let lines = utils::read_lines();
    println!("Part 1: {}", part1(&lines));
    println!("Part 2: {}", part2(&lines));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_works() {
        let lines: Vec<String> = vec![
            "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X",
            "mem[8] = 11",
            "mem[7] = 101",
            "mem[8] = 0",
        ]
        .into_iter()
        .map(String::from)
        .collect();

        assert_eq!(part1(&lines), 165);
    }

    #[test]
    fn part2_works() {
        let lines: Vec<String> = vec![
            "mask = 000000000000000000000000000000X1001X",
            "mem[42] = 100",
            "mask = 00000000000000000000000000000000X0XX",
            "mem[26] = 1",
        ]
        .into_iter()
        .map(String::from)
        .collect();

        assert_eq!(part2(&lines), 208);
    }
}
