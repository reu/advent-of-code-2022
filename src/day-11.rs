use std::{
    collections::{HashMap, VecDeque},
    str::FromStr, cmp::Reverse,
};

use advent::input_lines;
use itertools::Itertools;

fn main() -> std::io::Result<()> {
    println!("Part1: {}", part1(input_lines("inputs/day-11.txt")?));
    // println!("Part2: {}", part2(input_lines("inputs/day-11.txt")?));
    Ok(())
}

fn part1(input: impl Iterator<Item = String>) -> usize {
    let monkeys = input
        .group_by(|line| line.is_empty())
        .into_iter()
        .map(|(_, lines)| lines.collect::<String>())
        .filter_map(|lines| lines.parse::<Monkey>().ok())
        .map(|monkey| (monkey.id, monkey))
        .collect::<HashMap<_, _>>();

    let keys = monkeys.keys().sorted().copied().collect::<Vec<_>>();

    (0..20)
        .flat_map(|_| keys.iter())
        .fold(monkeys, |mut monkeys, i| {
            let mut monkey = monkeys.get_mut(i).unwrap();

            let moves = monkey
                .items
                .drain(..)
                .rev()
                .map(|item| {
                    let worry = (monkey.operation)(item) / 3;
                    let target = if worry % monkey.divisor == 0 {
                        monkey.targets.0
                    } else {
                        monkey.targets.1
                    };
                    (target, worry)
                })
                .collect::<Vec<_>>();

            monkey.count += moves.len();

            for (target, worry) in moves {
                monkeys.get_mut(&target).unwrap().items.push_back(worry);
            }

            monkeys
        })
        .into_values()
        .map(|monkey| monkey.count)
        .sorted_by_key(|count| Reverse(*count))
        .take(2)
        .product()

    // round(&mut monkeys);
    //
    // monkeys
    //     .into_iter()
    //     .map(|monkey| monkey.count)
    //     .sorted()
    //     .rev()
    //     .take(2)
    //     .product()
}

fn part2(input: impl Iterator<Item = String>) -> usize {
    let mut monkeys = input
        .group_by(|line| line.is_empty())
        .into_iter()
        .map(|(_, lines)| lines.collect::<String>())
        .filter_map(|lines| lines.parse::<Monkey>().ok())
        .collect::<Vec<_>>();

    round(&mut monkeys);

    monkeys
        .into_iter()
        .map(|monkey| monkey.count)
        .sorted()
        .rev()
        .take(2)
        .product()
}

// fn part2(input: impl Iterator<Item = String>) -> usize {
//     todo!()
// }

fn round(monkeys: &mut Vec<Monkey>) {
    for _ in 0..20 {
        for i in 0..monkeys.len() {
            monkeys[i].count += monkeys[i].items.len();
            while let Some(item) = monkeys[i].items.pop_front() {
                let worry = (monkeys[i].operation)(item) / 3;
                let target = if worry % monkeys[i].divisor == 0 {
                    monkeys[i].targets.0
                } else {
                    monkeys[i].targets.1
                };
                monkeys[target as usize].items.push_back(worry);
            }
        }
    }
}

struct Monkey {
    id: u32,
    items: VecDeque<u128>,
    operation: Box<dyn Fn(u128) -> u128>,
    divisor: u128,
    targets: (u32, u32),
    count: usize,
}

impl FromStr for Monkey {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use nom::{
            branch::alt,
            bytes::complete::tag,
            character::complete::{multispace0, u128, u32},
            combinator::map,
            multi::separated_list1,
            sequence::{delimited, preceded},
            Finish, IResult,
        };

        fn parse_id(input: &str) -> IResult<&str, u32> {
            delimited(tag("Monkey "), u32, tag(":"))(input)
        }

        fn parse_items(input: &str) -> IResult<&str, Vec<u128>> {
            preceded(tag("Starting items: "), separated_list1(tag(", "), u128))(input)
        }

