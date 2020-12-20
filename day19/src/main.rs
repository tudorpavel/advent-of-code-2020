#[derive(Clone)]
enum Tree {
    Node(Box<Tree>, Box<Tree>),
    Solution,
    Nil,
}

use Tree::{Nil, Node, Solution};

impl Tree {
    fn build(rules: &[&str], rule_id: usize) -> Tree {
        match rules[rule_id] {
            "\"a\"" => Node(Box::new(Solution), Box::new(Nil)),
            "\"b\"" => Node(Box::new(Nil), Box::new(Solution)),
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

    fn append_solution(&self, other: &Tree) -> Tree {
        match self {
            Solution => other.clone(),
            Node(left, right) => Node(
                Box::new(left.append_solution(&other)),
                Box::new(right.append_solution(&other)),
            ),
            Nil => Nil,
        }
    }

    fn merge(&self, other: &Tree) -> Tree {
        match (self, other) {
            (Nil, _) => other.clone(),
            (_, Nil) => self.clone(),
            (Node(l1, r1), Node(l2, r2)) => Node(Box::new(l1.merge(l2)), Box::new(r1.merge(r2))),
            _ => Nil,
        }
    }
}

fn solve(lines: &[String]) -> usize {
    let groups: Vec<&[String]> = lines.split(|l| l == "").collect();
    let rules = groups[0];
    let rules: Vec<&str> = rules.iter().fold(vec![""; rules.len()], |mut acc, rule| {
        let parts: Vec<&str> = rule.split(": ").collect();
        let rule_id: usize = parts[0].parse().unwrap();

        acc[rule_id] = parts[1];

        acc
    });

    let rule_tree = Tree::build(&rules, 0);

    groups[1]
        .iter()
        .filter(|message| rule_tree.is_valid(message))
        .count()
}

fn main() {
    let lines = utils::read_lines();
    println!("Part 1: {}", solve(&lines));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_works() {
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

        assert_eq!(solve(&lines), 2);
    }

    #[test]
    fn is_valid_works() {
        let rule2 = Node(
            Box::new(Node(Box::new(Solution), Box::new(Nil))),
            Box::new(Node(Box::new(Nil), Box::new(Solution))),
        );

        assert!(rule2.is_valid("aa"));
    }

    #[test]
    fn append_solution_works() {
        let rule2 = Node(
            Box::new(Node(Box::new(Solution), Box::new(Nil))),
            Box::new(Node(Box::new(Nil), Box::new(Solution))),
        );
        let rule3 = Node(
            Box::new(Node(Box::new(Nil), Box::new(Solution))),
            Box::new(Node(Box::new(Solution), Box::new(Nil))),
        );
        let append_2_3 = rule2.append_solution(&rule3);

        assert!(append_2_3.is_valid("aaab"));
    }

    #[test]
    fn merge_works() {
        let rule4 = Node(Box::new(Solution), Box::new(Nil));
        let rule5 = Node(Box::new(Nil), Box::new(Solution));
        let append_4_5 = rule4.append_solution(&rule5);
        let append_5_4 = rule5.append_solution(&rule4);
        let merge_4_5_with_5_4 = append_4_5.merge(&append_5_4);

        assert!(merge_4_5_with_5_4.is_valid("ba"));
    }
}
