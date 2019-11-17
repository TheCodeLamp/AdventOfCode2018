use std::time::Instant;
use std::fs::File;
use std::path::Path;
use std::io::{BufReader, BufRead, BufWriter, Write};

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

#[allow(dead_code)]
pub fn tests(){
    let input_file = File::open(Path::new("src/days/day4/input")).expect("Can't find file for day 4 in tests.");
    let output_file = File::create(Path::new("out/sorted_day4.txt")).unwrap();
    let reader = BufReader::new(input_file).lines();

    let mut out_vec = Vec::<String>::with_capacity(reader.size_hint().0);

    for line in reader{
        out_vec.push(line.unwrap());
    }

    out_vec.sort_unstable();

    let mut writer = BufWriter::with_capacity(out_vec.len(), output_file);

    for line in out_vec{
        writer.write(line.as_bytes()).unwrap();
        writer.write("\n".as_bytes()).unwrap();
    }

}