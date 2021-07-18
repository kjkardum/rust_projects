use std::collections::{HashMap, HashSet};
use std::{fs::File, io::prelude::*, io::BufReader};

fn main() {
    let map = parse_lines(read_lines("input.txt"));

    let mut part1: HashSet<String> = HashSet::new();
    go_up("shiny gold", &map, &mut part1);
    println!("{}", part1.len());
    println!("{}", go_down("shiny gold", &map) - 1);
}

fn go_up(current: &str, map: &HashMap<String, HashSet<(i32, String)>>, res: &mut HashSet<String>) {
    map.iter().for_each(|(key, value)| {
        value.iter().for_each(|(_, color)| {
            if color == current {
                res.insert(key.to_string());
                go_up(key, map, res);
            }
        })
    });
}

fn go_down(current: &str, map: &HashMap<String, HashSet<(i32, String)>>) -> i32 {
    map[current]
        .iter()
        .map(|(amount, color)| amount * go_down(color, map))
        .sum::<i32>()
        + 1
}

fn parse_lines(lines: Vec<(String, Vec<String>)>) -> HashMap<String, HashSet<(i32, String)>> {
    lines
        .iter()
        .map(|(outer, inner)| {
            (
                outer.clone(),
                inner
                    .iter()
                    .filter(|inner_bags| inner_bags != &"no other bags.")
                    .map(|one| {
                        let mut splitted = one.split(" ").map(|s| s.to_string());
                        (
                            splitted.next().unwrap().parse::<i32>().unwrap(),
                            splitted
                                .filter(|bag_type| {
                                    bag_type != "bag."
                                        && bag_type != "bags."
                                        && bag_type != "bag"
                                        && bag_type != "bags"
                                })
                                .reduce(|acc, item| acc + " " + &item)
                                .expect("Can't read inner bags"),
                        )
                    })
                    .collect(),
            )
        })
        .collect()
}

fn read_lines(filename: &str) -> Vec<(String, Vec<String>)> {
    let file = File::open(filename).expect("File not found");
    let reader = BufReader::new(file);
    reader
        .lines()
        .map(|l| {
            l.expect("Can't read line")
                .split(" contain ")
                .map(|s| s.to_string())
                .collect::<Vec<String>>()
        })
        .map(|splitted| {
            (
                splitted
                    .iter()
                    .next()
                    .unwrap()
                    .split(" ")
                    .map(|i| i.to_string())
                    .filter(|bag_type| {
                        bag_type != "bag."
                            && bag_type != "bags."
                            && bag_type != "bag"
                            && bag_type != "bags"
                    })
                    .reduce(|acc, item| acc + " " + &item)
                    .expect("Can't read outer bag"),
                splitted
                    .clone()
                    .iter()
                    .skip(1)
                    .next()
                    .unwrap()
                    .split(", ")
                    .map(|i| i.to_string())
                    .collect(),
            )
        })
        .collect()
}
