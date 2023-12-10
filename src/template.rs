#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<usize> {
    vec![1]
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &[u32]) -> usize {
    1
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &[u32]) -> usize {
    1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let input_str1: &str = r"".trim();

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
