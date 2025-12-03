use std::fs::File;
use std::io::{
    // BufRead,
    BufReader, Read
};

fn main() {
    let r1 = part1();
    // let r2 = part2();

    println!("Part 1: {}", r1.unwrap());
    // println!("Part 2: {}", r2.unwrap());
}

fn part1() -> std::io::Result<u64> {
    let mut reader = BufReader::new(File::open("input.txt")?);
    let mut contents = String::new();
    reader.read_to_string(&mut contents)?;

    let mut matches_sum = 0u64;
    for range in contents.trim().split(",") {
        matches_sum += process_range(range);
    }

    Ok(matches_sum)
}

fn process_range(range: &str) -> u64 {
    let mut matches = 0u64;
    let (lo, hi) = range.split_once("-").unwrap();
    let lo: u64 = lo.parse().unwrap();
    let hi: u64 = hi.parse().unwrap();

    for i in lo..=hi {
        let istr = i.to_string();

        if istr.len() % 2 != 0 {
            continue;
        }

        let (left,right) = istr.split_at(istr.len() / 2);

        // println!("{} {} {}", i, left, right);

        if left == right {
            println!("{} {} {}", i, left, right);
            matches += i;
        }
    }

    println!("{}", matches);
    matches
}