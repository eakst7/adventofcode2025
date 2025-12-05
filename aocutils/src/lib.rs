#![allow(unused_imports)]

mod mod1;


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

use std::path::{
    Path,
};

use std::fmt::{
    Display,
};

pub struct UnwrappedLines<R> {
    lines: Lines<R>
}

impl<R: BufRead> Iterator for UnwrappedLines<R> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        self.lines.next().map(|l| l.unwrap())
    }
}

pub trait UnwrappedLinesExt: Read + Sized {
    fn unwrapped_lines(self) -> UnwrappedLines<BufReader<Self>>;
}

impl<R: Read> UnwrappedLinesExt for R {
    fn unwrapped_lines(self) -> UnwrappedLines<BufReader<Self>> {
        let buf_reader = BufReader::new(self);
        UnwrappedLines { lines: buf_reader.lines() }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unwrapped_lines_file() {
        let file = File::open("input.txt").unwrap();

        for line in file.unwrapped_lines() {
            println!("{}", line);
        }
    }

    #[test]
    fn test_file_1() {
        let file = File::open("input.txt").unwrap();
        let buffered_reader = BufReader::new(file);
        
        for line in buffered_reader.lines() {
            println!("{}", line.unwrap());
        }
    }

    #[test]
    fn test_file_2() {
        let file = File::open("input.txt").unwrap();
        let buffered_reader = BufReader::new(file);
        
        for line in buffered_reader.lines().map(|l| l.unwrap()) {
            println!("{}", line);
        }
    }

    #[test]
    fn test_file_3() {
        let file = File::open("input.txt").unwrap();
        
        for line in file.unwrapped_lines() {
            println!("{}", line);
        }
    }


    #[test]
    fn test_string_1() {
        let str = String::from("Hello from String!\nLine2\n");
        let buffered_reader = BufReader::new(Cursor::new(str.as_bytes()));

        for line in buffered_reader.lines() {
            println!("{}", line.unwrap());
        }

    }
    
    #[test]
    fn test_cursor_1() {
        let str = String::from("Hello from String!\nLine2\n");
        let cursor = Cursor::new(str.as_bytes());

        for line in cursor.unwrapped_lines() {
            println!("{}", line);
        }

    }

   #[test]
    fn test_cursor_2() {
        let v: Vec<u8> = vec![49, 50, 51, 52]; // ASCII codes for '1', '2', '3', '4'
        let cursor = Cursor::new(v);

        for line in cursor.unwrapped_lines() {
            println!("{}", line);
        }

    }

}