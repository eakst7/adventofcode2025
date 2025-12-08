#![allow(unused_imports)]
#![allow(unused)]

use itertools::Itertools;
use std::collections::HashSet;
use std::fmt::Display;
use std::fs::{ File, read_to_string };
use std::io::{BufRead, BufReader, Read};
use aocutils::ulines::UnwrappedLinesExt;

fn main() {
    let r1 = part1("input.txt", 1000);
    let r2 = part2("input.txt");

    println!("Part 1: {}", r1.unwrap());
    println!("Part 2: {}", r2.unwrap());
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Point3D {
    id: char,
    x: i64,
    y: i64,
    z: i64,
}

// impl(i64, i64, i64);

impl Point3D {
    fn new(x: i64, y: i64, z: i64) -> Self {
        Point3D { id: ' ',x, y, z }
    }

    fn from(s: &str) -> Self {
        let (x, y, z) = s.splitn(3, ",")
            .map(|s| s.parse::<i64>().unwrap())
            .collect_tuple().unwrap();
        Point3D { id: ' ',x, y, z }
    }
}

impl Display for Point3D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // write!(f, "{}({:03}, {:03}, {:03})", self.id, self.x, self.y, self.z)
        write!(f, "{}", self.id)
    }
}


fn distance(p1: &Point3D, p2: &Point3D) -> f64 {
    let dx = p1.x - p2.x;
    let dy = p1.y - p2.y;
    let dz = p1.z - p2.z;
    ((dx.pow(2) + dy.pow(2) + dz.pow(2)) as f64).sqrt()
}

fn part1(input_filename: &str, connections: usize) -> std::io::Result<u64> {
    let file = File::open(input_filename)?;

    let ids = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let mut idnum = 0;
    let mut junction_boxes: Vec<Point3D> = Vec::new();
    let mut circuits: Vec<Vec<Point3D>>= Vec::new();
    file.unwrapped_lines().for_each(|line| {
        let mut point = Point3D::from(&line);
        point.id = ids.chars().nth(idnum).or(Some(' ')).unwrap();
        idnum += 1;
        junction_boxes.push(point);
        circuits.push(vec![point]);
    });
        
    let mut distances: Vec<(Point3D, Point3D, f64)> = junction_boxes.iter()
        .combinations(2).map(|c| 
            (*c[0], *c[1], distance(&c[0], &c[1])))
            .collect();

    distances.sort_by(|(_, _, d1), (_, _, d2)| d1.partial_cmp(d2).unwrap());

    for (p1, p2, d) in distances[0..15].iter() {
        println!("{:.3} {} {}", d, p1, p2);
    }

    for (p1, p2, d) in &distances[0..connections] {
        let i1 = find_circuit(&circuits, p1);
        let i2 = find_circuit(&circuits, p2);

        if i1 != i2 {
            // To append one vector to another, we need mutable access to both.
            // We can't do `&mut circuits[i1]` and `&mut circuits[i2]` at the same time
            // because the borrow checker can't be sure they are different.
            // `split_at_mut` allows us to get two mutable slices safely.
            if i1 < i2 {
                let (head, tail) = circuits.split_at_mut(i2);
                head[i1].append(&mut tail[0]);
            } else { // i2 < i1
                let (head, tail) = circuits.split_at_mut(i1);
                tail[0].append(&mut head[i2]);
            }
        }
        print_circuits(&circuits);
    }

    circuits.sort_by(|c1, c2| { c2.len().partial_cmp(&c1.len()).unwrap() });
    print_circuits(&circuits);


    let r = circuits[0].len() * circuits[1].len() * circuits[2].len();

    Ok(r as u64)
}

fn print_circuits(circuits: &Vec<Vec<Point3D>>) {
    println!("-- CIRCUITS --");    
    for circuit in circuits {
        if circuit.is_empty() {
            continue;
        }
        print!("{}: ", circuit.len());
        for jb in circuit {
            print!("{} ", jb);
        }
        println!();
    }
    println!();
}

fn find_circuit(circuits: &Vec<Vec<Point3D>>, p: &Point3D) -> usize {
    for (i, circuit) in circuits.iter().enumerate() {
        if circuit.contains(p) {
            return i;
        }
    }

    panic!();
}

fn part2(input_filename: &str) -> std::io::Result<u64> {
    let file = File::open(input_filename)?;
    todo!()
}

#[test]
fn test_part1() {
    let r = part1("input.txt", 1000);
    assert!(r.is_ok());
    assert_eq!(r.unwrap(), 40);
}

#[test]
fn test_part2() {
    let r = part2("test.txt");
    assert!(r.is_ok());
    assert_eq!(r.unwrap(), 0);

}

#[test]
fn test_point3d_from() {
    let p = Point3D::from("1,2,3");
    assert_eq!(p, Point3D { id: ' ', x: 1, y: 2, z: 3 });
    assert_eq!(p.x, 1);
    assert_eq!(p.y, 2);
    assert_eq!(p.z, 3);
}

#[test]
fn test_point3d_dist() {
    let p1 = Point3D::from("0,0,0");
    let p2 = Point3D::from("3,4,0");
    assert_eq!(distance(&p1, &p2), 5f64);
}

/*
316.902 A(162, 817, 812) T(425, 690, 689)
321.560 A(162, 817, 812) H(431, 825, 988)
322.369 C(906, 360, 560) N(805, 096, 715)
328.119 H(431, 825, 988) T(425, 690, 689)
333.656 R(862, 061, 035) S(984, 092, 344)
338.339 J(052, 470, 668) M(117, 168, 530)
344.389 L(819, 987, 018) Q(941, 993, 340)
347.599 C(906, 360, 560) I(739, 650, 466)
350.786 O(346, 949, 466) T(425, 690, 689)
352.936 C(906, 360, 560) S(984, 092, 344)
367.982 D(592, 479, 940) T(425, 690, 689)
371.706 E(352, 342, 300) G(542, 029, 236)
372.023 E(352, 342, 300) M(117, 168, 530)
373.411 E(352, 342, 300) F(466, 668, 158)
379.243 G(542, 029, 236) R(862, 061, 035)

-- CIRCUITS --
2: A T 

-- CIRCUITS --
3: A T H 

-- CIRCUITS --
3: A T H 
2: C N 

-- CIRCUITS --
3: A T H 
2: C N 
2: R S 

-- CIRCUITS --
3: A T H 
2: C N 
2: R S 
2: J M 

-- CIRCUITS --
3: A T H 
2: C N 
2: R S 
2: J M 
2: L Q 

-- CIRCUITS --
3: A T H 
3: C N I 
2: R S 
2: J M 
2: L Q 

-- CIRCUITS --
4: A T H O 
3: C N I 
2: R S 
2: J M 
2: L Q 

-- CIRCUITS --
4: A T H O 
4: C N I S 
2: R S 
2: J M 
2: L Q 

-- CIRCUITS --
5: A T H O D 
4: C N I S 
2: R S 
2: J M 
2: L Q 

-- CIRCUITS --
5: A T H O D 
4: C N I S 
2: R S 
2: J M 
2: L Q 

*/