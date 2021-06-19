use std::{
    fs::File,
    io::prelude::*,
    io::BufReader,
};

fn main() {
    let lines = read_lines("input.txt");
    let mut part1_counter = 0;
    let mut part2_counter = 0;
    for line in lines {
        let splitted = line
            .split(|c| c==':' || c==' ' || c=='-')
            .take(5)
            .collect::<Vec<&str>>();
        if let [min, max, letter, _, password] = &splitted[..]{
            let occurences =  password.matches(letter).count();
            if (occurences as i32)>=min.parse::<i32>().unwrap() &&
                (occurences as i32)<=max.parse::<i32>().unwrap() {
                    part1_counter += 1;
            }
            let first = (password.
                            as_bytes()
                            [(min.parse::<i32>().unwrap() as usize)-1]
                                as char)
                            .to_string().eq(letter);
            let second = (password.
                            as_bytes()
                            [(max.parse::<i32>().unwrap() as usize)-1]
                                as char)
                            .to_string().eq(letter);
            if (first as i32) + (second as i32) == 1 {
                part2_counter += 1
            }
        }
    }
    println!("Part 1: {}", part1_counter);
    println!("Part 2: {}", part2_counter);
}

fn read_lines(filename: &str) -> Vec<String> {
    let file = File::open(filename).expect("No filename provided");
    let reader = BufReader::new(file);
    reader.lines()
          .map(|l| l.expect("Can't read line"))
          .collect()
}