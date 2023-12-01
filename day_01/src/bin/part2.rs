use std::collections::HashMap;

fn create_key_map() -> HashMap<&'static str, i32> {
    [
        ("zero", 0), ("one", 1), ("two", 2), ("three", 3), ("four", 4),
        ("five", 5), ("six", 6), ("seven", 7), ("eight", 8), ("nine", 9),
    ].iter().cloned().collect()
}

fn main() {
    let input = include_str!("./input/input.txt");
    let output = part2(input);
    dbg!(output);
}

fn find_first_digit(line: &str) -> i32 {
    let key_map: HashMap<&str, i32> = create_key_map();
    let mut searched = String::new();
    for char in line.chars() {
        if char.is_ascii_digit() {
            return char as i32 - 48;
        }
        searched.push(char);
        for key in key_map.keys() {
            if searched.contains(key) {
                return key_map[key];
            }
        }
    }
    return 0;
}

fn find_last_digit(line: &str) -> i32 {
    let key_map: HashMap<&str, i32> = create_key_map();
    let mut searched = String::new();
    for char in line.chars().rev() {
        if char.is_ascii_digit() {
            return char as i32 - 48;
        }
        searched.insert(0, char);
        for key in key_map.keys() {
            if searched.contains(key) {
                return key_map[key];
            }
        }
    }
    return 0;
}

pub fn part2(input: &str) -> i32 {
    let lines: Vec<String> = input.lines().map(String::from).collect();
    let mut values = Vec::<i32>::new();
    for line in lines {
        values.push(find_first_digit(&line) * 10 + find_last_digit(&line));
    }
    return values.iter().sum();
}

#[cfg(test)]
mod tests {
    use crate::part2;

    #[test]
    fn part2_test() {
        let expected_output = 281;
        let result = part2("two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen");
        assert_eq!(result, expected_output);
    }
}