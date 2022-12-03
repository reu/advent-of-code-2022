use std::{fs, str::FromStr};

fn main() {
    let input = fs::read_to_string("inputs/day-02.txt").unwrap_or_default();
    println!("{}", score(&input));
    println!("{}", part2(&input));
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
    use Weapon::*;
    use Outcome::*;
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
    use Weapon::*;
    use Outcome::*;
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

fn score(input: &str) -> usize {
    input
        .lines()
        .filter_map(|line| {
            let (opponent, me) = line.trim().split_once(" ")?;
            Some(match_score(me.parse().ok()?, opponent.parse().ok()?))
        })
        .sum()
}

fn part2(input: &str) -> usize {
    input
        .lines()
        .filter_map(|line| {
            let (opponent, outcome) = line.trim().split_once(" ")?;
            let opponent = opponent.parse().ok()?;
            let me = weapon_for_outcome(opponent, outcome.parse().ok()?);
            Some(match_score(me, opponent))
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_file() {
        assert_eq!(score(""), 0);
    }

    #[test]
    fn test_ignores_invalid_lines() {
        let input = "
            c b
            A Y
            10
        ";
        assert_eq!(score(input), 8);
    }

    #[test]
    fn test_score() {
        let input = "
            A Y
            B X
            C Z
        ";
        assert_eq!(score(input), 15);
    }

    #[test]
    fn test_part2() {
        let input = "
            A Y
            B X
            C Z
        ";
        assert_eq!(part2(input), 12);
    }
}
