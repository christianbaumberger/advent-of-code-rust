advent_of_code::solution!(2);

struct Report {
    levels: Vec<i32>,
    consecutive_diffs: Vec<i32>,
}
impl Report {
    fn parse(input: &str) -> Self {
        let numbers_list: Vec<i32> = input.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect();
        let consecutive_diff: Vec<i32> = Report::get_consecutive_diffs(&numbers_list);
        Self{levels: numbers_list, consecutive_diffs: consecutive_diff}
    }
    fn get_consecutive_diffs(numbers_list: &Vec<i32>) -> Vec<i32>{
        numbers_list.windows(2).map(|x| x[1] - x[0]).collect()
    }

    fn strictly_increasing_or_decreasing(&self) -> bool {
        self.consecutive_diffs.iter().all(|&x| x > 0) || self.consecutive_diffs.iter().all(|&x| x < 0)
    }

    fn check_adjacent_levels(&self) -> bool {
        self.consecutive_diffs.iter().all(|&x| (x.abs() >= 1) && (x.abs() <= 3))
    }

    fn is_safe(&self) -> bool {
        self.strictly_increasing_or_decreasing() && self.check_adjacent_levels()
    }

    fn is_safe_2(&self) -> bool {
        if self.strictly_increasing_or_decreasing() && self.check_adjacent_levels() {
            return true;
        } else {
            // iterate trough vector and remove each element at a time
            for index in 0..self.levels.len() {
                let mut levels_clone = self.levels.clone();
                levels_clone.remove(index);
                let consecutive_diff: Vec<i32> = Report::get_consecutive_diffs(&levels_clone);
                let report = Report{levels: levels_clone, consecutive_diffs: consecutive_diff};
                if report.is_safe() {
                    return true;
                }
            }
            false
        }
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    if input.is_empty() {
        return None;
    }

    let lines = input.lines();
    let mut safe_reports = 0;
    for line in lines {
        let report = Report::parse(line);
        if report.is_safe() {
            safe_reports += 1;
        }
    }

    Some(safe_reports)
}

pub fn part_two(input: &str) -> Option<u64> {
    if input.is_empty() {
        return None;
    }

    let lines = input.lines();
    let mut safe_reports = 0;
    for line in lines {
        let report = Report::parse(line);
        if report.is_safe_2() {
            safe_reports += 1;
        }
    }

    Some(safe_reports)
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
