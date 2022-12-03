use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use once_cell::sync::Lazy;

static TYPES: Lazy<HashMap<char, usize>> = Lazy::new(|| {
    "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
        .chars()
        .zip(1..=52)
        .collect()
});

fn main() {
    let input = include_str!("../inputs/day-03.txt");
    println!("{}", part1(input));
    println!("{}", part2(input));
}

fn lines(input: &str) -> impl Iterator<Item = &str> {
    input
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
}

fn part1(input: &str) -> usize {
    lines(input)
        .map(|line| line.split_at(line.len() / 2))
        .map(|(c1, c2)| {
            c1.chars()
                .filter(|c| c2.contains(*c))
                .collect::<HashSet<char>>()
        })
        .map(|items| {
            items
                .into_iter()
                .filter_map(|item| TYPES.get(&item))
                .sum::<usize>()
        })
        .sum()
}

fn part2(input: &str) -> usize {
    lines(input)
        .chunks(3)
        .into_iter()
        .filter_map(|group| {
            group
                .map(|line| line.chars().collect::<HashSet<char>>())
                .reduce(|group, line| group.intersection(&line).cloned().collect())
        })
        .map(|items| {
            items
                .into_iter()
                .filter_map(|item| TYPES.get(&item))
                .sum::<usize>()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_01() {
        let input = "
            vJrwpWtwJgWrhcsFMMfFFhFp
            jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
            PmmdzqPrVvPwwTWBwg
            wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
            ttgJtRGJQctTZtZT
            CrZsJsPPZsGzwwsLwLmpwMDw
        ";

        assert_eq!(part1(input), 157);
    }

    #[test]
    fn test_part_02() {
        let input = "
            vJrwpWtwJgWrhcsFMMfFFhFp
            jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
            PmmdzqPrVvPwwTWBwg
            wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
            ttgJtRGJQctTZtZT
            CrZsJsPPZsGzwwsLwLmpwMDw
        ";

        assert_eq!(part2(input), 70);
    }
}
