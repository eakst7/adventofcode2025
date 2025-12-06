#![allow(unused_imports)]
use std::fs::{
    File,
};

use std::ops::{
    Range,
};

use aocutils::ulines::UnwrappedLinesExt;
use aocutils::ranges::merge_ranges;

fn main() {
    // let r1 = part1();
    let r2 = part2();

    // println!("Part 1: {}", r1.unwrap());
    println!("Part 2: {}", r2.unwrap());
}

fn part1() -> std::io::Result<u32> {
    let f = File::open("input.txt").expect("Input file not found");

    let mut ranges = Vec::new();
    let mut parsing_ranges = true;
    let mut fresh_count = 0;
    for l in f.unwrapped_lines() {
        if l == "" {
            parsing_ranges = false;
            continue;
        }

        if parsing_ranges {
            let (lo,hi) = l.split_once("-").unwrap();
            let r = Range::<u64> { start: lo.parse::<u64>().unwrap(), end: hi.parse::<u64>().unwrap()+1 };
            dbg!(&r);
            ranges.push(r);
        } else {
            let ingredient = l.parse::<u64>().unwrap();
            for r in ranges.iter() {
                if r.contains(&ingredient) {
                    fresh_count += 1;
                    break;
                }
            }

        }

    }

    Ok(fresh_count)
}

fn part2() -> std::io::Result<u64> {
    let f = File::open("input.txt").expect("Input file not found");

    let mut ranges = Vec::new();
    let mut parsing_ranges = true;
    let mut fresh_count = 0;
    for l in f.unwrapped_lines() {
        if l == "" {
            parsing_ranges = false;
            continue;
        }

        if parsing_ranges {
            let (lo,hi) = l.split_once("-").unwrap();
            let r = Range::<u64> { start: lo.parse::<u64>().unwrap(), end: hi.parse::<u64>().unwrap()+1 };
            dbg!(&r);
            ranges.push(r);
        } else {
            let mut not_done = true;
            while not_done {
                not_done = false;
                'l: for i in 0..ranges.len() {
                    for j in i+1..ranges.len() {
                        let r1 = &ranges[i];
                        let r2 = &ranges[j];

                        let r = merge_ranges(r1, r2);
                        if r.is_ok() {
                            let new_range = r.unwrap();
                            not_done = true;
                            dbg!("Merging ranges", &r1, &r2, &new_range);
                            ranges.remove(i);
                            ranges.remove(j-1);
                            ranges.push(new_range);
                            break 'l;
                        } 
                    }
                }
            }

            dbg!(&ranges);
            for r in ranges {
                fresh_count += r.end - r.start;
            }
            break;
        }

    }

    Ok(fresh_count)
}
