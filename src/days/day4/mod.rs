use std::fs::File;
use std::io::{BufReader, BufRead, BufWriter, Write};

use regex::Regex;
use std::fmt::{Formatter, Error};
use std::collections::{HashMap, HashSet};

#[derive(Copy, Clone, Eq, PartialEq)]
struct Guard {
    id: usize,
    sleep_total: usize,
    sleep_max: usize,
    sleep_start: Option<usize>,
}

pub fn run_part_1() {
    let mut input = read(match File::open("src/days/day4/input") {
        Ok(x) => x,
        _ => panic!("File for day 4 could'nt read."),
    });


    input.sort_unstable_by(|a, b| a.0.cmp(&b.0));

    let mut guard_map = HashMap::<usize, Guard>::new();
    let mut guards = HashSet::<usize>::new();
    let mut current_guard= 0;
    let mut highest_guard = (0,0);

    for (date_time, state) in input {
        match state {
            SleepState::BeginsShift(id) => {
                if guard_map.contains_key(&id) {
                    current_guard = id;
                    assert_eq!(guard_map.get(&id).unwrap().sleep_start, None);
                } else {
                    current_guard = id;
                    guards.insert(id);
                    guard_map.insert(id, Guard {
                        id,
                        sleep_total: 0,
                        sleep_max: 0,
                        sleep_start: None,
                    });
                }
            }

            SleepState::FallsAsleep => {
                let temp_guard = guard_map.get_mut(&current_guard).unwrap();
                assert_eq!(temp_guard.sleep_start, None);
                temp_guard.sleep_start = Some(date_time);
            }

            SleepState::WakesUp => {
                let temp_guard = guard_map.get_mut(&current_guard).unwrap();
                assert_ne!(temp_guard.sleep_start, None);
                let time = date_time - temp_guard.sleep_start.unwrap();
                if temp_guard.sleep_max < time{
                    temp_guard.sleep_max = time;
                }
                temp_guard.sleep_total += time;
                temp_guard.sleep_start = None;
                if temp_guard.sleep_total > highest_guard.1{
                    highest_guard.0 = temp_guard.id;
                    highest_guard.1 = temp_guard.sleep_total;
                }
            }
        }
    }

    let mut writer = BufWriter::new(File::create("src/days/day4/output_part1.txt").unwrap());

    let mut guard_list: Vec<usize> = guards.clone().into_iter().collect();
    guard_list.sort_unstable();

    for guard in guard_list {
        let current = guard_map.get(&guard).unwrap();
        writer.write(format!("Guard {} slept {} time.", current.id, current.sleep_total).as_bytes()).unwrap();
        writer.write("\n".as_bytes()).unwrap();
    }

    println!("Awnser is {} * {} = {}", highest_guard.0, guard_map.get(&highest_guard.0).unwrap().sleep_max, highest_guard.0*guard_map.get(&highest_guard.0).unwrap().sleep_max)

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

    let mut output = Vec::<(usize, SleepState)>::with_capacity(lines.size_hint().0);

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