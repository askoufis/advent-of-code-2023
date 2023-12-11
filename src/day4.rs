use nom::bytes::complete::tag;
use nom::character::complete::char;
use nom::multi::many1;
use nom::sequence::separated_pair;
use nom::{multi::separated_list1, IResult};

use std::collections::HashMap;

use crate::parsers::parse_usize;

#[derive(Clone)]
pub struct Card {
    id: usize,
    winning: Vec<usize>,
    nums: Vec<usize>,
}

impl Card {
    fn get_score(self: &Self) -> usize {
        let mut score = 0;
        self.nums.iter().for_each(|num| {
            if self.winning.contains(num) {
                if score == 0 {
                    score = 1;
                } else {
                    score *= 2
                }
            }
        });

        score
    }

    fn get_part2_score(self: &Self) -> usize {
        self.nums
            .iter()
            .filter(|num| self.winning.contains(num))
            .collect::<Vec<_>>()
            .len()
    }

    fn process(self: &Self, score_map: &mut HashMap<usize, usize>) -> usize {
        let score = self.get_part2_score();
        let won_card_ids = ((self.id + 1)..(self.id + 1 + score)).collect::<Vec<_>>();
        let won_cards_score: usize = won_card_ids
            .iter()
            .map(|card_id| score_map.get(card_id).unwrap_or(&1).to_owned())
            .sum();

        let score = won_cards_score + 1;
        score_map.insert(self.id, score);
        score
    }
}

fn parse_num_list(input: &str) -> IResult<&str, Vec<usize>> {
    separated_list1(many1(char(' ')), parse_usize)(input.trim())
}

fn parse_card_id(input: &str) -> IResult<&str, usize> {
    let (input, result) = separated_pair(tag("Card"), many1(char(' ')), parse_usize)(input)?;

    IResult::Ok((input, result.1))
}

fn parse_all_nums(input: &str) -> IResult<&str, (Vec<usize>, Vec<usize>)> {
    separated_pair(parse_num_list, tag(" | "), parse_num_list)(input)
}

fn parse_card(input: &str) -> IResult<&str, Card> {
    let (input, result) = separated_pair(parse_card_id, tag(": "), parse_all_nums)(input)?;

    IResult::Ok((
        input,
        Card {
            id: result.0,
            winning: result.1 .0,
            nums: result.1 .1,
        },
    ))
}

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Vec<Card> {
    let result = separated_list1(char('\n'), parse_card)(input);

    result.ok().expect("Bad parse").1
}

#[aoc(day4, part1)]
pub fn solve_part1(input: &[Card]) -> usize {
    input.iter().map(|card| card.get_score()).sum()
}

#[aoc(day4, part2)]
pub fn solve_part2(input: &[Card]) -> usize {
    let mut score_map: HashMap<usize, usize> = HashMap::new();

    input
        .iter()
        .rev()
        .map(|card| card.process(&mut score_map))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_num_list() {
        let input = " 1 48 83  6 17";

        let result = parse_num_list(input).ok().unwrap().1;
        assert_eq!(result, vec![1, 48, 83, 6, 17])
    }

    #[test]
    fn test_parse_card_id() {
        let input = "Card  32";

        let result = parse_card_id(input).ok().unwrap().1;
        assert_eq!(result, 32)
    }

    #[test]
    fn test_parse_all_nums() {
        let input = " 1 48 83  6 17 |  3 86  6 31 17  9 48 53";

        let result = parse_all_nums(input).ok().unwrap().1;
        assert_eq!(
            result,
            (vec![1, 48, 83, 6, 17], vec![3, 86, 6, 31, 17, 9, 48, 53])
        )
    }

    #[test]
    fn test_parse_card() {
        let input = "Card 3:  1 48 83  6 17 | 83 86  6 31 17  9 48 53";

        let result = parse_card(input).ok().unwrap().1;
        assert_eq!(result.id, 3);
        assert_eq!(result.winning, vec![1, 48, 83, 6, 17]);
        assert_eq!(result.nums, vec![83, 86, 6, 31, 17, 9, 48, 53])
    }

    #[test]
    fn part1_test() {
        let input_str1: &str = r"
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
"
        .trim();

        let input = input_generator(input_str1);
        assert_eq!(solve_part1(&input), 13);
    }

    #[test]
    fn part2_test() {
        let input_str2: &str = r"
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
"
        .trim();

        let input = input_generator(input_str2);
        assert_eq!(solve_part2(&input), 30);
    }
}
