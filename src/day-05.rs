use advent::read_input;
use itertools::Itertools;
use once_cell::sync::Lazy;
use regex::Regex;
use std::{collections::HashMap, iter::FromIterator, str::FromStr};

fn main() -> std::io::Result<()> {
    println!("Part1: {}", part1(read_input("inputs/day-05.txt")?));
    println!("Part2: {}", part2(read_input("inputs/day-05.txt")?));
    Ok(())
}

fn part1(mut input: impl Iterator<Item = String>) -> String {
    let mut stacks = Stacks::from_iter(input.by_ref().take_while(|line| !line.is_empty()));

    for command in input.filter_map(|command| Command::from_str(&command).ok()) {
        for _ in 1..=command.quantity {
            if let Some(item) = stacks.pop(command.src) {
                stacks.push(command.dst, item);
            }
        }
    }

    stacks
        .into_iter()
        .filter_map(|(_, items)| items.last().cloned())
        .collect()
}

fn part2(mut input: impl Iterator<Item = String>) -> String {
    let mut stacks = Stacks::from_iter(input.by_ref().take_while(|line| !line.is_empty()));

    for command in input.filter_map(|command| Command::from_str(&command).ok()) {
        if let Some(removed) = stacks.pop_stack(command.src, command.quantity) {
            stacks.push_stack(command.dst, removed);
        }
    }

    stacks
        .into_iter()
        .filter_map(|(_, items)| items.last().cloned())
        .collect()
}

struct Command {
    src: usize,
    dst: usize,
    quantity: usize,
}

static COMMAND_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap());

impl FromStr for Command {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let caps = COMMAND_REGEX.captures(s).ok_or("Invalid command")?;
        let amt = caps[1].parse().map_err(|_| "Invalid quantity")?;
        let src = caps[2].parse().map_err(|_| "Invalid src")?;
        let dst = caps[3].parse().map_err(|_| "Invalid dst")?;
        Ok(Command {
            src,
            dst,
            quantity: amt,
        })
    }
}

#[derive(Debug, Default)]
struct Stacks(HashMap<usize, Vec<String>>);

impl Stacks {
    fn push(&mut self, stack: usize, item: String) {
        if let Some(stack) = self.0.get_mut(&stack) {
            stack.push(item);
        }
    }

    fn push_stack(&mut self, stack: usize, mut items: Vec<String>) {
        if let Some(stack) = self.0.get_mut(&stack) {
            stack.append(&mut items);
        }
    }

    fn pop(&mut self, stack: usize) -> Option<String> {
        self.0.get_mut(&stack).and_then(|stack| stack.pop())
    }

    fn pop_stack(&mut self, stack: usize, size: usize) -> Option<Vec<String>> {
        self.0
            .get_mut(&stack)
            .map(|stack| stack.drain(stack.len() - size..).collect())
    }
}

impl IntoIterator for Stacks {
    type Item = (usize, Vec<String>);
    type IntoIter = Box<dyn Iterator<Item = Self::Item>>;

    fn into_iter(self) -> Self::IntoIter {
        Box::new(self.0.into_iter().sorted_by_key(|(id, _)| *id))
    }
}

static STACKS_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"\[([A-Z])\]").unwrap());
static STACKS_IDS_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"(\d+)").unwrap());

impl FromIterator<String> for Stacks {
    fn from_iter<T: IntoIterator<Item = String>>(iter: T) -> Self {
        let input = iter.into_iter();
        let (items, mut ids) = input.tee();

        let ids = ids
            .find(|line| !STACKS_REGEX.is_match(line))
            .map(|line| {
                STACKS_IDS_REGEX
                    .captures_iter(&line)
                    .filter_map(|cap| cap.get(1))
                    .map(|item| (item.start(), item.as_str().parse::<usize>().unwrap()))
                    .collect::<HashMap<_, _>>()
            })
            .unwrap_or_default();

        let items = items
            .take_while(|line| STACKS_REGEX.is_match(line))
            .flat_map(|line| {
                STACKS_REGEX
                    .captures_iter(&line)
                    .filter_map(|cap| cap.get(1))
                    .map(|item| (item.start(), item.as_str().to_owned()))
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
            .fold(
                HashMap::<usize, Vec<String>>::new(),
                |mut stacks, (col, item)| {
                    stacks.entry(ids[&col]).or_default().push(item);
                    stacks
                },
            );

        Stacks(items)
    }
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
