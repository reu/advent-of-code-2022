use std::{
    collections::{HashMap, HashSet},
    fs,
    io::{self, BufRead, BufReader},
    path::Path,
};

use itertools::Itertools;
use once_cell::sync::Lazy;

static TYPES: Lazy<HashMap<char, usize>> = Lazy::new(|| {
    "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
        .chars()
        .zip(1..=52)
        .collect()
});

fn main() -> io::Result<()> {
    println!("Part1: {}", part1(read_input("inputs/day-03.txt")?));
    println!("Part2: {}", part2(read_input("inputs/day-03.txt")?));
    Ok(())
}

fn read_input(path: impl AsRef<Path>) -> io::Result<impl Iterator<Item = String>> {
    Ok(BufReader::new(fs::File::open(path)?)
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| line.trim().to_owned())
        .filter(|line| !line.is_empty()))
}

fn part1(input: impl Iterator<Item = String>) -> usize {
    input
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

    fn test_input(input: &'static str) -> impl Iterator<Item = String> {
        input
            .lines()
            .map(|line| line.trim())
            .filter(|line| !line.is_empty())
            .map(|line| line.to_owned())
    }

    #[test]
    fn test_part1() {
        let input = "
            vJrwpWtwJgWrhcsFMMfFFhFp
            jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
            PmmdzqPrVvPwwTWBwg
            wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
            ttgJtRGJQctTZtZT
            CrZsJsPPZsGzwwsLwLmpwMDw
        ";

        assert_eq!(part1(test_input(input)), 157);
    }

    #[test]
    fn test_part2() {
        let input = "
            vJrwpWtwJgWrhcsFMMfFFhFp
            jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
            PmmdzqPrVvPwwTWBwg
            wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
            ttgJtRGJQctTZtZT
            CrZsJsPPZsGzwwsLwLmpwMDw
        ";

        assert_eq!(part2(test_input(input)), 70);
    }
}
