use std::{collections::HashSet, ops::RangeInclusive};

use advent::input_lines;

fn main() -> std::io::Result<()> {
    println!("Part1: {}", part1(input_lines("inputs/day-04.txt")?));
    println!("Part2: {}", part2(input_lines("inputs/day-04.txt")?));
    Ok(())
}

fn part1(input: impl Iterator<Item = String>) -> usize {
    input
        .filter(|line| !line.is_empty())
        .filter_map(|line| parse_pair(&line))
        .filter(|(e1, e2)| {
            e1.intersection(e2).count() == e1.len() || e2.intersection(e1).count() == e2.len()
        })
        .count()
}

fn part2(input: impl Iterator<Item = String>) -> usize {
    input
        .filter(|line| !line.is_empty())
        .filter_map(|line| parse_pair(&line))
        .filter(|(e1, e2)| e1.intersection(e2).next().is_some())
        .count()
}

fn parse_pair(input: &str) -> Option<(HashSet<usize>, HashSet<usize>)> {
    let (e1, e2) = input.split_once(',')?;
    Some((
        parse_assignments(e1)?.collect(),
        parse_assignments(e2)?.collect(),
    ))
}

fn parse_assignments(input: &str) -> Option<RangeInclusive<usize>> {
    let (start, end) = input.split_once('-')?;
    Some(start.parse().ok()?..=end.parse().ok()?)
}

#[cfg(test)]
mod tests {
    use advent::static_input_lines;
    use indoc::indoc;

    use super::*;

    const INPUT: &'static str = indoc! {"
        2-4,6-8
        2-3,4-5
        5-7,7-9
        2-8,3-7
        6-6,4-6
        2-6,4-8
    "};

    #[test]
    fn test_part1() {
        assert_eq!(part1(static_input_lines(INPUT)), 2);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(static_input_lines(INPUT)), 4);
    }
}
