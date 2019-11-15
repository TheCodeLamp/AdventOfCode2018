use std::fs::File;
use std::io::{BufReader, BufRead};

use regex::Regex;
use std::fmt::{Formatter, Error};



pub fn run_part_1() {
    let mut input = read(match File::open("src/days/day4/input") {
        Ok(x) => x,
        _ => panic!(),
    });

    input.sort_unstable_by(|a,b | a.0.cmp(&b.0));



}

#[allow(unused_variables)]
pub fn run_part_2() {
    /*let input = read(match File::open("src/days/day4/input") {
        Ok(x) => x,
        _ => panic!(),
    });*/
}



fn read(f: File) -> Vec<(usize, SleepState)> {
    let lines = BufReader::new(f).lines();

    let mut output = Vec::<(usize, SleepState)>::with_capacity(match lines.size_hint() {
        (x, Some(_)) => x,
        _ => 0,
    });

    for line in lines {
        if let Ok(line_ok) = line {
            let date_int = match parse_to_int(&line_ok) {
                Ok(x) => x,
                _ => panic!("Date error."),
            };

            let state = match parse_to_sleep_state(&line_ok) {
                Ok(x) => x,
                _ => panic!("State error."),
            };

            output.push((date_int, state));
        }
    }

    output
}

fn parse_to_sleep_state(input: &str) -> Result<SleepState, &str> {
    let regex_string = Regex::new(r"]\s(.*)$").unwrap();
    let guard_regex = Regex::new(r"Guard\s#(\d+)\sbegins\sshift").unwrap();

    let state_string = regex_string.captures(&input).expect("Can't find capture string")[1].to_string();

    return match state_string.as_ref() {
        "wakes up" => Result::Ok(SleepState::WakesUp),
        "falls asleep" => Result::Ok(SleepState::FallsAsleep),
        _ => Result::Ok(SleepState::BeginsShift(
            guard_regex.captures(&state_string).unwrap()[1].parse().unwrap())),
    };

}

fn parse_to_int(input: &str) -> Result<usize, &str> {
    let regex_num = Regex::new(r"].*|[^\d]").unwrap();

    let mut int_string = regex_num.replace_all(input, "").to_string();

    if int_string.len() != 12 {
        for _i in 0..(int_string.len() - 12) {
            int_string.pop();
            panic!("not here")
        }
    }

    let int_parsed: usize = int_string.parse().unwrap();
    Result::Ok(int_parsed)
}


#[derive(Copy, Clone, Debug)]
enum SleepState {
    FallsAsleep,
    WakesUp,
    BeginsShift(usize),
}

impl std::fmt::Display for SleepState {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{:?}", self)
    }
}