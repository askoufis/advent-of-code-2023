use nom::bytes::complete::tag;
use nom::character::complete::char;
use nom::multi::many1;
use nom::{multi::separated_list1, IResult};

use crate::parsers::parse_usize;

fn parse_time(input: &str) -> IResult<&str, Vec<usize>> {
    let (input, _) = tag("Time:")(input)?;
    let (input, _) = many1(char(' '))(input)?;

    separated_list1(many1(char(' ')), parse_usize)(input)
}

fn parse_distance(input: &str) -> IResult<&str, Vec<usize>> {
    let (input, _) = tag("Distance:")(input)?;
    let (input, _) = many1(char(' '))(input)?;

    separated_list1(many1(char(' ')), parse_usize)(input)
}

// Smaller root is .0
fn quadratic_roots(a: i128, b: i128, c: i128) -> (f64, f64) {
    let x0 = ((-1 * b) as f64 + f64::sqrt((b.pow(2) - (4 * a * c)) as f64)) as f64 / (2 * a) as f64;
    let x1 = ((-1 * b) as f64 - f64::sqrt((b.pow(2) - (4 * a * c)) as f64)) as f64 / (2 * a) as f64;

    if x0 < x1 {
        return (x0, x1);
    }

    return (x1, x0);
}

pub struct Races {
    time: Vec<usize>,
    distance: Vec<usize>,
}

fn parse_races(input: &str) -> IResult<&str, Races> {
    let (input, time) = parse_time(input)?;
    let (input, _) = char('\n')(input)?;
    let (input, distance) = parse_distance(input)?;

    IResult::Ok((input, Races { time, distance }))
}

#[aoc_generator(day6)]
pub fn input_generator(input: &str) -> Races {
    parse_races(input).ok().unwrap().1
}
#[aoc(day6, part1)]

pub fn solve_part1(input: &Races) -> f64 {
    let a = -1;

    let mut ways = 1.0;

    for index in 0..input.time.len() {
        let time = input.time[index];
        let distance = input.distance[index];

        let b = time as i128;
        let c = -1 * distance as i128;

        let (x0, x1) = quadratic_roots(a, b, c);

        let min = x0.floor() + 1.0;
        let max = x1.ceil() - 1.0;

        let _ways = ((max - min) + 1.0).round();
        ways *= _ways;
    }

    ways
}

#[aoc(day6, part2)]
pub fn solve_part2(input: &Races) -> f64 {
    let a = -1;

    let time = input.time.iter().map(|t| t.to_string()).collect::<String>();
    let distance = input
        .distance
        .iter()
        .map(|t| t.to_string())
        .collect::<String>();

    let b = time.parse::<i128>().unwrap();
    let c = -1 * distance.parse::<i128>().unwrap();

    let (x0, x1) = quadratic_roots(a, b, c);

    let min = x0.floor() + 1.0;
    let max = x1.ceil() - 1.0;

    let _ways = ((max - min) + 1.0).round();

    _ways
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_time_test() {
        let input = "Time:      7  15   30";

        let result = parse_time(input).ok().unwrap().1;
        assert_eq!(result, vec![7, 15, 30]);
    }

    #[test]
    fn parse_distance_test() {
        let input = "Distance:  9  40  200";

        let result = parse_distance(input).ok().unwrap().1;
        assert_eq!(result, vec![9, 40, 200]);
    }

    #[test]
    fn parse_races_test() {
        let input = r"
Time:      7  15   30
Distance:  9  40  200
"
        .trim();

        let result = parse_races(input).ok().unwrap().1;
        assert_eq!(result.time, vec![7, 15, 30]);
        assert_eq!(result.distance, vec![9, 40, 200]);
    }

    #[test]
    fn part1_test() {
        let input_str1: &str = r"
Time:      7  15   30
Distance:  9  40  200
        "
        .trim();

        let input = input_generator(input_str1);
        assert_eq!(solve_part1(&input), 288.0);
    }

    #[test]
    fn part2_test() {
        let input_str2: &str = r"
Time:      7  15   30
Distance:  9  40  200
        "
        .trim();

        let input = input_generator(input_str2);
        assert_eq!(solve_part2(&input), 71503.0);
    }
}
