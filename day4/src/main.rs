#![allow(unused_imports)]

use std::{fs::File, io::BufReader, io::BufRead};
use aocutils::ulines::UnwrappedLinesExt;

fn main() {
    // let r1 = part1();
    let r2 = part2();

    // println!("Part 1: {}", r1.unwrap());
    println!("Part 2: {}", r2.unwrap());
}

fn part1() -> std::io::Result<u32> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut v: Vec<Vec<char>> = Vec::new();

    for l in reader.unwrapped_lines() {
        let mut temp = vec!['-'];
        temp.extend(l.chars());
        temp.push('-');
        v.push(temp);
    }
    let cols = v[0].len();
    let temp = vec!['-'; cols];
    v.insert(0, temp.clone());
    v.push(temp);

    dumpv(&v);
    
    count_neighbors(&v, 1, 8);

    Ok(count_accessible_rolls(&mut v))

}

fn part2() -> std::io::Result<u32> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut v: Vec<Vec<char>> = Vec::new();

    for l in reader.unwrapped_lines() {
        let mut temp = vec!['-'];
        temp.extend(l.chars());
        temp.push('-');
        v.push(temp);
    }
    let cols = v[0].len();
    let temp = vec!['-'; cols];
    v.insert(0, temp.clone());
    v.push(temp);

    // dumpv(&v);
    
    let mut removed_rolls = 0;

    let mut accessible_rolls = count_and_mark_accessible_rolls(&mut v);
    while accessible_rolls > 0 {
        // println!("Removed {} rolls\n", accessible_rolls);
        remove_rolls(&mut v);
        // dumpv(&v);

        removed_rolls += accessible_rolls;
        accessible_rolls = count_and_mark_accessible_rolls(&mut v);
    }

    Ok(removed_rolls)

}


fn dumpv(v: &Vec<Vec<char>>) {
    for l in v {
        println!("{:?}", l);
    }
}

fn count_accessible_rolls(v: &mut Vec<Vec<char>>) -> u32 {
    let mut accessible_rolls = 0;

    for row in 1..v.len()-1 {
        for col in 1..v[0].len()-1 {

            if v[row][col] == '@' && count_neighbors(v, row, col) < 4 {
                // println!("{},{} is accessible", row, col);
                accessible_rolls += 1;
            }
        }
    }

    accessible_rolls
}

fn count_and_mark_accessible_rolls(v: &mut Vec<Vec<char>>) -> u32 {
    let mut accessible_rolls = 0;

    for row in 1..v.len()-1 {
        for col in 1..v[0].len()-1 {

            if v[row][col] == '@' && count_neighbors(v, row, col) < 4 {
                // println!("{},{} is accessible", row, col);
                accessible_rolls += 1;
                v[row][col] = 'X';
            }
        }
    }

    accessible_rolls
}


fn count_neighbors(v: &Vec<Vec<char>>, r: usize, c: usize) -> usize {
    let mut neighbor_count = 0usize;
    for row in r-1..=r+1 {
        for col in c-1..=c+1 {
            if row == r && col == c {
                continue;
            }
            if v[row][col] == '@' || v[row][col] == 'X' {
                neighbor_count += 1;
            }
        }
    }
    return neighbor_count;
}

fn remove_rolls(v: &mut Vec<Vec<char>>) {
    for row in 1..v.len()-1 {
        for col in 1..v[0].len()-1 {
            if v[row][col] == 'X' {
                v[row][col] = '.';
            }
        }
    }
}
