use std::vec;

fn main() {
    let input = include_str!("./input/input.txt");
    let output = part1(input);
    dbg!(output);
}

pub struct Game {
    id: i32,
    blue: i32,
    green: i32,
    red: i32
}

pub fn create_game (game_id: i32) -> Game {
    return Game {
        id: game_id,
        blue: 0,
        green: 0,
        red: 0
    }
}

pub fn add_balls (mut game: Game, balls: &str) -> Game {
    let sets: Vec<&str> = balls.split("; ").collect();
    for set in sets {
        let balls: Vec<&str> = set.split(", ").collect();
        for ball in balls {
            let ball_breakdown: Vec<&str> = ball.split(" ").collect();
            if let [amount, color] = ball_breakdown.as_slice() {
            let amount = amount.parse::<i32>();
            if let Ok(amount) = amount {
                match color {
                    &"blue" => game.blue += amount,
                    &"green" => game.green += amount,
                    &"red" => game.red += amount,
                    _ => (),
                    }
                }
            }
        }
    }
    return game
}

pub fn is_valid_game(game: &Game) -> bool {
    if game.red > 12 || game.green > 13 || game.blue > 13 {return false;}
    return true;
}

pub fn part1(input: &str) -> i32 {
    let lines: Vec<String> = input.lines().map(String::from).collect();
    let mut result: i32 = 0;
    let mut games: Vec<Game> = Vec::new();
    for line in lines {
        let vector: Vec<&str> = line.split(": ").collect();
        if let [game_id, balls] = vector.as_slice() {
            let game_id = game_id[5..].parse::<i32>();
            if let Ok(game_id) = game_id {
                 let game = create_game(game_id);
                 let game = add_balls(game, balls);
                 games.push(game);
                }
            }
        }
    for game in games.iter() {
        if is_valid_game(game) {
            println!("{:?}", game.id);
            result += game.id;
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
