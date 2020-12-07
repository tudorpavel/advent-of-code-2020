use regex::Regex;
use std::collections::HashMap;

fn build_dag(lines: Vec<String>) -> (HashMap<String, Vec<String>>, HashMap<(String, String), u32>) {
    let mut dag = HashMap::new();
    let mut edge_costs = HashMap::new();
    let re_vertex = Regex::new(r"(.+) bags").unwrap();
    let re_neighbor = Regex::new(r"(\d+) (.+) (bag|bags)").unwrap();

    for line in lines {
        let pair: Vec<&str> = line.split(" contain ").collect();
        let cap_vertex = re_vertex.captures(pair[0]).unwrap();
        let vertex = cap_vertex[1].to_string();
        dag.insert(vertex.clone(), vec![]);

        if pair[1] == "no other bags." {
            continue;
        }

        let adj_list = dag.get_mut(&vertex).unwrap();

        for neighbor in pair[1].split(", ") {
            let cap = re_neighbor.captures(neighbor).unwrap();
            let count = cap[1].parse().unwrap();
            let name = cap[2].to_string();

            adj_list.push(name.clone());
            edge_costs.insert((vertex.clone(), name.clone()), count);
        }
    }

    (dag, edge_costs)
}

fn path_contains(dag: &HashMap<String, Vec<String>>, current: String, lookup: &str) -> bool {
    if current == *lookup {
        return true;
    }

    let adj_list = dag.get(&current).unwrap();

    if adj_list.is_empty() {
        return false;
    }

    adj_list
        .iter()
        .any(|vertex| path_contains(dag, vertex.clone(), lookup))
}

fn total_bag_count(
    dag: &HashMap<String, Vec<String>>,
    edge_costs: &HashMap<(String, String), u32>,
    current: String,
) -> u32 {
    let adj_list = dag.get(&current).unwrap();

    if adj_list.is_empty() {
        return 1;
    }

    adj_list
        .iter()
        .map(|vertex| {
            let current_cost = edge_costs.get(&(current.clone(), vertex.clone())).unwrap();

            current_cost * total_bag_count(dag, edge_costs, vertex.clone())
        })
        .sum::<u32>()
        + 1
}

fn solve(lines: Vec<String>) -> (usize, u32) {
    let (bag_dag, edge_costs) = build_dag(lines);

    let part1 = bag_dag
        .iter()
        .filter(|(vertex, _)| {
            *vertex != "shiny gold"
                && path_contains(&bag_dag, vertex.to_string(), &"shiny gold".to_string())
        })
        .count();

    let part2 = total_bag_count(&bag_dag, &edge_costs, "shiny gold".to_string()) - 1;

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
                    "light red bags contain 1 bright white bag, 2 muted yellow bags.",
                    "dark orange bags contain 3 bright white bags, 4 muted yellow bags.",
                    "bright white bags contain 1 shiny gold bag.",
                    "muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.",
                    "shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.",
                    "dark olive bags contain 3 faded blue bags, 4 dotted black bags.",
                    "vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.",
                    "faded blue bags contain no other bags.",
                    "dotted black bags contain no other bags.",
                ]
                .into_iter()
                .map(|s| String::from(s))
                .collect()
            ),
            (4, 32)
        );
    }
}
