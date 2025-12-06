use std::fmt;
use std::ops::Range;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MergeError {
    /// Used when two items cannot be merged, for example, if two ranges do not overlap or are not adjacent.
    NotMergeable,
}

impl fmt::Display for MergeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MergeError::NotMergeable => write!(f, "Items are not mergeable"),
        }
    }
}

pub trait Mergeable {
    fn merge(&mut self, other: &Self) -> Result<(), MergeError>;

}   

impl<Idx: PartialOrd + Copy> Mergeable for Range<Idx> {
    fn merge(&mut self, other: &Self) -> Result<(), MergeError> {
        fn overlaps<Idx: PartialOrd>(r1: &Range<Idx>, r2: &Range<Idx>) -> bool {
            r1.contains(&r2.start) || r1.contains(&r2.end) || r2.contains(&r1.start) || r2.contains(&r1.end)
        }

        if !overlaps(self, other) {
            return Err(MergeError::NotMergeable)
        }
        
        self.start = if self.start < other.start { self.start } else { other.start };
        self.end = if self.end > other.end { self.end} else {other.end };
        Ok(())
    }
}

pub fn ranges_overlap<Idx: PartialOrd>(r1: &Range<Idx>, r2: &Range<Idx>) -> bool {
    r1.contains(&r2.start) || r1.contains(&r2.end) || r2.contains(&r1.start) || r2.contains(&r1.end)
}

pub fn merge_ranges<Idx: PartialOrd + Copy>(r1: &Range<Idx>, r2: &Range<Idx>) -> Result<Range<Idx>, MergeError> {
    if !ranges_overlap(r1, r2) {
        return Err(MergeError::NotMergeable)
    }

    Ok(Range {
        start: if r1.start < r2.start { r1.start } else { r2.start },
        end: if r1.end > r2.end { r1.end} else {r2.end }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

#[test]
fn test_merge_ok_1() {
    let mut r1 = Range { start: -100i32, end: 300 };
    let r2 = Range { start: 200, end: 400 };
    r1.merge(&r2).unwrap();
    assert_eq!(r1, Range { start: -100, end: 400 });
}

#[test]
fn test_merge_ok_2() {
    let mut r1 = Range { start: 100u32, end: 300 };
    let r2 = Range { start: 0, end: 200 };
    r1.merge(&r2).unwrap();
    assert_eq!(r1, Range { start: 0, end: 300 });
}

#[test]
fn test_merge_ok_3() {
    let mut r1 = Range { start: 100u64, end: 300 };
    let r2 = Range { start: 150, end: 200 };
    r1.merge(&r2).unwrap();
    assert_eq!(r1, Range { start: 100, end: 300 });
}

#[test]
fn test_merge_ok_4() {
    let mut r1 = Range { start: 1u8, end: 99 };
    let r2 = Range { start: 99, end: 200 };
    r1.merge(&r2).unwrap();
    assert_eq!(r1, Range { start: 1, end: 200 });
}

#[test]
fn test_merge_unmergeable_1() {
    let mut r1 = Range { start: 1, end: 99 };
    let r2 = Range { start: 100, end: 200 };
    let result = r1.merge(&r2);
    assert_eq!(result, Err(MergeError::NotMergeable));
}

}