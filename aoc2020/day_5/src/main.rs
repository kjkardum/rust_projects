use std::{fs::File, io::prelude::*, io::BufReader};

fn main() {
    let lines = read_input("input.txt");
    println!(
        "First part: {:?}",
        lines
            .iter()
            .map(|line| convert_to_binary(line))
            .map(|binary| convert_to_seat_id(binary))
            .reduce(|ac, el| if ac > el { ac } else { el })
            .expect("Wrong input, can't parse data")
    );

    let mut sorted: Vec<i32> = lines
        .iter()
        .map(|line| convert_to_binary(line))
        .map(|binary| convert_to_seat_id(binary))
        .collect();
    sorted.sort();

    sorted.iter().reduce(|ac, el| {
        if el - ac == 2 {
            println!("Second part {:?}", el - 1);
        };
        el
    });
}

fn convert_to_seat_id(binary: (i32, i32)) -> i32 {
    let (row, column) = binary;
    row * 8 + column
}

fn convert_to_binary(line: &str) -> (i32, i32) {
    let parsed = line
        .chars()
        .map(|c| if c == 'R' || c == 'B' { '1' } else { '0' })
        .collect::<String>();
    (
        i32::from_str_radix(&parsed[..7], 2).unwrap(),
        i32::from_str_radix(&parsed[7..10], 2).unwrap(),
    )
}

fn read_input(filename: &str) -> Vec<String> {
    let file = File::open(filename).expect("File not found!");
    let reader = BufReader::new(file);
    reader
        .lines()
        .map(|l| l.expect("Can't read line"))
        .collect()
}
