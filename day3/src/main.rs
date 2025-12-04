use std::{fs::File, io::BufReader, io::BufRead};


fn main() {
    // let r1 = part1();
    let r2 = part2();

    // println!("Part 1: {}", r1.unwrap());
    println!("Part 2: {}", r2.unwrap());
}

fn part1() -> std::io::Result<u32> {
    let file = File::open("input.txt")?;
    let mut reader = BufReader::new(file);
    let mut contents = String::new();

    let mut value = 0;

    let mut bytes_read = reader.read_line(&mut contents).unwrap();
    while bytes_read > 0 {
        value += process_line(&contents.trim());
        println!("{} {}", contents.trim(), value);
        contents.clear();
        bytes_read = reader.read_line(&mut contents).unwrap();
    }

    Ok(value)
}

fn part2() -> std::io::Result<u64> {
    let file = File::open("input.txt")?;
    let mut reader = BufReader::new(file);
    let mut contents = String::new();

    let mut value = 0;

    let mut bytes_read = reader.read_line(&mut contents).unwrap();
    while bytes_read > 0 {
        value += process_line_part2(12, &contents.trim());
        println!("{} {}", contents.trim(), value);
        contents.clear();
        bytes_read = reader.read_line(&mut contents).unwrap();
    }

    Ok(value)
}


fn process_line(line: &str) -> u32 {

    let (d1, p1) = find_highest(&line[..line.len()-1]);
    let (d2, _) = find_highest(&line[(p1+1)..]);

    d1 * 10 + d2
}




fn find_highest(line: &str) -> (u32,usize) {
    let mut highest_pos: i32 = -1;
    let mut highest = 0;

    for (pos,c) in line.chars().enumerate() {
        let v = c.to_digit(10).unwrap();
        if v > highest {
            highest_pos = pos as i32;
            highest = v;
        }
    }
    println!("{} {} {}", line, highest, highest_pos);
    (highest, highest_pos as usize)
}

fn process_line_part2(digits: usize, line: &str) -> u64 {
    let digits1: usize = digits - 1;

    let mut v: u64 = 0;

    let mut start = 0;
    for i in 0..=digits1 {
        let (d, p) = find_highest_part2(&line, start, line.len() - (digits1-i));
        start = p + 1;

        v += (d as u64 * 10u64.pow((digits1-i) as u32)) as u64;
    }

    v
}

fn find_highest_part2(line: &str, start: usize, end: usize) -> (u32,usize) {
    let mut highest_pos: usize = 0;
    let mut highest: i32 = -1;

    for (pos,c) in line[start..end].chars().enumerate() {
        let v = c.to_digit(10).unwrap();
        if v as i32 > highest {
            highest_pos = pos+start;
            highest = v as i32;
        }
    }
    // println!("{} {} {}", line, highest, highest_pos);
    (highest as u32, highest_pos as usize)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_highest() {
        assert_eq!(find_highest("12345"), (5, 4));
    }

    #[test]
    fn test_process_line() {
        assert_eq!(process_line("12345"), 45);
        assert_eq!(process_line("81994"), 99);
        assert_eq!(process_line("81949"), 99);
        assert_eq!(process_line("81749"), 89);
    }

    #[test]
    fn test_process_line_part2() {
        assert_eq!(process_line_part2(4, "123456789012345"), 9345);
        assert_eq!(process_line_part2(5, "123456789012345"), 92345);
        assert_eq!(process_line_part2(6, "123456789012345"), 912345);
        assert_eq!(process_line_part2(7, "123456789012345"), 9012345);
        assert_eq!(process_line_part2(8, "123456789012345"), 89012345);

        assert_eq!(process_line_part2(8, "129456789012345"), 99012345);
    }
}