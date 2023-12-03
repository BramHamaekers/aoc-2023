use core::num;

fn main() {
    let input = include_str!("./input/input.txt");
    let output = part1(input);
    dbg!(output);
}

pub fn build_number (number: i32, digit: i32) -> i32 {
    return (number * 10) + digit;
}

pub fn is_symbol(char: char) -> bool {
    return char != '.' && !char.is_digit(10);
}

pub fn has_adjacent_symbol (matrix: &Vec<Vec<char>>, i: usize, j: usize) -> bool {
        let directions = vec![
        (-1, 0), (1, 0), (0, -1), (0, 1), // horizontal and vertical
        (-1, -1), (-1, 1), (1, -1), (1, 1), // diagonal
    ];
    directions
        .iter()
        .any(|&(x, y)| {
            let new_x = (i as i32 + x as i32) as usize;
            let new_y = (j as i32 + y as i32) as usize;
            new_x < matrix.len() && new_y < matrix[0].len() && is_symbol(matrix[new_x][new_y])
        })
    }

pub fn part1(input: &str) -> i32 {
    let matrix: Vec<Vec<char>> = input
        .trim()
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let mut result = 0;

    for (i, row) in matrix.iter().enumerate() {
        let mut number = 0;
        let mut is_part_number = false;

        for (j, &element) in row.iter().enumerate() {
            if element.is_digit(10) {
                if !is_part_number {is_part_number = has_adjacent_symbol(&matrix, i, j)}
                number = build_number(number, element as i32 - '0' as i32);
                continue;
            }
            if is_part_number {
                result += number
            }
            number = 0;
            is_part_number = false;
        }
        if is_part_number {result += number}
    }
    result
}

#[cfg(test)]
mod tests {
    use crate::part1;

    #[test]
    fn part1_test() {
        let expected_output = 4361;
        let result = part1(
"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..",
        );
        assert_eq!(result, expected_output);
    }

    #[test]
    fn part1_edge_case() {
        let expected_output = 3;
        let result = part1(
"*1#2",
        );
        assert_eq!(result, expected_output);
    }
}
