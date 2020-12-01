const YEAR: i32 = 2020;

fn find_pair(nums: Vec<i32>) -> (i32, i32) {
    if nums.len() < 1 {
        return (0, 0)
    }

    let mut left = 0;
    let mut right = nums.len() - 1;
    let mut sorted = nums.clone();
    sorted.sort();

    while left < right {
        let sum = sorted[left] + sorted[right];

        if sum == YEAR {
            return (sorted[left], sorted[right]);
        }

        if sum > YEAR {
            right -= 1;
        } else if sum < YEAR {
            left += 1;
        }
    }

    (0, 0)
}

fn part1(nums: Vec<i32>) -> i32 {
    let (a, b) = find_pair(nums);

    a * b
}

fn main() {
    let nums = utils::read_nums();
    println!("Part 1: {}", part1(nums));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_pair_with_vector() {
        assert_eq!(
            find_pair(vec![1721, 979, 366, 299, 675, 1456]),
            (299, 1721)
        );
    }

    #[test]
    fn find_pair_with_empty_vector() {
        assert_eq!(
            find_pair(vec![]),
            (0, 0)
        );
    }

    #[test]
    fn part1_works() {
        assert_eq!(
            part1(vec![1721, 979, 366, 299, 675, 1456]),
            514579
        );
    }
}
