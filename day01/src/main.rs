use itertools::Itertools;

const YEAR: i32 = 2020;

fn find_combination(nums: Vec<i32>, length: usize) -> Vec<i32> {
    for combination in nums.into_iter().combinations(length) {
        if combination.iter().sum::<i32>() == YEAR {
            return combination;
        }
    }

    vec![]
}

fn part1(nums: Vec<i32>) -> i32 {
    let combination = find_combination(nums, 2);

    combination.iter().product()
}

fn part2(nums: Vec<i32>) -> i32 {
    let combination = find_combination(nums, 3);

    combination.iter().product()
}

fn main() {
    let nums = utils::read_nums();
    println!("Part 1: {}", part1(nums.clone()));
    println!("Part 2: {}", part2(nums.clone()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_combination_of_length_2() {
        assert_eq!(
            find_combination(vec![1721, 979, 366, 299, 675, 1456], 2),
            vec![1721, 299]
        );
    }

    #[test]
    fn find_combination_of_length_3() {
        assert_eq!(
            find_combination(vec![1721, 979, 366, 299, 675, 1456], 3),
            vec![979, 366, 675]
        );
    }

    #[test]
    fn find_combination_with_empty_vector() {
        assert_eq!(
            find_combination(vec![], 2),
            vec![]
        );
    }

    #[test]
    fn part1_works() {
        assert_eq!(
            part1(vec![1721, 979, 366, 299, 675, 1456]),
            514579
        );
    }

    #[test]
    fn part2_works() {
        assert_eq!(
            part2(vec![1721, 979, 366, 299, 675, 1456]),
            241861950
        );
    }
}
