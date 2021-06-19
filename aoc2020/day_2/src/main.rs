use std::{fs::File, io::prelude::*, io::BufReader};

fn main() {
    let lines = read_lines("input.txt");
    let mut part1_counter = 0;
    let mut part2_counter = 0;
    for line in lines {
        parse_line(&line[..], &mut part1_counter, &mut part2_counter);
    }
    println!("Part 1: {}", part1_counter);
    println!("Part 2: {}", part2_counter);
}

fn parse_line(line: &str, part1_counter: &mut i32, part2_counter: &mut i32) -> Option<i32> {
    let mut split_iter = line.split(|c| c == ':' || c == ' ' || c == '-');

    let min = split_iter.next()?.parse::<i32>().ok()?;
    let max = split_iter.next()?.parse::<i32>().ok()?;
    let letter = split_iter.next()?;
    split_iter.next()?;
    let password = split_iter.next()?;

    let occurences = password.matches(letter).count();
    if (occurences as i32) >= min && (occurences as i32) <= max {
        *part1_counter += 1;
    }
    let first = (password.as_bytes()[(min as usize) - 1] as char)
        .to_string()
        .eq(letter);
    let second = (password.as_bytes()[(max as usize) - 1] as char)
        .to_string()
        .eq(letter);
    if first ^ second {
        *part2_counter += 1
    }
    Some(0)
}

fn read_lines(filename: &str) -> Vec<String> {
    let file = File::open(filename).expect("No filename provided");
    let reader = BufReader::new(file);
    reader
        .lines()
        .map(|l| l.expect("Can't read line"))
        .collect()
}
