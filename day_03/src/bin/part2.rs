fn main() {
    let input = include_str!("./input/input.txt");
    let output = part2(input);
    dbg!(output);
}

pub fn part2(input: &str) -> i32 {
    let lines: Vec<String> = input.lines().map(String::from).collect();
    return 0;
}

#[cfg(test)]
mod tests {
    use crate::part2;

    #[test]
    fn part2_test() {
        let expected_output = 0;
        let result = part2("",
        );
        assert_eq!(result, expected_output);
    }
}
