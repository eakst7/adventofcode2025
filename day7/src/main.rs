#![allow(unused_imports)]
#![allow(unused)]
use std::collections::{ HashSet, HashMap };
use std::fs::{ File, read_to_string };
use std::io::{BufRead, BufReader, Read};
use std::thread::sleep;
use std::time::Duration;
use aocutils::ulines::UnwrappedLinesExt;

static mut CACHE_HITS: u32 = 0;

fn main() {
    let r1 = part1("input.txt");
    let r2 = part2("input.txt");

    println!("Part 1: {}", r1.unwrap());
    println!("Part 2: {}", r2.unwrap());
    println!("  Cache hits: {}", unsafe { CACHE_HITS });
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

    let lines: Vec<String> = file.unwrapped_lines().collect();
    let mut m: HashMap<(usize,usize), u64> = HashMap::new();
    let mut splitter_cache: HashMap<usize, HashSet<usize>> = HashMap::new();
    let mut beam: usize = lines[0].find('S').unwrap();

    let total = take_path(&mut m, &mut splitter_cache, &lines, beam, 1, 0);

    Ok(total as u64)
}

fn take_path(
        cache: &mut HashMap<(usize,usize), u64>,
        splitter_cache: &mut HashMap<usize, HashSet<usize>>,
        lines: &[String],
        beam: usize,
        row: usize,
        call_depth: u8) -> u64 {
    if lines.len() <= row { return 1; }
    visualize(lines, beam, row);

    let r = cache.get(&(beam, row));
    if r.is_some() {
        unsafe { CACHE_HITS += 1; }
        return *r.unwrap(); 
    }

    let splitters = splitter_cache.get(&row);
    let mut splitters = match splitters {
        Some(s) => {
            s
        },
        None => {
            let mut s = HashSet::new();
            lines[row].chars().enumerate().for_each(|(i, c)| {
                if c == '^' { s.insert(i); }
            });
            splitter_cache.insert(row, s);
            splitter_cache.get(&row).unwrap()
        }
    };

    if splitters.contains(&beam) {
        let r1 = take_path(cache, splitter_cache, &lines, beam-1, row+1, call_depth+1);
        cache.insert((beam, row), r1);
        let r2 = take_path(cache, splitter_cache, &lines, beam+1, row+1, call_depth+1);
        cache.insert((beam, row), r2);
        // dbg!(call_depth);
        return r1 + r2;
    } else {
        let r = take_path(cache, splitter_cache, &lines, beam, row+1, call_depth+1);    
        cache.insert((beam, row), r);
        // dbg!(call_depth);
        return r;
    }
}

fn visualize(lines: &[String], beam: usize, row: usize) {
    // print!("{esc}[2J{esc}[1;1H", esc = 27 as char);

    // let mut lines2: Vec<String> = lines.iter().map(|l| l.clone()).collect();
    // let mut s = String::new();
    // s.push_str(&lines2[row][0..beam]);
    // s.push('|');
    // s.push_str(&lines2[row][beam + 1..]);
    // lines2[row] = s;
    // for l in lines2 {
    //     println!("{}", l);
    // }
    // sleep(Duration::from_millis(5));
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
    assert_eq!(r.unwrap(), 40);

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
