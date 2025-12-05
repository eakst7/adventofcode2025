use crate::UnwrappedLinesExt;

use std::io::{
    Read,
    BufRead,
    BufReader,
    Cursor,
    Lines
};

use std::fs::{
    File,
};


#[test]
fn test_a1() {
    let file = File::open("input.txt").unwrap();
    let buffered_reader = BufReader::new(file);
    
    for line in buffered_reader.lines() {
        println!("{}", line.unwrap());
    }
}

#[test]
fn test_a2() {
    let file = File::open("input.txt").unwrap();
    
    for line in file.unwrapped_lines() {
        println!("{}", line);
    }
}
