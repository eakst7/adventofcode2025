#![allow(unused_imports)]
#![allow(unused)]
use std::fs::{ File, read_to_string };
use std::io::{BufRead, BufReader, Read};
use aocutils::ulines::UnwrappedLinesExt;

fn main() {
    // let r1 = part1();
    let r2 = part2();

    // println!("Part 1: {}", r1.unwrap());
    println!("Part 2: {}", r2.unwrap());
}

fn part1() -> std::io::Result<u64> {
    let input = read_to_string("input.txt").unwrap();
    let input = input.split("\n").collect::<Vec<&str>>();

    let values = &input[0..input.len()-2];
    let ops = &input[input.len()-2];

    let mut value_vecs: Vec<Vec<u64>> = Vec::new();
    for v in values {
        value_vecs.push(v.split_whitespace().map(|x| x.parse::<u64>().unwrap()).collect::<Vec<u64>>());
    }
    let ops_vec: Vec<char> = ops.split_whitespace().map(|x| x.parse::<char>().unwrap()).collect();

    // dbg!(&value_vecs);

    let mut total = 0;
    for (i,op) in ops_vec.iter().enumerate() {
        let mut column_total = 0;
        for value_vec in &value_vecs {
            match op {
                '+' => {
                    column_total += value_vec[i];
                },
                '*' => {
                    if column_total == 0 {
                        column_total = value_vec[i];
                    } else {
                        column_total *= value_vec[i];
                    }
                }
                _ => panic!()
            }
        }

        println!("{}: {}", i, column_total);
        total += column_total;
    }

    Ok(total)
}

fn part2() -> std::io::Result<u64> {
    let mut f = File::open("input2.txt").unwrap();
    let mut b = BufReader::new(f);
    let mut filedata: Vec<u8> = Vec::new();

    let mut ops: String= String::new();
    b.read_line(&mut ops).unwrap();

    let ops: Vec<char> = ops.split_whitespace().map(|x| x.chars().nth(0).unwrap()).collect();
    dbg!(&ops);

    b.read_to_end(&mut filedata);

    let mut column_vecs: Vec<Vec<char>> = Vec::new();

    let mut i = 0;
    for b in filedata {
        if b != b'\n' {
            if column_vecs.len() <= i {
                column_vecs.push(Vec::new());
            }

            column_vecs[i].push(b as char);
            i = i + 1;
        } else {
            i = 0;
        }
    }
    
    dbg!(&column_vecs[0]);

    let mut group = 0;
    let mut total = 0;
    let mut group_total = 0;
    for col in &column_vecs {
        dbg!(group);
        let s: String = col.iter().collect::<String>().trim().to_string();

        if s.is_empty() {
            dbg!(group_total);
            group += 1;
            total += group_total;
            group_total = 0;
            continue;
        }

        let op = ops[group];
        let i: u64 = s.parse().unwrap();
        // dbg!(i, op);

        if op == '*' {
            if group_total == 0 {
                group_total = i;
            } else {
                group_total *= i;
            }
        } else if op == '+' {
            group_total += i;
        } else {
            panic!();
        }

    }
    total += group_total;
    dbg!(&total);
    Ok(total)
}
