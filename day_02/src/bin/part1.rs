fn main() {
    let input = include_str!("./input/input.txt");
    let output = part1(input);
    dbg!(output);
}

pub fn is_valid_game (balls: &str) -> bool {
    for set in balls.split("; ") {
        let mut blue = 0;
        let mut red = 0;
        let mut green = 0;
        for ball in set.split(", ") {
            if let [amount, color] = ball.split(" ").collect::<Vec<&str>>().as_slice() {
            let amount = amount.parse::<i32>().unwrap_or_default();
            match color {
                &"blue" => blue += amount,
                &"green" => green += amount,
                &"red" => red += amount,
                _ => (),
                }
            }
        }
        if red > 12 || green > 13 || blue > 14 {return false;}
    }
    return true
}
    


pub fn part1(input: &str) -> i32 {
    let lines: Vec<String> = input.lines().map(String::from).collect();
    let mut result: i32 = 0;

    for line in lines {
        let vector: Vec<&str> = line.split(": ").collect();
        if let [game_id, balls] = vector.as_slice() {
            let game_id = game_id[5..].parse::<i32>().unwrap_or_default();
            if is_valid_game(balls) {
                result += game_id;
            }
        }
    }
    return result;
    }

    

#[cfg(test)]
mod tests {
    use crate::part1;

    #[test]
    fn part1_test() {
        let expected_output = 8;
        let result = part1("
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
        ",
        );
        assert_eq!(result, expected_output);
    }
}
