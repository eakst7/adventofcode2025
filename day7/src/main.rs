#![allow(unused_imports)]
#![allow(unused)]
use std::collections::{ HashSet };
use std::fs::{ File, read_to_string };
use std::io::{BufRead, BufReader, Read};
use aocutils::ulines::UnwrappedLinesExt;

fn main() {
    let r1 = part1("day7/input.txt");
    let r2 = part2("day7/input.txt");

    println!("Part 1: {}", r1.unwrap());
    println!("Part 2: {}", r2.unwrap());
}

fn part1(input_filename: &str) -> std::io::Result<u64> {
    let file = File::open(input_filename)?;

    let mut split_total = 0;
    let mut beams: HashSet<u64> = HashSet::new();
    let mut splitters: HashSet<u64> = HashSet::new();
    for line in file.unwrapped_lines() {
        splitters.clear();
        if beams.is_empty() {
            beams.insert(line.find('S').unwrap() as u64);
            continue;
        }
        line.chars().enumerate().for_each(|(i, c)| {
            if c == '^' { splitters.insert(i as u64); }
        });

        let r = split_beams(&beams, &splitters);
        split_total += r.0;
        beams = r.1;
    }

    Ok(split_total as u64)
}

fn part2(input_filename: &str) -> std::io::Result<u64> {
    let file = File::open(input_filename)?;
    todo!()
}

fn split_beams(beams: &HashSet<u64>, splitters: &HashSet<u64>) -> (u64,HashSet<u64>) {
    let mut split_count = 0;
    let mut new_beams = HashSet::new();
    for beam in beams {
        if splitters.contains(&beam) {
            split_count += 1;
            new_beams.insert(beam-1);
            new_beams.insert(beam+1);
        } else {
            new_beams.insert(*beam);
        }
    }
    (split_count, new_beams)
}

#[test]
fn test_part1() {
    let r = part1("test.txt");
    assert!(r.is_ok());
    assert_eq!(r.unwrap(), 21);
}

#[test]
fn test_part2() {
    let r = part2("test.txt");
    assert!(r.is_ok());
    assert_eq!(r.unwrap(), 0);

}

#[test]
fn test_split_beams_1() {
    let beams = HashSet::from([8]);
    let splitters = HashSet::from([]);
    let (count, new_beams) = split_beams(&beams, &splitters);
    assert_eq!(new_beams, HashSet::from([8]));
    assert_eq!(count, 0);
}

#[test]
fn test_split_beams_2() {
    let beams = HashSet::from([8]);
    let splitters = HashSet::from([8]);
    let (count, new_beams) = split_beams(&beams, &splitters);
    assert_eq!(new_beams, HashSet::from([7,9]));
    assert_eq!(count, 1);
}

#[test]
fn test_split_beams_3() {
    let beams = HashSet::from([7,9]);
    let splitters = HashSet::from([7,9]);
    let (count, new_beams) = split_beams(&beams, &splitters);
    assert_eq!(new_beams, HashSet::from([6,8,10]));
    assert_eq!(count, 2);
}