        fn parse_operation(input: &str) -> IResult<&str, Box<dyn Fn(u128) -> u128>> {
            enum Expr {
                Old,
                Const(u128),
                Add(Box<Expr>, Box<Expr>),
                Mul(Box<Expr>, Box<Expr>),
            }

            impl Expr {
                fn eval(&self, old: u128) -> u128 {
                    match self {
                        Expr::Old => old,
                        Expr::Const(n) => *n,
                        Expr::Add(lhs, rhs) => lhs.eval(old) + rhs.eval(old),
                        Expr::Mul(lhs, rhs) => lhs.eval(old) * rhs.eval(old),
                    }
                }
            }

            let (i, _) = tag("Operation: ")(input)?;
            let (i, _) = tag("new = ")(i)?;
            let (i, lhs) = alt((map(tag("old"), |_| Expr::Old), map(u128, Expr::Const)))(i)?;
            let (i, op) = delimited(multispace0, alt((tag("+"), tag("*"))), multispace0)(i)?;
            let (i, rhs) = alt((map(tag("old"), |_| Expr::Old), map(u128, Expr::Const)))(i)?;

            let expr = match op {
                "+" => Expr::Add(Box::new(lhs), Box::new(rhs)),
                "*" => Expr::Mul(Box::new(lhs), Box::new(rhs)),
                _ => unreachable!(),
            };

            Ok((i, Box::new(move |old| expr.eval(old))))
        }

        fn parse_test(input: &str) -> IResult<&str, u128> {
            preceded(tag("Test: divisible by "), u128)(input)
        }

        fn parse_throw(input: &str) -> IResult<&str, u32> {
            preceded(tag("throw to monkey "), u32)(input)
        }

        fn parse_outcome(input: &str) -> IResult<&str, (bool, u32)> {
            let parse_true = map(tag("If true: "), |_| true);
            let parse_false = map(tag("If false: "), |_| false);
            let (i, b) = alt((parse_true, parse_false))(input)?;
            let (i, o) = parse_throw(i)?;
            Ok((i, (b, o)))
        }

        fn parse_monkey(input: &str) -> IResult<&str, Monkey> {
            let (i, id) = preceded(multispace0, parse_id)(input)?;
            let (i, items) = preceded(multispace0, parse_items)(i)?;
            let (i, operation) = preceded(multispace0, parse_operation)(i)?;
            let (i, test) = preceded(multispace0, parse_test)(i)?;
            let (i, target1) = preceded(multispace0, parse_outcome)(i)?;
            let (i, target2) = preceded(multispace0, parse_outcome)(i)?;

            Ok((
                i,
                Monkey {
                    id,
                    items: items.into(),
                    operation,
                    divisor: test,
                    targets: (target1.1, target2.1),
                    count: 0,
                },
            ))
        }

        match parse_monkey(s).finish() {
            Ok((_, monkey)) => Ok(monkey),
            Err(err) => Err(err.to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use advent::static_input_lines;
    use indoc::indoc;

    use super::*;

    const INPUT: &'static str = indoc! {"
        Monkey 0:
          Starting items: 79, 98
          Operation: new = old * 19
          Test: divisible by 23
            If true: throw to monkey 2
            If false: throw to monkey 3

        Monkey 1:
          Starting items: 54, 65, 75, 74
          Operation: new = old + 6
          Test: divisible by 19
            If true: throw to monkey 2
            If false: throw to monkey 0

        Monkey 2:
          Starting items: 79, 60, 97
          Operation: new = old * old
          Test: divisible by 13
            If true: throw to monkey 1
            If false: throw to monkey 3

        Monkey 3:
          Starting items: 74
          Operation: new = old + 3
          Test: divisible by 17
            If true: throw to monkey 0
            If false: throw to monkey 1
    "};

    #[test]
    fn test_part1() {
        assert_eq!(part1(static_input_lines(INPUT)), 10605);
    }

    // #[test]
    // fn test_part2() {
    //     assert_eq!(part2(static_input_lines(INPUT)), 10605);
    // }
}
