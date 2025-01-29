advent_of_code::solution!(3);
use regex::Regex;

pub fn part_one(input: &str) -> Option<u64> {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let mut total_mul_sum = 0;
    for (_, [first_number, second_number]) in re.captures_iter(input).map(|c| c.extract()) {
        total_mul_sum += first_number.parse::<u64>().unwrap() * second_number.parse::<u64>().unwrap();
    }
    Some(total_mul_sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    None
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
