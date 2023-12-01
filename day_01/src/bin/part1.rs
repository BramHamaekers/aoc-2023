fn main() {
    let input = include_str!("./input/input.txt");
    let output = part1(input);
    dbg!(output);
}

fn find_digit(line: &str) -> i32 {
    line.chars()
        .find(|c| c.is_ascii_digit())
        .map(|c| c as i32 - 48)
        .unwrap_or(0)
}

pub fn part1(input: &str) -> i32 {
    let lines: Vec<String> = input.lines().map(String::from).collect();
    let mut values = Vec::<i32>::new();
    for line in lines {
        values.push(find_digit(&line) * 10 + find_digit(&line.chars().rev().collect::<String>()));
    }
    return values.iter().sum();
}

#[cfg(test)]
mod tests {
    use crate::part1;

    #[test]
    fn part1_test() {
        let expected_output = 142;
        let result = part1(
            "1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet",
        );
        assert_eq!(result, expected_output);
    }
}
