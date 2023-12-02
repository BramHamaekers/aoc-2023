fn main() {
    let input = include_str!("./input/input.txt");
    let output = part2(input);
    dbg!(output);
}

pub fn get_power (balls: &str) -> i32 {
    let mut blue = 0;
    let mut red = 0;
    let mut green = 0;

    for set in balls.split("; ") {
        for ball in set.split(", ") {
            if let [amount, color] = ball.split_whitespace().collect::<Vec<&str>>().as_slice() {
                let amount = amount.parse::<i32>().unwrap_or_default();
                match color {
                    &"blue" => blue = blue.max(amount),
                    &"green" => green = green.max(amount),
                    &"red" => red = red.max(amount),
                    _ => (),
                }
            }
        }
    }
    blue * red * green
}

pub fn part2(input: &str) -> i32 {
    return input.lines()
                .map(|line| {
                    let balls = line.split(": ").nth(1).unwrap_or_default();
                    get_power(balls)
                })
                .sum()
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
