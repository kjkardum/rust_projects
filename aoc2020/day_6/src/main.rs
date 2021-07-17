use std::collections::HashSet;
use std::fs::read_to_string;

fn main() {
    let first: usize = read_to_string("input.txt")
        .expect("File not found")
        .split("\n\n")
        .map(|group| {
            let mut data_set = HashSet::new();
            group.replace("\n", "").chars().for_each(|letter| {
                data_set.insert(letter);
            });
            data_set.len()
        })
        .sum();
    println!("First part: {:?}", first);

    let alphabet = "abcdefghijklmnopqrstuvwxyz";
    let second: usize = read_to_string("input.txt")
        .expect("File not found")
        .split("\n\n")
        .map(|group| {
            let length = group.matches(|c| c == '\n').count() + 1;
            alphabet
                .clone()
                .chars()
                .filter(|c| group.matches(|t| *c == t).count() == length)
                .count()
        })
        .sum();
    println!("Second part {:?}", second)
}
