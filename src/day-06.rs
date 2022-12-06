use std::{
    collections::{HashSet, VecDeque},
    iter::FromIterator,
};

use advent::input_bytes;

fn main() -> std::io::Result<()> {
    println!("Part1: {}", part1(input_bytes("inputs/day-06.txt")?));
    println!("Part2: {}", part2(input_bytes("inputs/day-06.txt")?));
    Ok(())
}

fn part1(input: impl Iterator<Item = u8>) -> usize {
    input
        .overlapping_chunks(4)
        .position(|bytes| HashSet::<u8>::from_iter(bytes).len() == 4)
        .map(|pos| pos + 4)
        .unwrap_or_default()
}

fn part2(input: impl Iterator<Item = u8>) -> usize {
    input
        .overlapping_chunks(14)
        .position(|bytes| HashSet::<u8>::from_iter(bytes).len() == 14)
        .map(|pos| pos + 14)
        .unwrap_or_default()
}

trait OverlappingChunkExt: Iterator {
    fn overlapping_chunks(self, size: usize) -> OverlappingChunk<Self>
    where
        Self: Sized,
    {
        OverlappingChunk {
            iter: self,
            buf: Default::default(),
            size,
        }
    }
}

impl<T> OverlappingChunkExt for T where T: Iterator {}

struct OverlappingChunk<I: Iterator> {
    iter: I,
    buf: VecDeque<I::Item>,
    size: usize,
}
impl<I> Iterator for OverlappingChunk<I>
where
    I: Iterator,
    I::Item: Clone,
{
    type Item = Vec<I::Item>;

    fn next(&mut self) -> Option<Self::Item> {
        for item in self.iter.by_ref() {
            self.buf.push_front(item);
            if self.buf.len() == self.size {
                let chunk = self.buf.iter().cloned().collect::<Vec<_>>();
                self.buf.pop_back();
                return Some(chunk);
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use advent::static_input_bytes as input;

    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(input("bvwbjplbgvbhsrlpgdmjqwftvncz")), 5);
        assert_eq!(part1(input("nppdvjthqldpwncqszvftbrmjlhg")), 6);
        assert_eq!(part1(input("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg")), 10);
        assert_eq!(part1(input("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw")), 11);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(input("mjqjpqmgbljsphdztnvjfqwrcgsmlb")), 19);
        assert_eq!(part2(input("bvwbjplbgvbhsrlpgdmjqwftvncz")), 23);
        assert_eq!(part2(input("nppdvjthqldpwncqszvftbrmjlhg")), 23);
        assert_eq!(part2(input("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg")), 29);
        assert_eq!(part2(input("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw")), 26);
    }
}
