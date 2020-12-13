#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Ship {
    current_position: Point,

    // N E S W
    // 0 1 2 3
    facing_direction: u8,
}

impl Ship {
    fn new() -> Ship {
        Ship {
            current_position: Point { x: 0, y: 0 },
            facing_direction: 1,
        }
    }

    fn step(&mut self, instruction: &str) {
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
                0 => self.step(&format!("N{}", value)),
                1 => self.step(&format!("E{}", value)),
                2 => self.step(&format!("S{}", value)),
                3 => self.step(&format!("W{}", value)),
                _ => (),
            },
            _ => (),
        }
    }
}

fn solve(lines: Vec<String>) -> i32 {
    let mut ship = Ship::new();

    for line in lines {
        // println!("--------------------------------------");
        // println!("instruction: {}", line);
        // println!("before: {:#?}", &ship);

        ship.step(&line);

        // println!("after: {:#?}", &ship);
    }

    ship.current_position.x.abs() + ship.current_position.y.abs()
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
                vec!["F10", "N3", "F7", "R90", "F11",]
                    .into_iter()
                    .map(String::from)
                    .collect()
            ),
            25
        );
    }
}
