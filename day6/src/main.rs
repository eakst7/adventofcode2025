#![allow(unused_imports)]
#![allow(unused)]
use std::fs::{ File, read_to_string };
use std::io::{BufRead, BufReader};
use aocutils::ulines::UnwrappedLinesExt;

fn main() {
    let r1 = part1();
    // let r2 = part2();

    println!("Part 1: {}", r1.unwrap());
    // println!("Part 2: {}", r2.unwrap());
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

    Ok(0)
}
