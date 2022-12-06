use std::{
    fs,
    io::{self, BufRead, BufReader, Read},
    path::Path,
};

pub fn read_input(path: impl AsRef<Path>) -> io::Result<impl Iterator<Item = String>> {
    Ok(BufReader::new(fs::File::open(path)?)
        .lines()
        .filter_map(|line| line.ok()))
}

pub fn input_bytes(path: impl AsRef<Path>) -> io::Result<impl Iterator<Item = u8>> {
    Ok(BufReader::new(fs::File::open(path)?)
        .bytes()
        .filter_map(|byte| byte.ok()))
}

pub fn static_input(input: &'static str) -> impl Iterator<Item = String> {
    input.lines().map(|line| line.to_owned())
}

pub fn static_input_bytes(input: &'static str) -> impl Iterator<Item = u8> {
    input.bytes()
}
