use std::ops::RangeInclusive;

type Ticket = Vec<usize>;

trait TicketMethods {
    fn build(line: &String) -> Ticket;
}

impl TicketMethods for Ticket {
    fn build(line: &String) -> Ticket {
        line.split(',').map(|s| s.parse().unwrap()).collect()
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Field {
    name: String,
    range1: RangeInclusive<usize>,
    range2: RangeInclusive<usize>,
}

impl Field {
    fn build(line: &String) -> Field {
        let rule: Vec<&str> = line.split(": ").collect();
        let field_ranges: Vec<&str> = rule[1].split(" or ").collect();

        let nums: Vec<&str> = field_ranges[0].split('-').collect();
        let range1 = nums[0].parse().unwrap()..=nums[1].parse().unwrap();

        let nums: Vec<&str> = field_ranges[1].split('-').collect();
        let range2 = nums[0].parse().unwrap()..=nums[1].parse().unwrap();

        Field {
            name: rule[0].to_string(),
            range1,
            range2,
        }
    }

    fn ticket_is_valid(&self, ticket: &Ticket) -> bool {
        ticket
            .iter()
            .all(|value| self.range1.contains(value) || self.range2.contains(value))
    }

    fn tickets_valid_at_offset(&self, tickets: &[Ticket], offset: usize) -> bool {
        tickets.iter().all(|ticket| {
            self.range1.contains(&ticket[offset]) || self.range2.contains(&ticket[offset])
        })
    }
}

fn part1(lines: &[String]) -> usize {
    let groups: Vec<&[String]> = lines.split(|l| l == "").collect();
    let all_ranges: Vec<RangeInclusive<usize>> = groups[0]
        .iter()
        .map(|line| {
            let rule: Vec<&str> = line.split(": ").collect();
            let field_ranges: Vec<&str> = rule[1].split(" or ").collect();
            let field_ranges: Vec<RangeInclusive<usize>> = field_ranges
                .iter()
                .map(|s| {
                    let nums: Vec<&str> = s.split('-').collect();

                    nums[0].parse().unwrap()..=nums[1].parse().unwrap()
                })
                .collect();

            field_ranges
        })
        .flatten()
        .collect();

    let all_nearby_ticket_values: Vec<usize> = groups[2][1..groups[2].len()]
        .iter()
        .map(|line| {
            line.split(',')
                .map(|s| s.parse().unwrap())
                .collect::<Vec<usize>>()
        })
        .flatten()
        .collect();

    all_nearby_ticket_values
        .iter()
        .filter(|value| all_ranges.iter().all(|r| !r.contains(value)))
        .sum()
}

fn part2(lines: &[String]) -> usize {
    let groups: Vec<&[String]> = lines.split(|l| l == "").collect();
    let fields: Vec<Field> = groups[0].iter().map(Field::build).collect();
    let my_ticket = Ticket::build(&groups[1][1]);
    let nearby_tickets: Vec<Ticket> = groups[2][1..groups[2].len()]
        .iter()
        .map(Ticket::build)
        .filter(|ticket| fields.iter().any(|f| f.ticket_is_valid(&ticket)))
        .collect();

    let mut valid_fields_for_offsets: Vec<(usize, Vec<&Field>)> = my_ticket
        .iter()
        .enumerate()
        .map(|(offset, _)| {
            (
                offset,
                fields
                    .iter()
                    .filter(|field| field.tickets_valid_at_offset(&nearby_tickets, offset))
                    .collect(),
            )
        })
        .collect();

    valid_fields_for_offsets.sort_by(|a, b| a.1.len().cmp(&(b.1.len())));

    let mut used_fields: Vec<&Field> = vec![];
    let mut part2_result = 1;

    for (offset, valid_fields) in valid_fields_for_offsets {
        let chosen_field: &Field = valid_fields
            .iter()
            .find(|&&f| !used_fields.contains(&f))
            .unwrap();

        used_fields.push(chosen_field);

        if chosen_field.name.starts_with("departure") {
            part2_result *= my_ticket[offset];
        }
    }

    part2_result
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
            "class: 1-3 or 5-7",
            "row: 6-11 or 33-44",
            "seat: 13-40 or 45-50",
            "",
            "your ticket:",
            "7,1,14",
            "",
            "nearby tickets:",
            "7,3,47",
            "40,4,50",
            "55,2,20",
            "38,6,12",
        ]
        .into_iter()
        .map(String::from)
        .collect();

        assert_eq!(part1(&lines), 71);
    }

    #[test]
    fn part2_works() {
        let lines: Vec<String> = vec![
            "departure station: 0-1 or 4-19",
            "departure date: 0-5 or 8-19",
            "seat: 0-13 or 16-19",
            "",
            "your ticket:",
            "11,12,13",
            "",
            "nearby tickets:",
            "3,9,18",
            "15,1,5",
            "5,14,9",
            "500,14,9",
        ]
        .into_iter()
        .map(String::from)
        .collect();

        assert_eq!(part2(&lines), 132);
    }
}
