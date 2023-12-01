fn main() {
    let input = include_str!("./input/input.txt");
    let output = part2(input);
    dbg!(output);
}

fn find_digit(line: &str) -> i32 {
    line.chars()
        .find(|c| c.is_ascii_digit())
        .map(|c| c as i32 - 48)
        .unwrap_or(0)
}

pub fn part2(input: &str) -> i32 {
    let lines: Vec<String> = input.lines().map(String::from).collect();
    let mut values = Vec::<i32>::new();
    for mut line in lines {
        line =  line.replace("one", "o1e").replace("two", "t2o").replace("three", "thre3e")
                    .replace("four", "f4ur").replace("five", "fi5e").replace("six", "s6x")
                    .replace("seven", "se7en").replace("eight", "ei8ht").replace("nine", "n9ne");
        values.push(find_digit(&line) * 10 + find_digit(&line.chars().rev().collect::<String>()));
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