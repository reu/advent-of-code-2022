use std::{
    collections::HashSet,
    iter::FromIterator,
    mem::{self, MaybeUninit},
};

use advent::input_bytes;

fn main() -> std::io::Result<()> {
    println!("Part1: {}", part1(input_bytes("inputs/day-06.txt")?));
    println!("Part2: {}", part2(input_bytes("inputs/day-06.txt")?));
    Ok(())
}

fn part1(input: impl Iterator<Item = u8>) -> usize {
    input
        .overlapping_chunks::<4>()
        .position(|bytes| HashSet::<u8>::from_iter(bytes).len() == 4)
        .map(|pos| pos + 4)
        .unwrap_or_default()
}

fn part2(input: impl Iterator<Item = u8>) -> usize {
    input
        .overlapping_chunks::<14>()
        .position(|bytes| HashSet::<u8>::from_iter(bytes).len() == 14)
        .map(|pos| pos + 14)
        .unwrap_or_default()
}

trait OverlappingChunkExt: Iterator {
    fn overlapping_chunks<const N: usize>(self) -> OverlappingChunk<Self, N>
    where
        Self: Sized,
    {
        OverlappingChunk {
            iter: self,
            buf: unsafe { MaybeUninit::uninit().assume_init() },
            written: 0,
        }
    }
}

impl<T> OverlappingChunkExt for T where T: Iterator {}

struct OverlappingChunk<I: Iterator, const N: usize> {
    iter: I,
    buf: [MaybeUninit<I::Item>; N],
    written: usize,
}

impl<I, const N: usize> Iterator for OverlappingChunk<I, N>
where
    I: Iterator,
    I::Item: Clone,
{
    type Item = [I::Item; N];

    fn next(&mut self) -> Option<Self::Item> {
        for item in self.iter.by_ref() {
            self.buf[self.written.clamp(0, N - 1)].write(item);
            self.written += 1;
            if self.written >= N {
                let window = unsafe { mem::transmute_copy(&self.buf) };
                self.buf.rotate_left(1);
                return Some(window);
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
