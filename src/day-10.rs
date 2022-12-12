use std::{iter, str::FromStr};

use advent::input_lines;
use itertools::Itertools;

fn main() -> std::io::Result<()> {
    println!("Part1: {}", part1(input_lines("inputs/day-10.txt")?));
    println!("Part2:\n{}", part2(input_lines("inputs/day-10.txt")?));
    Ok(())
}

fn part1(input: impl Iterator<Item = String>) -> i64 {
    cycles(input)
        .filter_map(|(cycle, x)| {
            let cycle = cycle + 1;
            [20, 60, 100, 140, 180, 220]
                .contains(&cycle)
                .then_some(x * cycle)
        })
        .sum()
}

fn part2(input: impl Iterator<Item = String>) -> String {
    cycles(input)
        .chunks(40)
        .into_iter()
        .map(|lines| {
            lines
                .map(|(cycle, x)| match cycle % 40 {
                    c if (c - x).abs() <= 1 => "#",
                    _ => ".",
                })
                .collect::<String>()
        })
        .take(6)
        .join("\n")
}

fn cycles(input: impl Iterator<Item = String>) -> impl Iterator<Item = (i64, i64)> {
    iter::once(0)
        .chain(
            input
                .filter_map(|line| line.parse::<Instruction>().ok())
                .flat_map(|instruction| match instruction {
                    Instruction::Noop => vec![0],
                    Instruction::Addx(x) => vec![0, x],
                }),
        )
        .scan(1, |x, dx| {
            *x += dx;
            Some(*x)
        })
        .enumerate()
        .map(|(cycle, x)| (cycle as i64, x))
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Addx(i64),
    Noop,
}

impl FromStr for Instruction {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split_at(4) {
            ("noop", _) => Ok(Instruction::Noop),
            ("addx", n) => Ok(Instruction::Addx(
                n.trim().parse().map_err(|_| "Invalid number")?,
            )),
            _ => Err("Invalid instruction"),
        }
    }
}

#[cfg(test)]
mod tests {
    use advent::static_input_lines;
    use indoc::indoc;

    use super::*;

    const INPUT: &'static str = indoc! {"
        addx 15
        addx -11
        addx 6
        addx -3
        addx 5
        addx -1
        addx -8
        addx 13
        addx 4
        noop
        addx -1
        addx 5
        addx -1
        addx 5
        addx -1
        addx 5
        addx -1
        addx 5
        addx -1
        addx -35
        addx 1
        addx 24
        addx -19
        addx 1
        addx 16
        addx -11
        noop
        noop
        addx 21
        addx -15
        noop
        noop
        addx -3
        addx 9
        addx 1
        addx -3
        addx 8
        addx 1
        addx 5
        noop
        noop
        noop
        noop
        noop
        addx -36
        noop
        addx 1
        addx 7
        noop
        noop
        noop
        addx 2
        addx 6
        noop
        noop
        noop
        noop
        noop
        addx 1
        noop
        noop
        addx 7
        addx 1
        noop
        addx -13
        addx 13
        addx 7
        noop
        addx 1
        addx -33
        noop
        noop
        noop
        addx 2
        noop
        noop
        noop
        addx 8
        noop
        addx -1
        addx 2
        addx 1
        noop
        addx 17
        addx -9
        addx 1
        addx 1
        addx -3
        addx 11
        noop
        noop
        addx 1
        noop
        addx 1
        noop
        noop
        addx -13
        addx -19
        addx 1
        addx 3
        addx 26
        addx -30
        addx 12
        addx -1
        addx 3
        addx 1
        noop
        noop
        noop
        addx -9
        addx 18
        addx 1
        addx 2
        noop
        noop
        addx 9
        noop
        noop
        noop
        addx -1
        addx 2
        addx -37
        addx 1
        addx 3
        noop
        addx 15
        addx -21
        addx 22
        addx -6
        addx 1
        noop
        addx 2
        addx 1
        noop
        addx -10
        noop
        noop
        addx 20
        addx 1
        addx 2
        addx 2
        addx -6
        addx -11
        noop
        noop
        noop
    "};

    #[test]
    fn test_part1() {
        assert_eq!(part1(static_input_lines(INPUT)), 13140);
    }

    #[test]
    fn test_part2() {
        let output = indoc! {"
            ##..##..##..##..##..##..##..##..##..##..
            ###...###...###...###...###...###...###.
            ####....####....####....####....####....
            #####.....#####.....#####.....#####.....
            ######......######......######......####
            #######.......#######.......#######.....
        "};
        assert_eq!(part2(static_input_lines(INPUT)), output.trim());
    }
}
