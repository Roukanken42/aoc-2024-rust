use advent_of_code::utils::{end_of_file, Parsable};
use itertools::Itertools;
use nom::bytes::complete::tag;
use nom::character::complete::line_ending;
use nom::combinator::map;
use nom::multi::{many1, separated_list1};
use nom::sequence::separated_pair;
use nom::IResult;
use std::collections::{HashMap, HashSet};

advent_of_code::solution!(5);

#[derive(Debug, PartialEq)]
struct Rule {
    lesser: u32,
    greater: u32,
}

impl Parsable<'_> for Rule {
    fn parse(input: &str) -> IResult<&str, Self> {
        let pair = separated_pair(u32::parse, tag("|"), u32::parse);
        map(pair, |(lesser, greater)| Rule { lesser, greater })(input)
    }
}

struct Input {
    rules: Vec<Rule>,
    updates: Vec<Vec<u32>>,
}

impl Parsable<'_> for Input {
    fn parse(input: &'_ str) -> IResult<&'_ str, Self> {
        let (input, rules) = separated_list1(line_ending, Rule::parse)(input)?;
        let (input, _) = many1(line_ending)(input)?;

        let parse_update = separated_list1(tag(","), u32::parse);
        let (input, updates) = separated_list1(line_ending, parse_update)(input)?;

        let (input, _) = end_of_file(input)?;

        Ok((input, Input { rules, updates }))
    }
}

fn construct_greater_than_tree(rules: &[Rule]) -> HashMap<u32, Vec<u32>> {
    let mut tree = HashMap::new();

    for rule in rules {
        tree.entry(rule.greater)
            .or_insert_with(Vec::new)
            .push(rule.lesser);
    }

    tree
}

fn determine_values(
    greater_than_tree: &HashMap<u32, Vec<u32>>,
    keys: &HashSet<u32>,
) -> HashMap<u32, u32> {
    let mut values = HashMap::new();

    fn determine_value(
        key: u32,
        greater_than_tree: &HashMap<u32, Vec<u32>>,
        values: &mut HashMap<u32, u32>,
        keys: &HashSet<u32>,
    ) -> u32 {
        if let Some(&val) = values.get(&key) {
            return val;
        }

        let binding = vec![];
        let lesser_keys = greater_than_tree.get(&key).unwrap_or(&binding);
        let value = lesser_keys
            .iter()
            .filter(|&k| keys.contains(k))
            .map(|&k| determine_value(k, greater_than_tree, values, keys))
            .max()
            .unwrap_or(0)
            + 1;

        values.insert(key, value);

        value
    }

    for &key in greater_than_tree.keys() {
        determine_value(key, &greater_than_tree, &mut values, &keys);
    }

    values
}

pub fn part_one(input: &str) -> Option<u32> {
    let (_, input) = Input::parse(input).unwrap();

    let greater_than_tree = construct_greater_than_tree(&input.rules);

    Some(
        input
            .updates
            .into_iter()
            .filter(|update| {
                let values =
                    determine_values(&greater_than_tree, &update.iter().copied().collect());
                update.is_sorted_by_key(|&key| values.get(&key).unwrap_or(&0))
            })
            .map(|update| update[update.len() / 2])
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let (_, input) = Input::parse(input).unwrap();

    let greater_than_tree = construct_greater_than_tree(&input.rules);

    Some(
        input
            .updates
            .into_iter()
            .filter_map(|update| {
                let values =
                    determine_values(&greater_than_tree, &update.iter().copied().collect());

                if update.is_sorted_by_key(|&key| values.get(&key).unwrap_or(&0)) {
                    return None;
                } else {
                    Some(
                        update
                            .into_iter()
                            .sorted_by_key(|&key| values.get(&key).unwrap_or(&0))
                            .collect::<Vec<_>>(),
                    )
                }
            })
            .map(|update| update[update.len() / 2])
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input = advent_of_code::template::read_file("examples", DAY);
        let result = Input::parse(&input);

        let (str, input) = result.unwrap();

        assert!(str.is_empty());
        assert_eq!(
            input.rules[0],
            Rule {
                lesser: 47,
                greater: 53
            }
        );
        assert_eq!(
            input.rules.last(),
            Some(&Rule {
                lesser: 53,
                greater: 13
            })
        );

        assert_eq!(input.updates[0], vec![75, 47, 61, 53, 29]);
    }

    #[test]
    fn test_construct_greater_than_tree() {
        let input = advent_of_code::template::read_file("examples", DAY);
        let (_, input) = Input::parse(&input).unwrap();

        let result = construct_greater_than_tree(&input.rules);

        assert_eq!(result.get(&13), Some(&vec![97, 61, 29, 47, 75, 53]));
        assert_eq!(result.get(&61), Some(&vec![97, 47, 75]));
    }

    #[test]
    fn test_determine_values() {
        let input = advent_of_code::template::read_file("examples", DAY);
        let (_, input) = Input::parse(&input).unwrap();

        let greater_than_tree = construct_greater_than_tree(&input.rules);
        let result = determine_values(&greater_than_tree, &HashSet::from([75, 47, 61, 53, 29]));

        assert_eq!(
            result,
            HashMap::from([(61, 3), (53, 4), (13, 6), (29, 5), (47, 2), (75, 1)])
        );
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
