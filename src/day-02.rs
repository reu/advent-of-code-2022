use advent::read_input;
use std::str::FromStr;

fn main() -> std::io::Result<()> {
    println!("Part1: {}", part1(read_input("inputs/day-02.txt")?));
    println!("Part2: {}", part2(read_input("inputs/day-02.txt")?));
    Ok(())
}

#[derive(Clone, Copy)]
enum Weapon {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl FromStr for Weapon {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Self::Rock),
            "B" | "Y" => Ok(Self::Paper),
            "C" | "Z" => Ok(Self::Scissors),
            _ => Err("Invalid weapon"),
        }
    }
}

enum Outcome {
    Win = 6,
    Draw = 3,
    Loss = 0,
}

impl FromStr for Outcome {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Z" => Ok(Self::Win),
            "Y" => Ok(Self::Draw),
            "X" => Ok(Self::Loss),
            _ => Err("Invalid outcome"),
        }
    }
}

fn match_outcome(me: Weapon, opponent: Weapon) -> Outcome {
    use Outcome::*;
    use Weapon::*;
    match (me, opponent) {
        (Rock, Scissors) => Win,
        (Paper, Rock) => Win,
        (Scissors, Paper) => Win,
        (Rock, Paper) => Loss,
        (Paper, Scissors) => Loss,
        (Scissors, Rock) => Loss,
        _ => Draw,
    }
}

fn weapon_for_outcome(opponent: Weapon, outcome: Outcome) -> Weapon {
    use Outcome::*;
    use Weapon::*;
    match (opponent, outcome) {
        (Rock, Win) => Paper,
        (Rock, Loss) => Scissors,
        (Paper, Win) => Scissors,
        (Paper, Loss) => Rock,
        (Scissors, Win) => Rock,
        (Scissors, Loss) => Paper,
        (weapon, Draw) => weapon,
    }
}

fn match_score(me: Weapon, opponent: Weapon) -> usize {
    match_outcome(me, opponent) as usize + me as usize
}

fn part1(input: impl Iterator<Item = String>) -> usize {
    input
        .filter_map(|line| {
            let (opponent, me) = line.trim().split_once(' ')?;
            Some(match_score(me.parse().ok()?, opponent.parse().ok()?))
        })
        .sum()
}

fn part2(input: impl Iterator<Item = String>) -> usize {
    input
        .filter_map(|line| {
            let (opponent, outcome) = line.trim().split_once(' ')?;
            let opponent = opponent.parse().ok()?;
            let me = weapon_for_outcome(opponent, outcome.parse().ok()?);
            Some(match_score(me, opponent))
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use advent::static_input;

    use super::*;

    #[test]
    fn test_part1() {
        let input = "
            A Y
            B X
            C Z
        ";
        assert_eq!(part1(static_input(input)), 15);
    }

    #[test]
    fn test_part2() {
        let input = "
            A Y
            B X
            C Z
        ";
        assert_eq!(part2(static_input(input)), 12);
    }
}
