advent_of_code::solution!(3);
use regex::Regex;

pub fn part_one(input: &str) -> Option<u64> {
    if input.is_empty() {
        return None;
    }

    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let mut total_mul_sum = 0;
    for (_, [first_number, second_number]) in re.captures_iter(input).map(|c| c.extract()) {
        total_mul_sum += first_number.parse::<u64>().unwrap_or(0) * second_number.parse::<u64>().unwrap_or(0);
    }
    Some(total_mul_sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    if input.is_empty() {
        return None;
    }

    let re = Regex::new(r"do\(\)|don't\(\)|mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let mut total_mul_sum = 0;
    let mut enable = true;
    for cap in re.captures_iter(input) {
        let text = cap.get(0).unwrap().as_str();
        match text {
            "do()" => enable = true,
            "don't()" => enable = false,
            _ => {
                if enable {
                    let first_number = cap.get(1).unwrap().as_str().parse::<u64>().unwrap_or(0);
                    let second_number = cap.get(2).unwrap().as_str().parse::<u64>().unwrap_or(0);
                    total_mul_sum += first_number * second_number;
                }
            }
        }
    }
    Some(total_mul_sum)
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
