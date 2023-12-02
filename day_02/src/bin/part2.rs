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
            if let [amount, color] = ball.split(" ").collect::<Vec<&str>>().as_slice() {
            let amount = amount.parse::<i32>().unwrap_or_default();
            match color {
                &"blue" => blue = if amount > blue {amount} else {blue},
                &"green" => green = if amount > green {amount} else {green},
                &"red" => red = if amount > red {amount} else {red},
                _ => (),
                }
            }
        }
    }
    return blue * red * green;

}

pub fn part2(input: &str) -> i32 {
    let lines: Vec<String> = input.lines().map(String::from).collect();
    let mut result: i32 = 0;

    for line in lines {
        let vector: Vec<&str> = line.split(": ").collect();
        if let [_, balls] = vector.as_slice() {
            result += get_power(balls)
        }
    }
    return result;
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
