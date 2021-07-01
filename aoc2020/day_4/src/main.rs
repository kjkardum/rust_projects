use std::fs::read_to_string;

fn main() {
    let part1: Vec<Vec<String>> = read_to_string("input.txt")
        .expect("file not found.")
        .split("\n\n")
        .map(|passport: &str| {
            passport
                .split(|c| c == '\n' || c == ' ')
                .filter(|field| field.split(":").nth(0).unwrap() != "cid")
                .map(str::to_string)
                .collect()
        })
        .filter(|x: &Vec<String>| x.len() == 7)
        .collect();
    println!("part1: {}", part1.len());
    println!(
        "part2: {}",
        part1
            .into_iter()
            .filter(|passport: &Vec<String>| {
                passport
                    .iter()
                    .map(|field| [&field[..3], field.split(':').nth(1).unwrap()])
                    .filter(|field| match field[0] {
                        "byr" => {
                            field[1].parse::<i32>().unwrap() >= 1920
                                && field[1].parse::<i32>().unwrap() <= 2002
                        }
                        "iyr" => {
                            field[1].parse::<i32>().unwrap() >= 2010
                                && field[1].parse::<i32>().unwrap() <= 2020
                        }
                        "eyr" => {
                            field[1].parse::<i32>().unwrap() >= 2020
                                && field[1].parse::<i32>().unwrap() <= 2030
                        }
                        "hgt" => match &field[1][field[1].len() - 2..] {
                            "cm" => {
                                field[1][..field[1].len() - 2].parse::<i32>().unwrap() >= 150
                                    && field[1][..field[1].len() - 2].parse::<i32>().unwrap() <= 193
                            }
                            "in" => {
                                field[1][..field[1].len() - 2].parse::<i32>().unwrap() >= 59
                                    && field[1][..field[1].len() - 2].parse::<i32>().unwrap() <= 76
                            }
                            _ => false,
                        },
                        "hcl" => {
                            field[1].chars().nth(0).unwrap() == '#' && field[1][1..].len() == 6
                        }
                        "ecl" => {
                            field[1] == "amb"
                                || field[1] == "blu"
                                || field[1] == "brn"
                                || field[1] == "gry"
                                || field[1] == "grn"
                                || field[1] == "hzl"
                                || field[1] == "oth"
                        }
                        "pid" => field[1].len() == 9,
                        _ => false,
                    })
                    .count()
                    == 7
            })
            .count()
    );
}
