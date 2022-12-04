use advent::read_input;
use itertools::Itertools;

fn main() -> std::io::Result<()> {
    println!("Part1: {}", part1(read_input("inputs/day-01.txt")?));
    println!("Part2: {}", part2(read_input("inputs/day-01.txt")?));
    Ok(())
}

fn part1(input: impl Iterator<Item = String>) -> usize {
    input
        .group_by(|line| line.is_empty())
        .into_iter()
        .map(|(_, lines)| lines)
        .map(|lines| {
            lines
                .filter_map(|cal| cal.parse::<usize>().ok())
                .sum::<usize>()
        })
        .max()
        .unwrap_or_default()
}

fn part2(input: impl Iterator<Item = String>) -> usize {
    input
        .group_by(|line| line.is_empty())
        .into_iter()
        .map(|(_, lines)| lines)
        .map(|lines| {
            lines
                .filter_map(|cal| cal.parse::<usize>().ok())
                .sum::<usize>()
        })
        .fold(Vec::with_capacity(3), |mut top, elf| {
            top.push(elf);
            top.sort();
            top.reverse();
            top.resize(3, 0);
            top
        })
        .into_iter()
        .sum()
}

#[cfg(test)]
mod tests {
    use advent::static_input;
    use indoc::indoc;

    use super::*;

    const INPUT: &'static str = indoc! {"
        1000
        2000
        3000

        4000

        5000
        6000

        7000
        8000
        9000

        10000
    "};

    #[test]
    fn test_part1() {
        assert_eq!(part1(static_input(INPUT)), 24000);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(static_input(INPUT)), 45000);
    }
}
