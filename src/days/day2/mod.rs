use std::io::{BufReader, BufRead};
use std::fs::File;
use std::collections::HashMap;
use std::iter::FromIterator;

pub fn run_part_2(){
    let input = read(File::open("src/days/day2/input.old").expect("No file day 2"));

    let mut best = (0u32, String::new(), String::new(), String::new());

    for (i, line) in input.iter().enumerate(){
        for line2 in input.iter().skip(i){

            if line == line2 {continue;};

            let mut best_id = 0u32;

            let word: Vec<char> = line2.chars().collect();

            let mut similar = Vec::<char>::new();

            for (j, c) in line.chars().enumerate(){
                if let Some(c2) = word.get(j){
                    if c == *c2 {
                        best_id += 1;
                        similar.push(c.clone());
                    }else{
                    }
                }
            }


            if best.0 < best_id{
                best = (best_id, line.clone(), line2.clone(), String::from_iter(similar.iter()));
            }

        }
    }

    println!("{}", best.3);

}

pub fn run_part_1() {
    let input_file = File::open("src/days/day2/input.old").expect("No file day 2");

    let input = read(input_file);

    let count = count(&input);

    println!("Checksum is: {:?}", (count.0 * count.1))
}

fn count(input: &[String]) -> (u32, u32) {
    let mut threes = 0;
    let mut twoos = 0;

    for line in input {
        let mut map = HashMap::<char, u8>::with_capacity(line.len());


        for c in line.chars() {
            if map.contains_key(&c) {
                if let Some(x) = map.get_mut(&c) {
                    *x += 1;
                }
            } else {
                map.insert(c, 1);
            }
        }


        let mut tr_bool = false;
        let mut tw_bool = false;

        for c in map.values() {
            match c {
                3 => tr_bool = true,
                2 => tw_bool = true,
                _ => {}
            };
        }

        if tr_bool { threes += 1; };

        if tw_bool { twoos += 1 };
    }

    (threes, twoos)
}


fn read(f: File) -> Vec<String> {
    let lines = BufReader::new(f).lines();

    let mut output = Vec::<String>::with_capacity(match lines.size_hint() {
        (x, Some(_)) => x,
        _ => 0,
    });

    for line in lines {
        if let Ok(x) = line {
            output.push(x);
        }
    }

    output
}