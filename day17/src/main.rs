const DELTA_NEIGHBORS: [(i32, i32, i32); 26] = [
    (-1, -1, -1),
    (-1, -1, 0),
    (-1, -1, 1),
    (-1, 0, -1),
    (-1, 0, 0),
    (-1, 0, 1),
    (-1, 1, -1),
    (-1, 1, 0),
    (-1, 1, 1),
    (0, -1, -1),
    (0, -1, 0),
    (0, -1, 1),
    (0, 0, -1),
    (0, 0, 1),
    (0, 1, -1),
    (0, 1, 0),
    (0, 1, 1),
    (1, -1, -1),
    (1, -1, 0),
    (1, -1, 1),
    (1, 0, -1),
    (1, 0, 0),
    (1, 0, 1),
    (1, 1, -1),
    (1, 1, 0),
    (1, 1, 1),
];
const CUBE_SIZE: usize = 50;
const MID: usize = CUBE_SIZE / 2;

type Cube = Vec<Vec<Vec<bool>>>;

fn step_simulation(cube: &Cube) -> Cube {
    let mut new_cube = cube.to_owned();

    for i in 1..CUBE_SIZE - 1 {
        for j in 1..CUBE_SIZE - 1 {
            for k in 1..CUBE_SIZE - 1 {
                let alive_neighbor_count = DELTA_NEIGHBORS
                    .iter()
                    .filter(|(di, dj, dk)| {
                        let ii = (i as i32 + di) as usize;
                        let jj = (j as i32 + dj) as usize;
                        let kk = (k as i32 + dk) as usize;

                        cube[ii][jj][kk]
                    })
                    .count();

                if cube[i][j][k] && !(2..=3).contains(&alive_neighbor_count) {
                    new_cube[i][j][k] = false;
                } else if !cube[i][j][k] && alive_neighbor_count == 3 {
                    new_cube[i][j][k] = true;
                }
            }
        }
    }

    new_cube
}

fn alive_cell_count(cube: &Cube) -> u32 {
    let mut count = 0;

    for i in 0..CUBE_SIZE {
        for j in 0..CUBE_SIZE {
            for k in 0..CUBE_SIZE {
                if cube[i][j][k] {
                    count += 1;
                }
            }
        }
    }

    count
}

fn solve(lines: &[String]) -> u32 {
    let mut cube: Cube = vec![vec![vec![false; CUBE_SIZE]; CUBE_SIZE]; CUBE_SIZE];

    for (i, line) in lines.iter().enumerate() {
        for (j, cell) in line.chars().enumerate() {
            cube[MID + i][MID + j][MID] = cell == '#';
        }
    }

    for _ in 0..6 {
        cube = step_simulation(&cube);
    }

    alive_cell_count(&cube)
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
        let lines: Vec<String> = vec![".#.", "..#", "###"]
            .into_iter()
            .map(String::from)
            .collect();

        assert_eq!(solve(&lines), 112);
    }
}
