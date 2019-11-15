use std::io::{BufReader, BufRead};
use std::fs::File;
use std::collections::HashSet;

pub fn run_part_1() {

    let input_file = File::open("src/days/day1/input.old.old").expect("No file day 1");

    let input_reader = BufReader::new(input_file);

    let mut _x = 0;

    for line in input_reader.lines() {
        let ent = line.unwrap().clone();

        match ent.split_at(1).0 {
            "+" => _x += ent.split_at(1).1.parse::<i32>().expect("Cant parse to int."),
            "-" => _x -= ent.split_at(1).1.parse::<i32>().expect("Cant parse to int."),
            _ => println!("Däng"),
        }
    }

    println!("{}", _x);
}


pub fn run_part_2() {
    let input_file = File::open("src/days/day1/input.old.old").expect("No file day 1");

    let input_reader = BufReader::new(input_file).lines();

    let mut _x = 0;

    let mut lines: Vec<String> = Vec::new();

    for line in input_reader {
        lines.push(line.unwrap())
    }

    let mut visited: HashSet<String> = HashSet::new();

    let mut index: usize = 0;

    loop {
        let ent;
        match lines.get(index as usize) {
            None => {
                println!("ERRRR: {} > {}", index, lines.len());
                break;
            },
            Some(s) => {
                ent = s.clone();
            }

        }

        index = (index + 1)%lines.len();

        match ent.split_at(1).0 {
            "+" => _x = add(_x, &ent.split_at(1).1.parse::<i32>().expect("Cant parse to int."), &mut visited),
            "-" => _x = sub(_x, &ent.split_at(1).1.parse::<i32>().expect("Cant parse to int."), &mut visited),
            _ => println!("Däng"),
        }

        if visited.contains(format!("{}", _x).as_str()) {
            // println!("{}. Len = {}", _x, visited.len());
            break;
        }
    }

    println!("{} is repeted. Looped {} times", _x, visited.len() / lines.len());
}

fn add(x: i32, y: &i32, visited: &mut std::collections::HashSet<String>) -> i32 {
    visited.insert(format!("{}", x));
    x + *y as i32
}

fn sub(x: i32, y: &i32, visited: &mut std::collections::HashSet<String>) -> i32 {
    visited.insert(format!("{}", x));
    x - *y as i32
}