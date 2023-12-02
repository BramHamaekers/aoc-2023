fn main() {
    let input = include_str!("./input/input.txt");
    let output = part1(input);
    dbg!(output);
}

pub fn part1(input: &str) -> i32 {
    let lines: Vec<String> = input.lines().map(String::from).collect();
    return 0;
}

#[cfg(test)]
mod tests {
    use crate::part1;

    #[test]
    fn part1_test() {
        let expected_output = 0;
        let result = part1("",
        );
        assert_eq!(result, expected_output);
    }
}
