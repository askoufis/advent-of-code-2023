use std::cmp::Ordering;

use nom::bytes::complete::tag;
use nom::character::complete::{alphanumeric1, char};
use nom::{multi::separated_list1, IResult};

use crate::parsers::parse_usize;

#[derive(PartialOrd, PartialEq, Debug, Copy, Clone, Eq, Ord)]
enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    T,
    J,
    Q,
    K,
    A,
}

impl From<char> for Card {
    fn from(value: char) -> Self {
        return match value {
            '2' => Card::Two,
            '3' => Card::Three,
            '4' => Card::Four,
            '5' => Card::Five,
            '6' => Card::Six,
            '7' => Card::Seven,
            '8' => Card::Eight,
            '9' => Card::Nine,
            'T' => Card::T,
            'J' => Card::J,
            'Q' => Card::Q,
            'K' => Card::K,
            'A' => Card::A,
            _ => unreachable!(),
        };
    }
}

#[derive(PartialEq, PartialOrd, Eq, Ord, Debug, Copy, Clone)]
enum Type {
    HighCard,
    Pair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

fn get_cards_type(cards: &[Card]) -> Type {
    let mut counts = [0; 13];

    for card in cards {
        counts[*card as usize] += 1;
    }

    let mut pairs = 0;
    let mut threes = 0;
    let mut fours = 0;
    let mut fives = 0;

    for count in counts.iter() {
        match count {
            2 => pairs += 1,
            3 => threes += 1,
            4 => fours += 1,
            5 => fives += 1,
            _ => {}
        }
    }

    if fives == 1 {
        return Type::FiveOfAKind;
    }

    if fours == 1 {
        return Type::FourOfAKind;
    }

    if threes == 1 && pairs == 1 {
        return Type::FullHouse;
    }

    if threes == 1 {
        return Type::ThreeOfAKind;
    }

    if pairs == 2 {
        return Type::TwoPair;
    }

    if pairs == 1 {
        return Type::Pair;
    }

    return Type::HighCard;
}

#[derive(Debug)]
pub struct Hand {
    cards: Vec<Card>,
    bid: usize,
}

fn compare_cards(a: &[Card], b: &[Card]) -> Ordering {
    let mut result = Ordering::Equal;

    for (a, b) in a.iter().zip(b) {
        let comparison = a.cmp(b);
        if comparison != Ordering::Equal {
            result = comparison;
            break;
        }
    }

    result
}

fn compare_hands(a: &&Hand, b: &&Hand) -> Ordering {
    let a_type = get_cards_type(&a.cards);
    let b_type = get_cards_type(&b.cards);

    let comparison = a_type.cmp(&b_type);

    if comparison != Ordering::Equal {
        return comparison;
    }

    compare_cards(&a.cards, &b.cards)
}

fn parse_cards(input: &str) -> IResult<&str, Vec<Card>> {
    let (input, result) = alphanumeric1(input)?;

    let cards = result.chars().map(|c| Card::from(c)).collect::<Vec<Card>>();

    IResult::Ok((input, cards))
}

fn parse_hand(input: &str) -> IResult<&str, Hand> {
    let (input, cards) = parse_cards(input)?;
    let (input, _) = char(' ')(input)?;
    let (input, bid) = parse_usize(input)?;

    IResult::Ok((input, Hand { cards, bid }))
}

#[aoc_generator(day7)]
pub fn input_generator(input: &str) -> Vec<Hand> {
    separated_list1(tag("\n"), parse_hand)(input)
        .ok()
        .unwrap()
        .1
}

#[aoc(day7, part1)]
pub fn solve_part1(input: &[Hand]) -> usize {
    let mut sorted = vec![];
    for hand in input {
        sorted.push(hand);
    }

    sorted.sort_by(compare_hands);

    sorted
        .iter()
        .enumerate()
        .map(|(i, hand)| (i + 1) * hand.bid)
        .sum()
}

#[aoc(day7, part2)]
pub fn solve_part2(input: &[Hand]) -> usize {
    1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_cards_test() {
        let input = "32T3K";

        let result = parse_cards(input).ok().unwrap().1;
        assert_eq!(
            result,
            vec![Card::Three, Card::Two, Card::T, Card::Three, Card::K]
        )
    }

    #[test]
    fn parse_hand_test() {
        let input = "32T3K 765";

        let result = parse_hand(input).ok().unwrap().1;
        assert_eq!(
            result.cards,
            vec![Card::Three, Card::Two, Card::T, Card::Three, Card::K]
        );
        assert_eq!(result.bid, 765);
    }

    #[test]
    fn get_cards_type_pair_test() {
        let input = vec![Card::Three, Card::Two, Card::T, Card::Three, Card::K];

        let result = get_cards_type(&input);
        assert_eq!(result, Type::Pair);
    }

    #[test]
    fn get_cards_type_twopair_test() {
        let input = vec![Card::Three, Card::Two, Card::Two, Card::Three, Card::K];

        let result = get_cards_type(&input);
        assert_eq!(result, Type::TwoPair);
    }

    #[test]
    fn get_cards_type_threeofakind_test() {
        let input = vec![Card::Three, Card::Two, Card::Three, Card::Three, Card::K];

        let result = get_cards_type(&input);
        assert_eq!(result, Type::ThreeOfAKind);
    }

    #[test]
    fn get_cards_type_fullhouse_test() {
        let input = vec![Card::Three, Card::Two, Card::Three, Card::Three, Card::Two];

        let result = get_cards_type(&input);
        assert_eq!(result, Type::FullHouse);
    }

    #[test]
    fn get_cards_type_fourofakind_test() {
        let input = vec![
            Card::Three,
            Card::Two,
            Card::Three,
            Card::Three,
            Card::Three,
        ];

        let result = get_cards_type(&input);
        assert_eq!(result, Type::FourOfAKind);
    }

    #[test]
    fn get_cards_type_fiveofakind_test() {
        let input = vec![
            Card::Three,
            Card::Three,
            Card::Three,
            Card::Three,
            Card::Three,
        ];

        let result = get_cards_type(&input);
        assert_eq!(result, Type::FiveOfAKind);
    }

    #[test]
    fn part1_test() {
        let input_str1: &str = r"
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
"
        .trim();

        let input = input_generator(input_str1);
        assert_eq!(solve_part1(&input), 6440);
    }

    #[test]
    fn part2_test() {
        let input_str2: &str = r"
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
        "
        .trim();

        let input = input_generator(input_str2);
        assert_eq!(solve_part2(&input), 1);
    }
}
