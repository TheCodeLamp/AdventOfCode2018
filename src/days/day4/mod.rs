use std::fs::File;
use std::io::{BufReader, BufRead};

use regex::Regex;
use std::fmt;
use std::fmt::{Formatter, Error};

pub fn run_part_1(){
    let input = read(match File::open("src/days/day4/input") {
        Ok(x) => x,
        _ => panic!(),
    });

    for (integer, state) in input{
        println!("{} {}", integer, state.to_string())
    }

}

pub fn run_part_2(){
    let input = read(match File::open("src/days/day4/input") {
        Ok(x) => x,
        _ => panic!(),
    });





}

fn read(f: File) -> Vec<(usize, SleepState)> {
    let lines = BufReader::new(f).lines();

    let mut output = Vec::<(usize, SleepState)>::with_capacity(match lines.size_hint() {
        (x, Some(_)) => x,
        _ => 0,
    });

    let regex_num = Regex::new(r"(\d[4])-(\d[2])-(\d[2])\s(\d[2]):(\d[2])").unwrap();
    let regex_string = Regex::new(r"\[\s(.*)&").unwrap();
    let guard_regex = Regex::new(r"Guard\s#(\d)+\sbegins\sshift").unwrap();

    for line in lines {
        if let Ok(x) = line {
            let int = {
                let caps = regex_num.captures(&x).expect("Can't find capture");
                let caps = regex_num.captures(&x).unwrap();
                let mut int_string = "".to_owned();
                for cap in caps.iter(){

                }
                let int_parse: usize = int_string.parse().expect("Can't parse");
                int_parse
            };

            let string = regex_string.captures(&x).expect("Can't find capture string")[0].to_string();

            if string.eq(&"wakes up".to_string()){
                output.push((int, SleepState::WakesUp));
                continue;
            } else if string.eq(&"falls asleep".to_string()){
                output.push((int, SleepState::FallsAsleep));
            } else if {
                guard_regex.is_match(&string)
            }{
                output.push((int, SleepState::BeginsShift(guard_regex.captures(&string).unwrap()[0].parse().unwrap())))
            } else {
                panic!("SleepState could'nt parse.")
            }
        }
    }

    output

}

enum SleepState {
    FallsAsleep,
    WakesUp,
    BeginsShift(usize),
}

impl fmt::Display for SleepState{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{:?}", self)
    }
}