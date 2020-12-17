mod common {
    pub const CUBE_SIZE: usize = 27;
    pub const MID: usize = CUBE_SIZE / 2;

    pub type Cube = Vec<Vec<Vec<bool>>>;
}

mod part1 {
    use crate::common::Cube;
    use crate::common::CUBE_SIZE;
    use crate::common::MID;

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

    pub fn solve(lines: &[String]) -> u32 {
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
}

mod part2 {
    use crate::common::Cube;
    use crate::common::CUBE_SIZE;
    use crate::common::MID;

    type HyperCube = Vec<Cube>;

    fn build_delta_neighors() -> Vec<(i32, i32, i32, i32)> {
        let mut result = vec![];

        for i in -1..=1 {
            for j in -1..=1 {
                for k in -1..=1 {
                    for l in -1..=1 {
                        if (i, j, k, l) == (0, 0, 0, 0) {
                            continue;
                        } else {
                            result.push((i, j, k, l));
                        }
                    }
                }
            }
        }

        result
    }

    fn step_simulation(
        hyper_cube: &HyperCube,
        delta_neighbors: &Vec<(i32, i32, i32, i32)>,
    ) -> HyperCube {
        let mut new_hyper_cube = hyper_cube.to_owned();

        for i in 1..CUBE_SIZE - 1 {
            for j in 1..CUBE_SIZE - 1 {
                for k in 1..CUBE_SIZE - 1 {
                    for l in 1..CUBE_SIZE - 1 {
                        let alive_neighbor_count = delta_neighbors
                            .iter()
                            .filter(|(di, dj, dk, dl)| {
                                let ii = (i as i32 + di) as usize;
                                let jj = (j as i32 + dj) as usize;
                                let kk = (k as i32 + dk) as usize;
                                let ll = (l as i32 + dl) as usize;

                                hyper_cube[ii][jj][kk][ll]
                            })
                            .count();

                        if hyper_cube[i][j][k][l] && !(2..=3).contains(&alive_neighbor_count) {
                            new_hyper_cube[i][j][k][l] = false;
                        } else if !hyper_cube[i][j][k][l] && alive_neighbor_count == 3 {
                            new_hyper_cube[i][j][k][l] = true;
                        }
                    }
                }
            }
        }

        new_hyper_cube
    }

    fn alive_cell_count(hyper_cube: &HyperCube) -> u32 {
        let mut count = 0;

        for i in 0..CUBE_SIZE {
            for j in 0..CUBE_SIZE {
                for k in 0..CUBE_SIZE {
                    for l in 0..CUBE_SIZE {
                        if hyper_cube[i][j][k][l] {
                            count += 1;
                        }
                    }
                }
            }
        }

        count
    }

    pub fn solve(lines: &[String]) -> u32 {
        let mut hyper_cube: HyperCube =
            vec![vec![vec![vec![false; CUBE_SIZE]; CUBE_SIZE]; CUBE_SIZE]; CUBE_SIZE];
        let delta_neighbors = build_delta_neighors();

        for (i, line) in lines.iter().enumerate() {
            for (j, cell) in line.chars().enumerate() {
                hyper_cube[MID + i][MID + j][MID][MID] = cell == '#';
            }
        }

        for _ in 0..6 {
            hyper_cube = step_simulation(&hyper_cube, &delta_neighbors);
        }

        alive_cell_count(&hyper_cube)
    }
}

fn main() {
    let lines = utils::read_lines();
    println!("Part 1: {}", crate::part1::solve(&lines));
    println!("Part 2: {}", crate::part2::solve(&lines));
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1_works() {
        let lines: Vec<String> = vec![".#.", "..#", "###"]
            .into_iter()
            .map(String::from)
            .collect();

        assert_eq!(crate::part1::solve(&lines), 112);
    }

    #[test]
    fn part2_works() {
        let lines: Vec<String> = vec![".#.", "..#", "###"]
            .into_iter()
            .map(String::from)
            .collect();

        assert_eq!(crate::part2::solve(&lines), 848);
    }
}
