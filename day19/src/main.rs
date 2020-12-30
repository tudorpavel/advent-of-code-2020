use std::collections::HashSet;
use std::rc::Rc;

#[derive(Clone)]
enum Tree {
    Node(Rc<Tree>, Rc<Tree>),
    Solution,
    Nil,
}

use Tree::{Nil, Node, Solution};

impl Tree {
    fn build(rules: &[&str], rule_id: usize) -> Tree {
        match rules[rule_id] {
            "\"a\"" => Node(Rc::new(Solution), Rc::new(Nil)),
            "\"b\"" => Node(Rc::new(Nil), Rc::new(Solution)),
            composed_rule => composed_rule
                .split(" | ")
                .map(|append_rules| {
                    append_rules.split(' ').fold(Solution, |acc, id| {
                        acc.append_solution(&Tree::build(&rules, id.parse().unwrap()))
                    })
                })
                .fold(Nil, |acc, tree| acc.merge(&tree)),
        }
    }

    fn is_valid(&self, message: &str) -> bool {
        match (self, message) {
            (Solution, "") => true,
            (_, "") => false,
            (Node(left, right), _) => {
                if &message[..1] == "a" {
                    left.is_valid(&message[1..])
                } else {
                    right.is_valid(&message[1..])
                }
            }
            _ => false,
        }
    }

    fn solution_depths(&self, depth: usize) -> HashSet<usize> {
        match self {
            Solution => {
                let mut result = HashSet::new();
                result.insert(depth);
                result
            }
            Node(left, right) => {
                let union: HashSet<_> = left
                    .solution_depths(depth + 1)
                    .union(&right.solution_depths(depth + 1))
                    .copied()
                    .collect();

                union
            }
            _ => HashSet::new(),
        }
    }

    fn append_solution(&self, other: &Tree) -> Tree {
        match self {
            Solution => other.clone(),
            Node(left, right) => Node(
                Rc::new(left.append_solution(&other)),
                Rc::new(right.append_solution(&other)),
            ),
            Nil => Nil,
        }
    }

    fn merge(&self, other: &Tree) -> Tree {
        match (self, other) {
            (Nil, _) => other.clone(),
            (_, Nil) => self.clone(),
            (Node(l1, r1), Node(l2, r2)) => Node(Rc::new(l1.merge(l2)), Rc::new(r1.merge(r2))),
            _ => Nil,
        }
    }
}

fn parse_input(lines: &[String]) -> (Vec<&str>, &[String]) {
    let groups: Vec<&[String]> = lines.split(|l| l == "").collect();
    let rules = groups[0];
    let rules: Vec<&str> = rules.iter().fold(vec![""; 135], |mut acc, rule| {
        let parts: Vec<&str> = rule.split(": ").collect();
        let rule_id: usize = parts[0].parse().unwrap();

        acc[rule_id] = parts[1];

        acc
    });

    (rules, groups[1])
}

fn part1(lines: &[String]) -> usize {
    let (rules, messages) = parse_input(lines);

    let rule_tree = Tree::build(&rules, 0);

    messages
        .iter()
        .filter(|message| rule_tree.is_valid(message))
        .count()
}

