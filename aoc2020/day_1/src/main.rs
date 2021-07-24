use std::{
    fs::File,
    io::{prelude::*, BufReader},
};

fn main() {
    let lines = read_lines("input.txt");
    println!("\nPart 1:");
    part1(lines);
    let lines = read_lines("input.txt");
    println!("\nPart 2:");
    part2(lines);
}

fn read_lines(filename: &str) -> Vec<String> {
    let file = File::open(filename).expect("input.txt should be in same folder as executable");
    let reader = BufReader::new(file);
    reader
        .lines()
        .map(|l| l.expect("Where's the line"))
        .collect()
}

fn part1(lines: Vec<String>) {
    for (i, line1) in lines.iter().enumerate() {
        for line2 in &lines[(i + 1)..] {
            let res = line1.parse::<i32>().unwrap() + line2.parse::<i32>().unwrap();
            if res == 2020 {
                println!("{:?} + {:?} = {}", line1, line2, res);
                let mul = line1.parse::<i32>().unwrap() * line2.parse::<i32>().unwrap();
                println!("{:?} * {:?} = {}", line1, line2, mul);
            }
        }
    }
}

fn part2(lines: Vec<String>) {
    for (i, line1) in lines.iter().enumerate() {
        for (j, line2) in (&lines[(i + 1)..]).iter().enumerate() {
            for line3 in &lines[(j + 1)..] {
                let res = line1.parse::<i32>().unwrap()
                    + line2.parse::<i32>().unwrap()
                    + line3.parse::<i32>().unwrap();
                if res == 2020 {
                    println!("{:?} + {:?} + {:?} = {}", line1, line2, line3, res);
                    let mul = line1.parse::<i32>().unwrap()
                        * line2.parse::<i32>().unwrap()
                        * line3.parse::<i32>().unwrap();
                    println!("{:?} * {:?} * {:?} = {}", line1, line2, line3, mul);
                }
            }
        }
    }
}
