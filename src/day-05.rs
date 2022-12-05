use advent::read_input;
use itertools::Itertools;
use regex::Regex;
use std::{collections::HashMap, str::FromStr};

fn main() -> std::io::Result<()> {
    println!("Part1: {}", part1(read_input("inputs/day-05.txt")?));
    println!("Part2: {}", part2(read_input("inputs/day-05.txt")?));
    Ok(())
}

fn part1(mut input: impl Iterator<Item = String>) -> String {
    let stack_input = input
        .by_ref()
        .take_while(|line| !line.is_empty())
        .collect::<Vec<String>>();

    let mut stacks = parse_stacks(stack_input);

    for command in input.filter_map(|command| Command::from_str(&command).ok()) {
        for _ in 1..=command.quantity {
            if let Some(item) = stacks
                .get_mut(&command.source)
                .and_then(|stack| stack.pop())
            {
                if let Some(to) = stacks.get_mut(&command.destination) {
                    to.push(item);
                };
            }
        }
    }

    stacks
        .into_iter()
        .sorted_by_key(|(id, _)| *id)
        .filter_map(|(_, items)| items.last().map(|item| item.to_owned()))
        .collect()
}

fn part2(mut input: impl Iterator<Item = String>) -> String {
    let stack_input = input
        .by_ref()
        .take_while(|line| !line.is_empty())
        .collect::<Vec<String>>();

    let mut stacks = parse_stacks(stack_input);

    for command in input.filter_map(|command| Command::from_str(&command).ok()) {
        if let Some(mut removed) = stacks.get_mut(&command.source).map(|stack| {
            stack
                .drain(stack.len() - command.quantity..)
                .collect::<Vec<_>>()
        }) {
            if let Some(stack) = stacks.get_mut(&command.destination) {
                stack.append(&mut removed);
            }
        }
    }

    stacks
        .into_iter()
        .sorted_by_key(|(id, _)| *id)
        .filter_map(|(_, items)| items.last().map(|item| item.to_owned()))
        .collect()
}

struct Command {
    source: usize,
    destination: usize,
    quantity: usize,
}

impl FromStr for Command {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();

        let caps = re.captures(s).ok_or("Invalid command")?;

        let quantity = caps
            .get(1)
            .and_then(|q| q.as_str().parse::<usize>().ok())
            .ok_or("Invalid command quantity")?;

        let source = caps
            .get(2)
            .and_then(|q| q.as_str().parse::<usize>().ok())
            .ok_or("Invalid command source")?;

        let destination = caps
            .get(3)
            .and_then(|q| q.as_str().parse::<usize>().ok())
            .ok_or("Invalid command destination")?;

        Ok(Command {
            source,
            destination,
            quantity,
        })
    }
}

fn parse_stacks(input: Vec<String>) -> HashMap<usize, Vec<String>> {
    let re = Regex::new(r"\[([A-Z])\]").unwrap();

    let (ids, items) = input.split_last().unwrap();

    let ids = Regex::new(r"(\d+)")
        .unwrap()
        .captures_iter(ids)
        .filter_map(|cap| cap.get(1))
        .map(|item| (item.start(), item.as_str().parse::<usize>().unwrap()))
        .collect::<HashMap<_, _>>();

    #[allow(clippy::needless_collect)]
    let items = items
        .iter()
        .flat_map(|line| re.captures_iter(line).filter_map(|cap| cap.get(1)))
        .map(|item| (item.start(), item.as_str()))
        .collect::<Vec<_>>();

    items
        .into_iter()
        .rev()
        .fold(HashMap::new(), |mut stacks, (col, item)| {
            stacks.entry(ids[&col]).or_default().push(item.to_string());
            stacks
        })
}

#[cfg(test)]
mod tests {
    use advent::static_input;
    use indoc::indoc;

    use super::*;

    const INPUT: &'static str = indoc! {"
            [D]    
        [N] [C]    
        [Z] [M] [P]
         1   2   3 

        move 1 from 2 to 1
        move 3 from 1 to 3
        move 2 from 2 to 1
        move 1 from 1 to 2
    "};

    #[test]
    fn test_part1() {
        assert_eq!(part1(static_input(INPUT)), "CMZ");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(static_input(INPUT)), "MCD");
    }
}
