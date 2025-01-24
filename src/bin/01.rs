advent_of_code::solution!(1);

use regex::Regex;

pub fn part_one(input: &str) -> Option<u64> {
    let lines = input.lines();

    let re = Regex::new(r"(?<left>\d+)\s+(?<right>\d+)").unwrap();

    // create two lists with left and right values
    let mut left_numbers = Vec::new();
    let mut right_numbers = Vec::new();
    for line in lines {
        if let Some(caps) = re.captures(line) {
            let left_number = caps["left"].parse::<u64>();
            let right_number = caps["right"].parse::<u64>();
            if let Ok(num_left) = left_number {
                left_numbers.push(num_left);
            }
            if let Ok(num_right) = right_number {
                right_numbers.push(num_right);
            }
        } else {
            println!("Could not find left and right number in line!");
            continue;
        };
    }
    assert_eq!(left_numbers.len(), right_numbers.len());

    // order lists
    left_numbers.sort();
    right_numbers.sort();

    // compute pairwise distances
    let mut diff_numbers = Vec::new();
    for (left, right) in left_numbers.iter().zip(right_numbers) {
        let diff = left.abs_diff(right);
        diff_numbers.push(diff);
    }

    // sum up distances
    let summed_diff: u64 = diff_numbers.iter().sum();
    Some(summed_diff)
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(42)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
