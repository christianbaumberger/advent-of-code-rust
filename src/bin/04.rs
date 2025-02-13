advent_of_code::solution!(4);
use regex::Regex;

fn get_horizontal_lines(input: &str) -> Vec<String> {
    let lines = input.lines();
    let mut lines_vec = Vec::<String>::new();
    for line in lines {
        lines_vec.push(line.to_string());
    }
    lines_vec
}

fn get_vertical_lines(input: &Vec<String>, max_length: usize) -> Vec<String> {
    let mut vertical_lines = Vec::<String>::new();
    for horizontal_pos in 0..max_length {
        let mut vertical_str = String::from("");
        for line in input {
            if let Some(c) = line.chars().nth(horizontal_pos) {
                vertical_str.push(c);
            }
        }
        vertical_lines.push(vertical_str);
    }
    vertical_lines
}

fn get_diag_lines_bottom_left_to_top_right(input: &Vec<String>, max_length: usize) -> Vec<String> {
    let mut diagonal_lines = Vec::<String>::new();
    for horizontal_start_pos in 3..max_length {
        let mut horizontal_pos = horizontal_start_pos;
        let mut diag_str = String::from("");
        for vertical_pos in 0..=horizontal_start_pos {
            if let Some(c) = input[vertical_pos].chars().nth(horizontal_pos) {
                diag_str.push(c);
            }
            if horizontal_pos > 0 {
                horizontal_pos -= 1;
            }
        }
        diagonal_lines.push(diag_str);
    }
    for horizontal_start_pos in 1..(max_length - 3) {
        let mut horizontal_pos = horizontal_start_pos;
        let mut diag_str = String::from("");
        for vertical_pos in (horizontal_pos..=(max_length - 1)).rev() {
            if let Some(c) = input[vertical_pos].chars().nth(horizontal_pos) {
                diag_str.push(c);
            }
            horizontal_pos += 1;
        }
        diagonal_lines.push(diag_str);
    }

    diagonal_lines
}

fn get_diag_lines_top_left_to_bottom_right(input: &Vec<String>, max_length: usize) -> Vec<String> {
    let mut diagonal_lines = Vec::<String>::new();

    for horizontal_start_pos in 0..(max_length - 3) {
        let mut horizontal_pos = horizontal_start_pos;
        let mut diag_str = String::from("");
        for vertical_pos in 0..(max_length - horizontal_pos) {
            if let Some(c) = input[vertical_pos].chars().nth(horizontal_pos) {
                diag_str.push(c);
            }
            horizontal_pos += 1;
        }
        diagonal_lines.push(diag_str);
    }
    for horizontal_start_pos in 3..(max_length - 1) {
        let mut horizontal_pos = horizontal_start_pos;
        let mut diag_str = String::from("");
        for vertical_pos in ((max_length - horizontal_pos - 1)..max_length).rev() {
            if let Some(c) = input[vertical_pos].chars().nth(horizontal_pos) {
                diag_str.push(c);
            }
            if horizontal_pos > 0 {
                horizontal_pos -= 1;
            }
        }
        diagonal_lines.push(diag_str);
    }
    diagonal_lines
}

pub fn part_one(input: &str) -> Option<u64> {
    if input.is_empty() {
        return None;
    }

    let horizontal_lines = get_horizontal_lines(input);
    let max_length = horizontal_lines.first().unwrap().chars().count();

    let mut vertical_lines = get_vertical_lines(&horizontal_lines, max_length);
    let mut diagonal_lines_1 =
        get_diag_lines_bottom_left_to_top_right(&horizontal_lines, max_length);
    let mut diagonal_lines_2 =
        get_diag_lines_top_left_to_bottom_right(&horizontal_lines, max_length);

    let mut lines = horizontal_lines;
    lines.append(&mut vertical_lines);
    lines.append(&mut diagonal_lines_1);
    lines.append(&mut diagonal_lines_2);
    let xmas_re = Regex::new(r"(XMAS)").unwrap();
    let samx_re = Regex::new(r"(SAMX)").unwrap();
    let mut total_counts = 0;
    for line in lines {
        let xmas_counts = xmas_re.find_iter(&line).count();
        let samx_counts = samx_re.find_iter(&line).count();
        total_counts += xmas_counts + samx_counts;
    }

    Some(total_counts as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    if input.is_empty() {
        return None;
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_horizontal_lines() {
        let input = advent_of_code::template::read_file("examples", DAY);
        let result = get_horizontal_lines(&input);
        let mut expected = Vec::<String>::new();
        for _i in 0..9 {
            expected.push("123456789".to_string());
        }
        assert_eq!(expected, result);
    }

    #[test]
    fn test_vertical_lines() {
        let input = advent_of_code::template::read_file("examples", DAY);
        let horizontal_lines = get_horizontal_lines(&input);
        let max_length = horizontal_lines.first().unwrap().chars().count();
        let result = get_vertical_lines(&horizontal_lines, max_length);
        let mut expected = Vec::<String>::new();
        for i in 0..9 {
            let column = (i + 1).to_string().repeat(9);
            expected.push(column);
        }
        assert_eq!(expected, result);
    }

    #[test]
    fn test_diag_lines_bottom_left_to_top_right() {
        let input = advent_of_code::template::read_file("examples", DAY);
        let horizontal_lines = get_horizontal_lines(&input);
        let max_length = horizontal_lines.first().unwrap().chars().count();
        let result = get_diag_lines_bottom_left_to_top_right(&horizontal_lines, max_length);
        let mut expected = Vec::<String>::new();
        expected.push("4321".to_string());
        expected.push("54321".to_string());
        expected.push("654321".to_string());
        expected.push("7654321".to_string());
        expected.push("87654321".to_string());
        expected.push("987654321".to_string());
        expected.push("23456789".to_string());
        expected.push("3456789".to_string());
        expected.push("456789".to_string());
        expected.push("56789".to_string());
        expected.push("6789".to_string());
        assert_eq!(expected, result);
    }

    #[test]
    fn test_diag_lines_top_left_to_bottom_right() {
        let input = advent_of_code::template::read_file("examples", DAY);
        let horizontal_lines = get_horizontal_lines(&input);
        let max_length = horizontal_lines.first().unwrap().chars().count();
        let result = get_diag_lines_top_left_to_bottom_right(&horizontal_lines, max_length);
        let mut expected = Vec::<String>::new();
        expected.push("123456789".to_string());
        expected.push("23456789".to_string());
        expected.push("3456789".to_string());
        expected.push("456789".to_string());
        expected.push("56789".to_string());
        expected.push("6789".to_string());
        expected.push("4321".to_string());
        expected.push("54321".to_string());
        expected.push("654321".to_string());
        expected.push("7654321".to_string());
        expected.push("87654321".to_string());
        assert_eq!(expected, result);
    }

    #[test]
    fn test_part_one() {
        let input = advent_of_code::template::read_file("examples", DAY);
        let result = part_one(&input);
        assert_eq!(result, Some(0));
    }

    #[test]
    fn test_part_one_small_example() {
        let input = advent_of_code::template::read_file_part("examples", DAY, 2);
        let result = part_one(&input);
        assert_eq!(result, Some(18));
    }


    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
