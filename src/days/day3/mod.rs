use std::io::{BufReader, BufRead};
use std::fs::File;
use std::collections::HashSet;
use std::str::FromStr;
use std::fs;

//use image::{PNG, save_buffer_with_format, ColorType, ImageBuffer, Rgb, Luma};
//use std::path::Path;
//use image::ColorType::RGB;

pub fn run_part_2() {
    let input = read(match File::open("src/days/day3/input") {
        Ok(x) => x,
        _ => panic!(),
    });

    let mut cloth = [[0u8; 1000]; 1000];


    for trip in input.iter() {
        for i in ((trip.1).0 as usize)..(((trip.2).0 + (trip.1).0) as usize) {
            for j in ((trip.1).1 as usize)..(((trip.2).1 + (trip.1).1) as usize) {
                cloth[i][j] += 1;
            }
        }
    }

    let mut list = HashSet::new();

    for trip in input.iter() {
        list.insert(trip.0.clone());
    }

    for trip in input.iter() {
        let mut restart = false;
        for i in ((trip.1).0 as usize)..(((trip.2).0 + (trip.1).0) as usize) {
            for j in ((trip.1).1 as usize)..(((trip.2).1 + (trip.1).1) as usize) {
                if cloth[i][j] > 1 {
                    list.remove(&trip.0);
                    restart = true;
                    break;
                }
            }
            if restart {
                break;
            }
        }
    }

    println!("ID: {}", list.iter().next().expect("No awnsers. Day 3"))
}


pub fn run_part_1() {
    let input = read(match File::open("src/days/day3/input") {
        Ok(x) => x,
        _ => panic!(),
    });

    let mut cloth = [[0u8; 1000]; 1000];

    //for ten in input.old.old.old { println!("({}, ({}, {}), ({}, {}))", ten.0, (ten.1).0, (ten.1).1, (ten.2).0, (ten.2).1, ); }

    for trip in input {
        for i in ((trip.1).0 as usize)..(((trip.2).0 + (trip.1).0) as usize) {
            for j in ((trip.1).1 as usize)..(((trip.2).1 + (trip.1).1) as usize) {
                cloth[i][j] += 1;
            }
        }
    }

    let mut out = 0u32;

    let mut out_string = String::with_capacity(1000 * 1000);

    for outer_array in cloth.iter() {
        out_string.push_str("\n");
        for value in outer_array.iter() {
            if *value > 1u8 {
                out += 1;
            }
            out_string.push_str(&format!("{}", value));
        }
    }

    fs::write("src/days/day3/fabric.txt", out_string).expect("Cant't write file");

    /*let mut imgbuf: image::GrayImage = image::ImageBuffer::new(1000, 1000);

    // Iterate over the coordinates and pixels of the image
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        *pixel = image::Luma([]);
    }


    render(&data, Path::new("src/days/day3/cloth.png"));*/

    println!("{} square inches", out);
}
/*
fn render(data: &[u8], file_name: &Path) {
    save_buffer_with_format(file_name, data, 1000, 1000, ColorType::Gray(100), PNG);
}*/

fn read(f: File) -> Vec<(u32, (u32, u32), (u32, u32))> {
    let lines = BufReader::new(f).lines();

    let mut output = Vec::<(u32, (u32, u32), (u32, u32))>::with_capacity(match lines.size_hint() {
        (x, Some(_)) => x,
        _ => 0,
    });

    for line in lines {
        if let Ok(x) = line {
            let mut splited = x.split(" @ ").collect::<Vec<&str>>();
            let str1 = splited[0];
            splited = splited[1].split(": ").collect();
            let str2 = splited[0].split(",").collect::<Vec<&str>>();
            let str3 = splited[1].split("x").collect::<Vec<&str>>();
            let temp1: u32 = if let Ok(x) = u32::from_str({ str1.split("#").skip(1).next().unwrap() }) { x } else { 0 };
            let temp2: (u32, u32) = ({ if let Ok(x) = u32::from_str(str2[0]) { x } else { 0 } }, { if let Ok(x) = u32::from_str(str2[1]) { x } else { 0 } });
            let temp3: (u32, u32) = ({ if let Ok(x) = u32::from_str(str3[0]) { x } else { 0 } }, { if let Ok(x) = u32::from_str(str3[1]) { x } else { 0 } });
            output.push((temp1, temp2, temp3));
        }
    }

    output
}