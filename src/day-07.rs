use std::{
    collections::HashMap,
    iter::FromIterator,
    path::{Path, PathBuf},
    str::FromStr,
};

use advent::input_lines;
use itertools::Itertools;

fn main() -> std::io::Result<()> {
    println!("Part1: {}", part1(input_lines("inputs/day-07.txt")?));
    println!("Part2: {}", part2(input_lines("inputs/day-07.txt")?));
    Ok(())
}

fn part1(input: impl Iterator<Item = String>) -> usize {
    FileSystem::from_iter(input)
        .dirs()
        .filter(|(_, size)| *size <= 100000)
        .map(|(_, size)| size)
        .sum()
}

fn part2(input: impl Iterator<Item = String>) -> usize {
    let fs = FileSystem::from_iter(input);

    let used = fs.dir_size("/");
    let free = 70_000_000 - used;

    fs.dirs()
        .map(|(_, size)| size)
        .sorted()
        .find(|size| *size >= 30_000_000 - free)
        .unwrap_or_default()
}

struct FileSystem {
    cwd: PathBuf,
    dirs: HashMap<PathBuf, usize>,
}

impl FileSystem {
    fn new() -> Self {
        Self {
            cwd: PathBuf::from("/"),
            dirs: HashMap::new(),
        }
    }

    fn cd(&mut self, dir: &str) {
        if dir == ".." {
            self.cwd.pop();
        } else {
            self.cwd.push(dir)
        }
    }

    fn add_size(&mut self, size: usize) {
        for dir in self.cwd.ancestors() {
            *self.dirs.entry(dir.to_owned()).or_default() += size;
        }
    }

    fn dir_size(&self, dir: impl AsRef<Path>) -> usize {
        self.dirs.get(dir.as_ref()).copied().unwrap_or_default()
    }

    fn dirs(&self) -> impl Iterator<Item = (&Path, usize)> {
        self.dirs.iter().map(|(name, size)| (name.as_path(), *size))
    }
}

impl FromIterator<String> for FileSystem {
    fn from_iter<T: IntoIterator<Item = String>>(iter: T) -> Self {
        iter.into_iter()
            .filter_map(|line| line.parse::<Entry>().ok())
            .fold(FileSystem::new(), |mut fs, entry| match entry {
                Entry::Cd(dir) => {
                    fs.cd(&dir);
                    fs
                }
                Entry::File(size, _) => {
                    fs.add_size(size);
                    fs
                }
                _ => fs,
            })
    }
}

#[derive(Debug)]
enum Entry {
    Ls,
    Cd(String),
    Dir(String),
    File(usize, String),
}

impl FromStr for Entry {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split(' ').collect::<Vec<_>>().as_slice() {
            ["$", "cd", dir] => Ok(Entry::Cd(dir.to_string())),
            ["$", "ls"] => Ok(Entry::Ls),
            ["dir", dir] => Ok(Entry::Dir(dir.to_string())),
            [size, name] => Ok(Entry::File(
                size.parse::<usize>().map_err(|err| err.to_string())?,
                name.to_string(),
            )),
            entry => Err(entry.join(" ")),
        }
    }
}

#[cfg(test)]
mod tests {
    use advent::static_input_lines;
    use indoc::indoc;

    use super::*;

    const INPUT: &'static str = indoc! {"
        $ cd /
        $ ls
        dir a
        14848514 b.txt
        8504156 c.dat
        dir d
        $ cd a
        $ ls
        dir e
        29116 f
        2557 g
        62596 h.lst
        $ cd e
        $ ls
        584 i
        $ cd ..
        $ cd ..
        $ cd d
        $ ls
        4060174 j
        8033020 d.log
        5626152 d.ext
        7214296 k
    "};

    #[test]
    fn test_part1() {
        assert_eq!(part1(static_input_lines(INPUT)), 95437);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(static_input_lines(INPUT)), 24933642);
    }
}
