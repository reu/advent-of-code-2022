use std::{
    collections::HashSet,
    ops::{Add, Sub},
    str::FromStr,
};

use advent::input_lines;

fn main() -> std::io::Result<()> {
    println!("Part1: {}", part1(input_lines("inputs/day-09.txt")?));
    println!("Part2: {}", part2(input_lines("inputs/day-09.txt")?));
    Ok(())
}

fn part1(input: impl Iterator<Item = String>) -> usize {
    let mut rope = Rope::default();
    input
        .filter_map(|line| line.parse::<Movement>().ok())
        .flat_map(|movement| movement.into_iter())
        .map(|position| {
            rope.move_head(position);
            rope.tail
        })
        .collect::<HashSet<_>>()
        .len()
}

fn part2(input: impl Iterator<Item = String>) -> usize {
    let mut ropes = [Rope::default(); 9];
    input
        .filter_map(|line| line.parse::<Movement>().ok())
        .flat_map(|movement| movement.into_iter())
        .filter_map(|position| {
            ropes
                .iter_mut()
                .scan(position, |pos, rope| {
                    let prev_tail = rope.tail;
                    rope.move_head(*pos);
                    let curr_tail = rope.tail;
                    *pos = curr_tail - prev_tail;
                    Some(curr_tail)
                })
                .last()
        })
        .collect::<HashSet<_>>()
        .len()
}

#[derive(Debug, Default, Clone, Copy, Hash, Eq, PartialEq)]
struct Vec2 {
    x: isize,
    y: isize,
}

impl From<(isize, isize)> for Vec2 {
    fn from((x, y): (isize, isize)) -> Self {
        Self { x, y }
    }
}

impl Add for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Self) -> Self::Output {
        Vec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Vec2 {
    type Output = Vec2;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

#[derive(Debug, Default, Clone, Copy)]
struct Rope {
    head: Vec2,
    tail: Vec2,
}

impl Rope {
    fn move_head(&mut self, pos: Vec2) {
        self.head = self.head + pos;

        let Rope { head, tail } = self;

        let dx = (head.x - tail.x).abs();
        let dy = (head.y - tail.y).abs();

        let nx = head.x + if head.x > tail.x { -1 } else { 1 };
        let ny = head.y + if head.y > tail.y { -1 } else { 1 };

        self.tail = if (dx, dy) == (2, 2) {
            (head.x, ny).into()
        } else if dx == 2 {
            (nx, head.y).into()
        } else if dy == 2 {
            (head.x, ny).into()
        } else {
            self.tail
        };
    }
}

enum Dir {
    Up,
    Down,
    Left,
    Right,
}

struct Movement(Dir, usize);

impl FromStr for Movement {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (dir, steps) = s.split_once(' ').ok_or("Invalid movement")?;
        let steps = steps.parse::<usize>().map_err(|_| "Invalid steps")?;
        match dir {
            "U" => Ok(Movement(Dir::Up, steps)),
            "D" => Ok(Movement(Dir::Down, steps)),
            "L" => Ok(Movement(Dir::Left, steps)),
            "R" => Ok(Movement(Dir::Right, steps)),
            _ => Err("Invalid direction"),
        }
    }
}

impl IntoIterator for Movement {
    type Item = Vec2;

    type IntoIter = Box<dyn Iterator<Item = Self::Item>>;

    fn into_iter(self) -> Self::IntoIter {
        Box::new((0..self.1).map(move |_| match self.0 {
            Dir::Up => (0, 1).into(),
            Dir::Down => (0, -1).into(),
            Dir::Left => (-1, 0).into(),
            Dir::Right => (1, 0).into(),
        }))
    }
}

#[cfg(test)]
mod tests {
    use advent::static_input_lines;
    use indoc::indoc;

    use super::*;

    #[test]
    fn test_part1() {
        let input = indoc! {"
            R 4
            U 4
            L 3
            D 1
            R 4
            D 1
            L 5
            R 2
        "};
        assert_eq!(part1(static_input_lines(input)), 13);
    }

    #[test]
    fn test_part2() {
        let input = indoc! {"
            R 5
            U 8
            L 8
            D 3
            R 17
            D 10
            L 25
            U 20
        "};
        assert_eq!(part2(static_input_lines(input)), 36);
    }
}
