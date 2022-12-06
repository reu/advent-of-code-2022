use std::collections::{HashMap, HashSet};

use advent::input_lines;
use itertools::Itertools;
use once_cell::sync::Lazy;

static TYPES: Lazy<HashMap<char, usize>> = Lazy::new(|| {
    "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
        .chars()
        .zip(1..=52)
        .collect()
});

fn main() -> std::io::Result<()> {
    println!("Part1: {}", part1(input_lines("inputs/day-03.txt")?));
    println!("Part2: {}", part2(input_lines("inputs/day-03.txt")?));
    Ok(())
}

fn part1(input: impl Iterator<Item = String>) -> usize {
    input
        .filter(|line| !line.is_empty())
        .map(|line| {
            let (c1, c2) = line.split_at(line.len() / 2);
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

fn part2(input: impl Iterator<Item = String>) -> usize {
    input
        .filter(|line| !line.is_empty())
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
    use advent::static_input_lines;
    use indoc::indoc;

    use super::*;

    const INPUT: &'static str = indoc! {"
        vJrwpWtwJgWrhcsFMMfFFhFp
        jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
        PmmdzqPrVvPwwTWBwg
        wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
        ttgJtRGJQctTZtZT
        CrZsJsPPZsGzwwsLwLmpwMDw
    "};

    #[test]
    fn test_part1() {
        assert_eq!(part1(static_input_lines(INPUT)), 157);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(static_input_lines(INPUT)), 70);
    }
}
