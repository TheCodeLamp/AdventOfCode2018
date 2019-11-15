use std::time::Instant;

mod day1;
mod day2;
mod day3;
mod day4;

#[allow(dead_code)]
pub fn run_day_4() {
    day4::run_part_1();

    println!("---------------------------------------------------------");

    day4::run_part_2();
}

#[allow(dead_code)]
pub fn run_day_3() {
    day3::run_part_1();

    println!("---------------------------------------------------------");

    day3::run_part_2();
}

#[allow(dead_code)]
pub fn run_day_2() {
    println!("22 day till christmas!");

    day2::run_part_1();

    println!();

    day2::run_part_2()
}

#[allow(dead_code)]
pub fn run_day_1() {
    println!("23 day till christmas!");

    day1::run_part_1();

    let time = Instant::now();

    day1::run_part_2();

    println!("{} ms", time.elapsed().as_millis())
}