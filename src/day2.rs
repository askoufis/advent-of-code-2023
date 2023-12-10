#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<u32> {
    vec![1]
}

#[aoc(day2, part1)]
pub fn solve_part1(input: &[u32]) -> u32 {
    1
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &[u32]) -> u32 {
    1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let input_str1: &str = r"
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
"
        .trim();

        let input = input_generator(input_str1);
        assert_eq!(solve_part1(&input), 1);
    }

    #[test]
    fn part2_test() {
        let input_str2: &str = r"".trim();

        let input = input_generator(input_str2);
        assert_eq!(solve_part2(&input), 1);
    }
}
