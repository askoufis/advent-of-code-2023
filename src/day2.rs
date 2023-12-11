use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::char;
use nom::multi::separated_list1;
use nom::{character::complete::digit1, sequence::separated_pair, IResult};

use crate::parsers::parse_usize;

#[derive(Clone, Copy, Debug, PartialEq)]
enum Color {
    Red(usize),
    Green(usize),
    Blue(usize),
}

impl Color {
    fn value(self: &Self) -> usize {
        match self {
            Color::Red(val) => *val,
            Color::Green(val) => *val,
            Color::Blue(val) => *val,
        }
    }
}

impl From<(&str, usize)> for Color {
    fn from(value: (&str, usize)) -> Self {
        let (color, number) = value;

        match color {
            "red" => Self::Red(number),
            "green" => Self::Green(number),
            "blue" => Self::Blue(number),

            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
struct Round {
    red: Color,
    green: Color,
    blue: Color,
}

impl Round {
    fn is_valid(self: &Self, cube_limits: (usize, usize, usize)) -> bool {
        let (r, g, b) = cube_limits;

        self.red.value() <= r && self.green.value() <= g && self.blue.value() <= b
    }
}

pub struct Game {
    id: usize,
    rounds: Vec<Round>,
}

impl Game {
    fn is_valid(self: &Self, cube_limits: (usize, usize, usize)) -> bool {
        self.rounds.iter().all(|round| round.is_valid(cube_limits))
    }

    fn min_set(self: &Self) -> (usize, usize, usize) {
        let mut r = 0;
        let mut g = 0;
        let mut b = 0;

        self.rounds.iter().for_each(|round| {
            let r_value = round.red.value();
            let g_value = round.green.value();
            let b_value = round.blue.value();

            if r_value > r {
                r = r_value;
            }

            if g_value > g {
                g = g_value;
            }

            if b_value > b {
                b = b_value;
            }
        });

        (r, g, b)
    }
}

fn min_set_product(input: (usize, usize, usize)) -> usize {
    let (a, b, c) = input;

    a * b * c
}

fn parse_color(input: &str) -> IResult<&str, Color> {
    let (input, (value_str, color_str)) = separated_pair(
        digit1,
        char(' '),
        alt((tag("red"), tag("green"), tag("blue"))),
    )(input)?;

    let (_, value) = parse_usize(value_str)?;

    IResult::Ok((input, Color::from((color_str, value))))
}

fn parse_round(input: &str) -> IResult<&str, Round> {
    let (input, result) = separated_list1(tag(", "), parse_color)(input)?;

    let red = result
        .iter()
        .find(|&&c| match c {
            Color::Red(_) => true,
            _ => false,
        })
        .unwrap_or(&Color::Red(0))
        .clone();

    let green = result
        .iter()
        .find(|&&c| match c {
            Color::Green(_) => true,
            _ => false,
        })
        .unwrap_or(&Color::Green(0))
        .clone();

    let blue = result
        .iter()
        .find(|&&c| match c {
            Color::Blue(_) => true,
            _ => false,
        })
        .unwrap_or(&Color::Blue(0))
        .clone();

    IResult::Ok((input, Round { red, green, blue }))
}

fn parse_round_list(input: &str) -> IResult<&str, Vec<Round>> {
    separated_list1(tag("; "), parse_round)(input)
}

fn parse_game_id(input: &str) -> IResult<&str, usize> {
    let (input, _) = tag("Game ")(input)?;

    parse_usize(input)
}

fn parse_game(input: &str) -> IResult<&str, Game> {
    let (input, (game_id, round_list)) =
        separated_pair(parse_game_id, tag(": "), parse_round_list)(input)?;

    IResult::Ok((
        input,
        Game {
            id: game_id,
            rounds: round_list,
        },
    ))
}

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<Game> {
    let result = separated_list1(char('\n'), parse_game)(input);

    result.ok().expect("Bad parse").1
}

#[aoc(day2, part1)]
pub fn solve_part1(input: &[Game]) -> usize {
    input
        .iter()
        .filter_map(|game| {
            if game.is_valid((12, 13, 14)) {
                return Some(game.id);
            }
            return None;
        })
        .sum()
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &[Game]) -> usize {
    input
        .iter()
        .map(|game| min_set_product(game.min_set()))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_color() {
        let input = "4 red";
        let result = parse_color(input).ok().unwrap().1;

        assert_eq!(result, Color::Red(4))
    }

    #[test]
    fn test_parse_game_id() {
        let input = "Game 3";
        let result = parse_game_id(input).ok().unwrap().1;

        assert_eq!(result, 3)
    }

    #[test]
    fn test_parse_round() {
        let input = "4 red, 3 green, 2 blue";
        let result = parse_round(input).ok().unwrap().1;

        assert_eq!(result.red, Color::Red(4));
        assert_eq!(result.green, Color::Green(3));
        assert_eq!(result.blue, Color::Blue(2));
    }

    #[test]
    fn test_parse_round_list() {
        let input = "4 red, 3 green, 2 blue; 3 blue, 4 green";
        let result = parse_round_list(input).ok().unwrap().1;

        assert_eq!(result[0].red, Color::Red(4));
        assert_eq!(result[0].green, Color::Green(3));
        assert_eq!(result[0].blue, Color::Blue(2));

        assert_eq!(result[1].red, Color::Red(0));
        assert_eq!(result[1].green, Color::Green(4));
        assert_eq!(result[1].blue, Color::Blue(3));
    }

    #[test]
    fn test_parse_game() {
        let input = "Game 3: 4 red, 3 green, 2 blue; 3 blue, 4 green";
        let result = parse_game(input).ok().unwrap().1;

        assert_eq!(result.id, 3);

        assert_eq!(result.rounds[0].red, Color::Red(4));
        assert_eq!(result.rounds[0].green, Color::Green(3));
        assert_eq!(result.rounds[0].blue, Color::Blue(2));

        assert_eq!(result.rounds[1].red, Color::Red(0));
        assert_eq!(result.rounds[1].green, Color::Green(4));
        assert_eq!(result.rounds[1].blue, Color::Blue(3));
    }

    #[test]
    fn test_min_set() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let result = parse_game(input).ok().unwrap().1.min_set();

        assert_eq!(result.0, 4);
        assert_eq!(result.1, 2);
        assert_eq!(result.2, 6);
    }

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
        assert_eq!(solve_part1(&input), 8);
    }

    #[test]
    fn part2_test() {
        let input_str2: &str = r"
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
"
        .trim();

        let input = input_generator(input_str2);
        assert_eq!(solve_part2(&input), 2286);
    }
}
