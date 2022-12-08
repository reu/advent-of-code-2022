use std::iter::FromIterator;

use advent::input_lines;

fn main() -> std::io::Result<()> {
    println!("Part1: {}", part1(input_lines("inputs/day-08.txt")?));
    println!("Part2: {}", part2(input_lines("inputs/day-08.txt")?));
    Ok(())
}

fn part1(input: impl Iterator<Item = String>) -> usize {
    let grid = input
        .map(|line| {
            line.chars()
                .filter_map(|char| char.to_digit(10))
                .collect::<Vec<_>>()
        })
        .collect::<Grid<_>>();

    let visible = (1..grid.rows - 1)
        .flat_map(|row| (1..grid.cols - 1).map(move |col| (row, col)))
        .filter(|(row, col)| {
            let (row, col) = (*row, *col);
            let tree = grid.get(row, col).unwrap();
            grid.iter_row(row).take(col).all(|item| item < tree)
                || grid.iter_row(row).skip(col + 1).all(|item| item < tree)
                || grid.iter_col(col).take(row).all(|item| item < tree)
                || grid.iter_col(col).skip(row + 1).all(|item| item < tree)
        })
        .count();

    visible + grid.cols * 2 + grid.rows * 2 - 4
}

fn part2(input: impl Iterator<Item = String>) -> usize {
    let grid = input
        .map(|line| {
            line.chars()
                .filter_map(|char| char.to_digit(10))
                .collect::<Vec<_>>()
        })
        .collect::<Grid<_>>();

    (1..grid.rows - 1)
        .flat_map(|row| (1..grid.cols - 1).map(move |col| (row, col)))
        .map(|(row, col)| {
            let tree = grid.get(row, col).unwrap();

            let check = |item: &&u32| *item < tree;

            let left = grid
                .iter_row(row)
                .take(col)
                .collect::<Vec<_>>()
                .into_iter()
                .rev()
                .take_while_inclusive(check)
                .count();

            let right = grid
                .iter_row(row)
                .skip(col + 1)
                .take_while_inclusive(check)
                .count();

            let up = grid
                .iter_col(col)
                .take(row)
                .collect::<Vec<_>>()
                .into_iter()
                .rev()
                .take_while_inclusive(check)
                .count();

            let down = grid
                .iter_col(col)
                .skip(row + 1)
                .take_while_inclusive(check)
                .count();

            [up, down, left, right]
                .map(|n| if n == 0 { 1 } else { n })
                .into_iter()
                .product()
        })
        .max()
        .unwrap_or_default()
}

#[derive(Debug)]
struct Grid<T> {
    rows: usize,
    cols: usize,
    data: Vec<T>,
}

impl<T> Grid<T> {
    fn new() -> Self {
        Self {
            rows: 0,
            cols: 0,
            data: Vec::new(),
        }
    }

    fn push_row(&mut self, row: impl IntoIterator<Item = T>) {
        self.data.extend(row);
        self.rows += 1;
        if self.cols == 0 {
            self.cols = self.data.len();
        }
    }

    fn get(&self, row: usize, col: usize) -> Option<&T> {
        self.data.get(row * self.cols + col)
    }

    fn iter_row(&self, row: usize) -> impl Iterator<Item = &T> {
        let start = row * self.cols;
        self.data[start..(start + self.cols)].iter()
    }

    fn iter_col(&self, col: usize) -> impl Iterator<Item = &T> {
        self.data[col..].iter().step_by(self.cols)
    }
}

impl<E, I: IntoIterator<Item = E>> FromIterator<I> for Grid<E> {
    fn from_iter<T: IntoIterator<Item = I>>(iter: T) -> Self {
        let mut grid = Grid::new();
        for row in iter.into_iter() {
            grid.push_row(row)
        }
        grid
    }
}

trait TakeWhileInclusiveExt: Iterator {
    fn take_while_inclusive<P>(self, predicate: P) -> TakeWhileInclusive<Self, P>
    where
        Self: Sized,
        P: FnMut(&Self::Item) -> bool,
    {
        TakeWhileInclusive {
            iter: self,
            found: false,
            finished: false,
            predicate,
        }
    }
}

impl<T> TakeWhileInclusiveExt for T where T: Iterator {}

struct TakeWhileInclusive<I, P> {
    iter: I,
    found: bool,
    finished: bool,
    predicate: P,
}

impl<I: Iterator, P> Iterator for TakeWhileInclusive<I, P>
where
    P: FnMut(&I::Item) -> bool,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<I::Item> {
        if self.finished {
            None
        } else {
            let x = self.iter.next()?;
            if (self.predicate)(&x) {
                self.found = true;
                Some(x)
            } else if self.found {
                self.finished = true;
                Some(x)
            } else {
                None
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use advent::static_input_lines;
    use indoc::indoc;

    use super::*;

    const INPUT: &'static str = indoc! {"
        30373
        25512
        65332
        33549
        35390
    "};

    #[test]
    fn test_part1() {
        assert_eq!(part1(static_input_lines(INPUT)), 21);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(static_input_lines(INPUT)), 8);
    }
}
