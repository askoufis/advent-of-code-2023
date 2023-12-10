const BASE_10: u32 = 10;
#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<Vec<u32>> {
    // Remove all the replaces to get the correct input for part 1
    input
        .replace("one", "o1e")
        .replace("two", "t2o")
        .replace("three", "t3e")
        .replace("four", "f4r")
        .replace("five", "f5e")
        .replace("six", "s6x")
        .replace("seven", "s7n")
        .replace("eight", "e8t")
        .replace("nine", "n9e")
        .lines()
        .map(|l| {
            l.chars()
                .filter_map(|c| {
                    if c.is_numeric() {
                        return c.to_digit(BASE_10);
                    }
                    return None;
                })
                .collect()
        })
        .collect()
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &[Vec<u32>]) -> u32 {
    input
        .into_iter()
        .map(|l| {
            let len = l.len();
            l[0] * 10 + l[len - 1]
        })
        .sum()
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &[Vec<u32>]) -> u32 {
    input
        .into_iter()
        .map(|l| {
            let len = l.len();
            l[0] * 10 + l[len - 1]
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let input_str1: &str = r"
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
"
        .trim();

        let input = input_generator(input_str1);
        assert_eq!(solve_part1(&input), 142);
    }

    #[test]
    fn part2_test() {
        let input_str2: &str = r"
two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
"
        .trim();

        let input = input_generator(input_str2);
        assert_eq!(solve_part2(&input), 281);
    }
}