fn part2(lines: &[String]) -> usize {
    let (rules, messages) = parse_input(lines);

    let tree42 = Tree::build(&rules, 42);
    let tree31 = Tree::build(&rules, 31);
    let depth = *tree42.solution_depths(0).iter().next().unwrap();

    messages
        .iter()
        .filter(|message| {
            let chunks: Vec<String> = message
                .as_bytes()
                .chunks(depth)
                .map(|chunk| String::from_utf8(chunk.to_vec()).unwrap())
                .collect();

            (1..=((chunks.len() - 1) / 2))
                .map(|count31| {
                    let mut combination = vec![&tree42; chunks.len() - count31];
                    combination.append(&mut vec![&tree31; count31]);
                    combination
                })
                .any(|trees| {
                    trees
                        .iter()
                        .zip(chunks.iter())
                        .all(|(tree, chunk)| tree.is_valid(chunk))
                })
        })
        .count()
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
    fn part1_works_with_small_example() {
        let lines: Vec<String> = vec![
            "0: 4 1 5",
            "1: 2 3 | 3 2",
            "2: 4 4 | 5 5",
            "3: 4 5 | 5 4",
            "4: \"a\"",
            "5: \"b\"",
            "",
            "ababbb",
            "bababa",
            "abbbab",
            "aaabbb",
            "aaaabbb",
        ]
        .into_iter()
        .map(String::from)
        .collect();

        assert_eq!(part1(&lines), 2);
    }

    fn large_example() -> Vec<String> {
        vec![
            "42: 9 14 | 10 1",
            "9: 14 27 | 1 26",
            "10: 23 14 | 28 1",
            "1: \"a\"",
            "11: 42 31",
            "5: 1 14 | 15 1",
            "19: 14 1 | 14 14",
            "12: 24 14 | 19 1",
            "16: 15 1 | 14 14",
            "31: 14 17 | 1 13",
            "6: 14 14 | 1 14",
            "2: 1 24 | 14 4",
            "0: 8 11",
            "13: 14 3 | 1 12",
            "15: 1 | 14",
            "17: 14 2 | 1 7",
            "23: 25 1 | 22 14",
            "28: 16 1",
            "4: 1 1",
            "20: 14 14 | 1 15",
            "3: 5 14 | 16 1",
            "27: 1 6 | 14 18",
            "14: \"b\"",
            "21: 14 1 | 1 14",
            "25: 1 1 | 1 14",
            "22: 14 14",
            "8: 42",
            "26: 14 22 | 1 20",
            "18: 15 15",
            "7: 14 5 | 1 21",
            "24: 14 1",
            "",
            "abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa",
            "bbabbbbaabaabba",
            "babbbbaabbbbbabbbbbbaabaaabaaa",
            "aaabbbbbbaaaabaababaabababbabaaabbababababaaa",
            "bbbbbbbaaaabbbbaaabbabaaa",
            "bbbababbbbaaaaaaaabbababaaababaabab",
            "ababaaaaaabaaab",
            "ababaaaaabbbaba",
            "baabbaaaabbaaaababbaababb",
            "abbbbabbbbaaaababbbbbbaaaababb",
            "aaaaabbaabaaaaababaa",
            "aaaabbaaaabbaaa",
            "aaaabbaabbaaaaaaabbbabbbaaabbaabaaa",
            "babaaabbbaaabaababbaabababaaab",
            "aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba",
        ]
        .into_iter()
        .map(String::from)
        .collect()
    }

    #[test]
    fn part1_works_with_large_example() {
        assert_eq!(part1(&large_example()), 3);
    }

    #[test]
    fn part2_works_with_large_example() {
        assert_eq!(part2(&large_example()), 12);
    }

    #[test]
    fn is_valid_works() {
        let rule2 = Node(
            Rc::new(Node(Rc::new(Solution), Rc::new(Nil))),
            Rc::new(Node(Rc::new(Nil), Rc::new(Solution))),
        );

        assert!(rule2.is_valid("aa"));
    }

    #[test]
    fn append_solution_works() {
        let rule2 = Node(
            Rc::new(Node(Rc::new(Solution), Rc::new(Nil))),
            Rc::new(Node(Rc::new(Nil), Rc::new(Solution))),
        );
        let rule3 = Node(
            Rc::new(Node(Rc::new(Nil), Rc::new(Solution))),
            Rc::new(Node(Rc::new(Solution), Rc::new(Nil))),
        );
        let append_2_3 = rule2.append_solution(&rule3);

        assert!(append_2_3.is_valid("aaab"));
    }

    #[test]
    fn merge_works() {
        let rule4 = Node(Rc::new(Solution), Rc::new(Nil));
        let rule5 = Node(Rc::new(Nil), Rc::new(Solution));
        let append_4_5 = rule4.append_solution(&rule5);
        let append_5_4 = rule5.append_solution(&rule4);
        let merge_4_5_with_5_4 = append_4_5.merge(&append_5_4);

        assert!(merge_4_5_with_5_4.is_valid("ba"));
    }
}
