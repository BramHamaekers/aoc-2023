fn main() {
    let input = include_str!("./input/input.txt");
    let output = part1(input);
    dbg!(output);
}

pub fn part1(input: &str) -> i32 {
    let points: i32 = input.lines()
         .map(|line| line.split(": ")
                               .nth(1)
                               .unwrap_or_default()
                               .split("|"))
         .map(|mut s| {
            let list1: Vec<i32> = s.next().unwrap_or_default().split_whitespace().filter_map(|x| x.parse::<i32>().ok()).collect();
            let list2: Vec<i32> = s.next().unwrap_or_default().split_whitespace().filter_map(|x| x.parse::<i32>().ok()).collect();
            let wins: u32 = list1.iter().filter(|x| list2.contains(x)).count().try_into().unwrap_or_default();
            if wins > 0 {2i32.pow(wins - 1)} else {0}
         })
         .sum();
    return points;
}

#[cfg(test)]
mod tests {
    use crate::part1;

    #[test]
    fn part1_test() {
        let expected_output = 13;
        let result = part1("
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
        );
        assert_eq!(result, expected_output);
    }
}
