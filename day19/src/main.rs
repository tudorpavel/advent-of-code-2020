use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
enum Tree {
    Node(RefCell<Rc<Tree>>, RefCell<Rc<Tree>>),
    Solution,
    Nil,
}

use Tree::{Nil, Node, Solution};

impl Tree {
    fn build(rules: &[&str], rule_id: usize) -> Rc<Tree> {
        match rules[rule_id] {
            "\"a\"" => Rc::new(Node(
                RefCell::new(Rc::new(Solution)),
                RefCell::new(Rc::new(Nil)),
            )),
            "\"b\"" => Rc::new(Node(
                RefCell::new(Rc::new(Nil)),
                RefCell::new(Rc::new(Solution)),
            )),
            composed_rule => {
                let trees: Vec<Rc<Tree>> = composed_rule
                    .split(" | ")
                    .map(|append_rules| {
                        let ids: Vec<usize> = append_rules
                            .split(' ')
                            .map(|s| s.parse().unwrap())
                            .collect();
                        let tree = Tree::build(&rules, *ids.first().unwrap());
                        for id in ids[1..].iter() {
                            tree.append_solution(&Tree::build(&rules, *id));
                        }

                        tree
                    })
                    .collect();
                let tree = trees.first().unwrap();
                for other in trees[1..].iter() {
                    tree.merge(&other);
                }

                Rc::clone(&tree)
            }
        }
    }

    fn is_valid(&self, message: &str) -> bool {
        match (self, message) {
            (Solution, "") => true,
            (_, "") => false,
            (Node(left, right), _) => {
                if &message[..1] == "a" {
                    left.borrow().is_valid(&message[1..])
                } else {
                    right.borrow().is_valid(&message[1..])
                }
            }
            _ => false,
        }
    }

    fn append_solution(&self, other: &Rc<Tree>) {
        if let Node(left, right) = self {
            {
                let mut left = left.borrow_mut();
                if let Solution = **left {
                    *left = Rc::clone(&other);
                    return;
                };

                let mut right = right.borrow_mut();
                if let Solution = **right {
                    *right = Rc::clone(&other);
                    return;
                };
            }

            left.borrow().append_solution(&other);
            right.borrow().append_solution(&other);
        }
    }

    fn merge(&self, other: &Tree) {
        if let (Node(l1, r1), Node(l2, r2)) = (self, other) {
            {
                let mut l1 = l1.borrow_mut();
                if let Nil = **l1 {
                    *l1 = Rc::clone(&l2.borrow());
                    return;
                };

                let mut r1 = r1.borrow_mut();
                if let Nil = **r1 {
                    *r1 = Rc::clone(&r2.borrow());
                    return;
                };
            }

            l1.borrow().merge(&l2.borrow());
            r1.borrow().merge(&r2.borrow());
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

    // let bla = Tree::build(&rules, 42);
    // println!("{:#?}", bla);

    // return 0;
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
            RefCell::new(Rc::new(Node(
                RefCell::new(Rc::new(Solution)),
                RefCell::new(Rc::new(Nil)),
            ))),
            RefCell::new(Rc::new(Node(
                RefCell::new(Rc::new(Nil)),
                RefCell::new(Rc::new(Solution)),
            ))),
        );

        assert!(rule2.is_valid("aa"));
    }

    #[test]
    fn append_solution_works() {
        let rule2 = Rc::new(Node(
            RefCell::new(Rc::new(Node(
                RefCell::new(Rc::new(Solution)),
                RefCell::new(Rc::new(Nil)),
            ))),
            RefCell::new(Rc::new(Node(
                RefCell::new(Rc::new(Nil)),
                RefCell::new(Rc::new(Solution)),
            ))),
        ));
        let rule3 = Rc::new(Node(
            RefCell::new(Rc::new(Node(
                RefCell::new(Rc::new(Nil)),
                RefCell::new(Rc::new(Solution)),
            ))),
            RefCell::new(Rc::new(Node(
                RefCell::new(Rc::new(Solution)),
                RefCell::new(Rc::new(Nil)),
            ))),
        ));

        rule2.append_solution(&rule3);

        assert!(rule2.is_valid("aaab"));
    }

    #[test]
    fn merge_works() {
        let rule4 = Rc::new(Node(
            RefCell::new(Rc::new(Solution)),
            RefCell::new(Rc::new(Nil)),
        ));
        let rule5 = Rc::new(Node(
            RefCell::new(Rc::new(Nil)),
            RefCell::new(Rc::new(Solution)),
        ));
        rule4.append_solution(&rule5);
        rule5.append_solution(&rule4);
        rule4.merge(&rule5);

        assert!(rule4.is_valid("ba"));
    }
}
