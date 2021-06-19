use std::{fs::File, io::prelude::*, io::BufReader};

struct PasswordPolicy {
    range: (usize, usize),
    character: char,
    password: String,
}
impl PasswordPolicy {
    fn new(input: &str) -> Self {
        let mut iterator = input.split(|c| c == ' ' || c == '-' || c == ':');
        let range = (
            iterator.next().unwrap().parse().unwrap(),
            iterator.next().unwrap().parse().unwrap(),
        );
        let character = iterator.next().unwrap().chars().nth(0).unwrap();
        iterator.next();
        let password = iterator.next().unwrap().to_owned();
        Self {
            range,
            character,
            password,
        }
    }
    fn check_for_part_1(&self) -> bool {
        let count = self
            .password
            .chars()
            .filter(|c| *c == self.character)
            .count();
        count >= self.range.0 && count <= self.range.1
    }
    fn check_for_part_2(&self) -> bool {
        let first = self.password.chars().nth(self.range.0 - 1).unwrap() == self.character;
        let second = self.password.chars().nth(self.range.1 - 1).unwrap() == self.character;
        first ^ second
    }
}

fn main() {
    let lines: Vec<PasswordPolicy> = read_lines("input.txt")
        .iter()
        .map(|line| PasswordPolicy::new(&line[..]))
        .collect();
    println!(
        "Part 1: {}",
        lines
            .iter()
            .filter(|line| PasswordPolicy::check_for_part_1(line))
            .count()
    );
    println!(
        "Part 2: {}",
        lines
            .iter()
            .filter(|line| PasswordPolicy::check_for_part_2(line))
            .count()
    );
}

fn read_lines(filename: &str) -> Vec<String> {
    let file = File::open(filename).expect("No filename provided");
    let reader = BufReader::new(file);
    reader
        .lines()
        .map(|l| l.expect("Can't read line"))
        .collect()
}
