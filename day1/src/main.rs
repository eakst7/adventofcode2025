mod modrem;
mod wrappingint;

use std::fs::File;
use std::io::{
    BufReader,
    BufRead
};

use wrappingint::WrappingInt;


fn main() {
    let r1 = part1();
    let r2 = part2();

    println!("Part 1: {}", r1.unwrap());
    println!("Part 2: {}", r2.unwrap());
}

fn part2() -> std::io::Result<u32> {
    let file = File::open("input.txt")?;
    let mut reader = BufReader::new(file);

    let mut contents = String::new();

    let mut value: WrappingInt = WrappingInt::new(50, 100);

    let mut bytes_read = reader.read_line(&mut contents).unwrap();
    while bytes_read > 0 {
        value = process_line(&contents, value);
        println!("{} {}", contents.trim(), value);
        contents.clear();
        bytes_read = reader.read_line(&mut contents).unwrap();
    }

    println!("Zero count: {}", value.rollover);

    Ok(value.rollover)
}

fn part1() -> std::io::Result<u32> {
    let file = File::open("input.txt")?;
    let mut reader = BufReader::new(file);

    let mut contents = String::new();

    let mut zero_count = 0;
    let mut value: WrappingInt = WrappingInt::new(50, 100);

    let mut bytes_read = reader.read_line(&mut contents).unwrap();
    while bytes_read > 0 {
        value = process_line(&contents, value);
        println!("{} {}", contents.trim(), value);
        if value.value == 0 {
            zero_count += 1;            
        }
        contents.clear();
        bytes_read = reader.read_line(&mut contents).unwrap();
    }

    println!("Zero count: {}", zero_count);

    Ok(zero_count)
}

fn process_line(line: &str, current: WrappingInt) -> WrappingInt {
    let line = line.trim();

    let dir = &line[0..1];
    let value: u32 = line[1..].parse().unwrap();

    match dir {
        "L" => {
            current - value
        }
        "R" => {
            current + value
        }
        _ => {
            panic!("Invalid direction");
        }
    }

    
}
