use nom::bytes::complete::tag;
use nom::character::complete::{char, not_line_ending};
use nom::{multi::separated_list1, IResult};
use std::ops::Range;

use crate::parsers::parse_usize;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
// dest, source, length
struct Map(usize, usize, usize);

impl Map {
    fn convert(self: &Self, seed: &usize) -> usize {
        // Difference between dest and source
        let diff = isize::try_from(self.0).unwrap() - isize::try_from(self.1).unwrap();
        usize::try_from(isize::try_from(*seed).unwrap() + diff).unwrap()
    }

    // Create a range of the source for the map
    fn to_range(self: &Self) -> Range<usize> {
        self.1..(self.1 + self.2)
    }
}

pub struct Almanac {
    seeds: Vec<usize>,
    map_list: Vec<Vec<Map>>,
}

impl Almanac {
    fn convert_seed(self: &Self, seed: usize) -> usize {
        let mut converted = seed;

        for list in self.map_list.iter() {
            let found_map = list.iter().find(|map| map.to_range().contains(&converted));

            if let Some(map) = found_map {
                converted = map.convert(&converted);
            }
        }

        converted
    }

    fn convert_seed_using_map_list(self: &Self, seed: usize, index: usize) -> usize {
        let found_map = self.map_list[index]
            .iter()
            .find(|map| map.to_range().contains(&seed));

        if let Some(map) = found_map {
            return map.convert(&seed);
        }

        seed
    }

    fn convert_seed_range(self: &Self, range: Range<usize>) -> Vec<Range<usize>> {
        let mut converted = vec![range];

        for (list_index, list) in self.map_list.iter().enumerate() {
            let mut converted_ranges = vec![];

            for r in converted.clone().iter() {
                let mut valid_map_bounds = list
                    .iter()
                    .flat_map(|m| {
                        let range = m.to_range();
                        [range.start, range.end]
                    })
                    .filter(|b| b > &r.start && b < &r.end)
                    .collect::<Vec<_>>();
                valid_map_bounds.push(r.start);
                valid_map_bounds.push(r.end);
                valid_map_bounds.sort_unstable();
                valid_map_bounds.dedup();

                let mut valid_map_ranges = valid_map_bounds
                    .windows(2)
                    .map(|w| w[0]..w[1])
                    .collect::<Vec<Range<usize>>>();

                for ran in valid_map_ranges.iter_mut() {
                    let converted_start = self.convert_seed_using_map_list(ran.start, list_index);
                    *ran = converted_start..(converted_start + (ran.len()));
                }

                converted_ranges.append(&mut valid_map_ranges);
            }

            converted = converted_ranges;
        }

        converted
    }
}

fn parse_seeds(input: &str) -> IResult<&str, Vec<usize>> {
    let (input, _) = tag("seeds: ")(input)?;

    separated_list1(tag(" "), parse_usize)(input)
}

fn parse_map(input: &str) -> IResult<&str, Map> {
    let (input, result) = separated_list1(char(' '), parse_usize)(input)?;

    IResult::Ok((input, Map(result[0], result[1], result[2])))
}

fn parse_map_list(input: &str) -> IResult<&str, Vec<Map>> {
    separated_list1(char('\n'), parse_map)(input)
}

fn parse_map_list_with_heading(input: &str) -> IResult<&str, Vec<Map>> {
    let (input, _) = not_line_ending(input)?;
    let (input, _) = char('\n')(input)?;

    parse_map_list(input)
}

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> Almanac {
    let sections = input.split("\n\n").collect::<Vec<_>>();

    let seeds = parse_seeds(sections[0]).ok().unwrap().1;

    let map_list = (1..sections.len())
        .map(|i| {
            let section = sections[i];
            parse_map_list_with_heading(section).ok().unwrap().1
        })
        .collect();

    Almanac { seeds, map_list }
}

#[aoc(day5, part1)]
pub fn solve_part1(input: &Almanac) -> usize {
    input
        .seeds
        .iter()
        .map(|seed| input.convert_seed(*seed))
        .min()
        .unwrap_or(0)
}

#[aoc(day5, part2)]
pub fn solve_part2(input: &Almanac) -> usize {
    let seed_ranges: Vec<Range<usize>> = input
        .seeds
        .chunks(2)
        .map(|chunk| chunk[0]..(chunk[0] + chunk[1]))
        .collect();

    seed_ranges
        .iter()
        .flat_map(|r| input.convert_seed_range(r.clone()))
        .flat_map(|r| [r.start, r.end])
        .min()
        .expect("Woops")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_seeds_test() {
        let input = "seeds: 1234 7327 27190837";

        let result = parse_seeds(input).ok().unwrap().1;
        assert_eq!(result, vec![1234, 7327, 27190837])
    }

    #[test]
    fn parse_map_test() {
        let input = "3224558845 3632370674 5378086";

        let result = parse_map(input).ok().unwrap().1;
        assert_eq!(result, Map(3224558845, 3632370674, 5378086))
    }

    #[test]
    fn parse_map_list_with_heading_test() {
        let input = r"
seed-to-soil map:
11 22 33
44 55 66
"
        .trim();

        let result = parse_map_list_with_heading(input).ok().unwrap().1;
        assert_eq!(result, vec![Map(11, 22, 33), Map(44, 55, 66)])
    }

    #[test]
    fn part1_test() {
        let input_str1: &str = r"
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
        "
        .trim();

        let input = input_generator(input_str1);
        assert_eq!(solve_part1(&input), 35);
    }

    #[test]
    fn part2_test() {
        let input_str2: &str = r"
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
        "
        .trim();

        let input = input_generator(input_str2);
        assert_eq!(solve_part2(&input), 46);
    }
}
