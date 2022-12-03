use std::{
    fs,
    io::{self, BufRead, BufReader},
    path::Path,
};

pub fn read_input(path: impl AsRef<Path>) -> io::Result<impl Iterator<Item = String>> {
    Ok(BufReader::new(fs::File::open(path)?)
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| line.trim().to_owned()))
}

pub fn static_input(input: &'static str) -> impl Iterator<Item = String> {
    input
        .lines()
        .map(|line| line.trim())
        .map(|line| line.to_owned())
}
