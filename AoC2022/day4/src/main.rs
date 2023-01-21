use std::fs;
use std::error::Error;
use std::cmp::min;
use std::cmp::max;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, world!");

    let data = fs::read_to_string("data.txt")?;

    let split: Vec<&str> = data.split("\n").collect();

    let mut counter:i32 = 0;

    let mut overlap_counter = 0;

    println!("lines count {}", split.len());

    for line in split {
        let split_line: Vec<&str> = line.split(",").collect();

        let e_1: Vec<&str> = split_line.get(0).unwrap().split("-").collect();
        let e_2: Vec<&str> = split_line.get(1).unwrap().split("-").collect();

        let e_1_l: u32 = e_1.get(0).unwrap().parse().unwrap();
        let e_1_h: u32 = e_1.get(1).unwrap().parse().unwrap();
        let e_2_l: u32 = e_2.get(0).unwrap().parse().unwrap();
        //println!("e2l {}", e_2.get(0).unwrap());
        //println!("e2h {}", e_2.get(1).unwrap());
        let e_2_h: u32 = e_2.get(1).unwrap().parse().unwrap();

        let bound = (min(e_1_l, e_2_l), max(e_1_h, e_2_h));

        if bound == (e_1_l, e_1_h) || bound == (e_2_l, e_2_h) {
            println!("{:?}", bound);
            counter += 1;
        }

        if e_1_l <= e_2_h && e_2_l <= e_1_h {
            overlap_counter += 1;
        }
    }

    println!("Final count of complete overlap is {}", counter);
    println!("final count for overlap is {}", overlap_counter);

    Ok(())
}
