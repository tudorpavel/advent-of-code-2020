#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Ship {
    current_position: Point,

    // Only for Part 1
    // N E S W
    // 0 1 2 3
    facing_direction: u8,

    // Only for Part 2
    // always relative to current_position
    waypoint: Point,
}

impl Ship {
    fn new() -> Ship {
        Ship {
            current_position: Point { x: 0, y: 0 },
            facing_direction: 1,
            waypoint: Point { x: 10, y: 1 },
        }
    }

    fn part1_step(&mut self, instruction: &str) {
        let (code, value) = instruction.split_at(1);
        let value: i32 = value.parse().unwrap();

        match code {
            "N" => self.current_position.y += value,
            "S" => self.current_position.y -= value,
            "E" => self.current_position.x += value,
            "W" => self.current_position.x -= value,
            "L" => {
                self.facing_direction = self.facing_direction.wrapping_sub((value / 90) as u8) % 4;
            }
            "R" => {
                self.facing_direction = (self.facing_direction + (value / 90) as u8) % 4;
            }
            "F" => match self.facing_direction {
                0 => self.part1_step(&format!("N{}", value)),
                1 => self.part1_step(&format!("E{}", value)),
                2 => self.part1_step(&format!("S{}", value)),
                3 => self.part1_step(&format!("W{}", value)),
                _ => (),
            },
            _ => (),
        }
    }

    fn part2_step(&mut self, instruction: &str) {
        let (code, value) = instruction.split_at(1);
        let value: i32 = value.parse().unwrap();

        match code {
            "N" => self.waypoint.y += value,
            "S" => self.waypoint.y -= value,
            "E" => self.waypoint.x += value,
            "W" => self.waypoint.x -= value,
            "L" => match value {
                90 => {
                    let y = self.waypoint.y;
                    self.waypoint.y = self.waypoint.x;
                    self.waypoint.x = -y;
                }
                180 => {
                    self.part2_step(&format!("L{}", 90));
                    self.part2_step(&format!("L{}", 90));
                }
                270 => {
                    self.part2_step(&format!("L{}", 90));
                    self.part2_step(&format!("L{}", 90));
                    self.part2_step(&format!("L{}", 90));
                }
                _ => panic!("expected rotation 90, 180 or 270, got {}", value),
            },
            "R" => match value {
                90 => {
                    let x = self.waypoint.x;
                    self.waypoint.x = self.waypoint.y;
                    self.waypoint.y = -x;
                }
                180 => {
                    self.part2_step(&format!("R{}", 90));
                    self.part2_step(&format!("R{}", 90));
                }
                270 => {
                    self.part2_step(&format!("R{}", 90));
                    self.part2_step(&format!("R{}", 90));
                    self.part2_step(&format!("R{}", 90));
                }
                _ => panic!("expected rotation 90, 180 or 270, got {}", value),
            },
            "F" => {
                self.current_position.x += value * self.waypoint.x;
                self.current_position.y += value * self.waypoint.y;
            }
            _ => (),
        }
    }
}

fn part1(lines: &[String]) -> i32 {
    let mut ship = Ship::new();

    for line in lines {
        // println!("--------------------------------------");
        // println!("instruction: {}", line);
        // println!("before: {:#?}", &ship);

        ship.part1_step(&line);

        // println!("after: {:#?}", &ship);
    }

    ship.current_position.x.abs() + ship.current_position.y.abs()
}

fn part2(lines: &[String]) -> i32 {
    let mut ship = Ship::new();

    for line in lines {
        // println!("--------------------------------------");
        // println!("instruction: {}", line);
        // println!("before: {:#?}", &ship);

        ship.part2_step(&line);

        // println!("after: {:#?}", &ship);
    }

    ship.current_position.x.abs() + ship.current_position.y.abs()
}

fn solve(lines: Vec<String>) -> (i32, i32) {
    (part1(&lines), part2(&lines))
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
                vec!["F10", "N3", "F7", "R90", "F11",]
                    .into_iter()
                    .map(String::from)
                    .collect()
            ),
            (25, 286)
        );
    }
}
