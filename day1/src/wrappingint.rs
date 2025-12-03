use crate::modrem::ModRem;
use std::fmt::Display;
use std::ops::{Add, Sub};

#[derive(Debug)]
#[derive(Clone)]
#[derive(Copy)]
pub struct WrappingInt {
    pub value: u32,
    pub wrap_at: u32,
    pub rollover: u32,
}

impl WrappingInt {
    pub fn new(value: u32, wrap_at: u32) -> WrappingInt {
        if wrap_at == 0 {
            panic!("wrap_at cannot be 0");
        }
        WrappingInt { value, wrap_at, rollover: 0 }
    }
}

impl Display for WrappingInt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Value: {} Rollover: {}", self.value, self.rollover)
    }
}

impl ModRem for WrappingInt {
    fn mod_rem(&self, b: u32) -> (u32, u32) {
        let m = self.value / b;
        let r = self.value - (m * b);
        (m, r)
    }
}

impl Add<u32> for WrappingInt {
    type Output = WrappingInt;

    fn add(self, other: u32) -> WrappingInt {
        let mut rollover = self.rollover;

        let (m,r) = other.mod_rem(self.wrap_at);
        rollover += m;

        let mut v = self.value + r;
        if v >= self.wrap_at {
            v -= self.wrap_at;
            rollover += 1;
        }

        WrappingInt { 
            value: v,
            wrap_at: self.wrap_at,
            rollover
        }
    }
}

impl Sub<u32> for WrappingInt {
    type Output = WrappingInt;

    fn sub(self, other: u32) -> WrappingInt {
        let mut rollover = self.rollover;

        let (m,r) = other.mod_rem(self.wrap_at);
        rollover += m;

        let mut v: i32 = self.value as i32 - r as i32;
        if v < 0 {
            v += self.wrap_at as i32;
            if self.value > 0 {
                rollover += 1;
            }
        } else if v == 0 {
            rollover += 1;
        }

        WrappingInt { 
            value: v as u32,
            wrap_at: self.wrap_at,
            rollover 
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let mut x = WrappingInt::new(50, 100);
        x = x + 10;
        assert_eq!(x.value, 60);
        assert_eq!(x.rollover, 0);
    }

    #[test]
    fn test_add2() {
        let mut x = WrappingInt::new(50, 100);
        x = x + 1000;
        assert_eq!(x.value, 50);
        assert_eq!(x.rollover, 10);
    }

    #[test]
    fn test_add3() {
        let mut x = WrappingInt::new(50, 100);
        x = x + 50;
        assert_eq!(x.value, 0);
        assert_eq!(x.rollover, 1);
    }

    #[test]
    fn test_add4() {
        let mut x = WrappingInt::new(0, 100);
        x = x + 50;
        assert_eq!(x.value, 50);
        assert_eq!(x.rollover, 0);
    }

    #[test]
    fn test_sub() {
        let mut x = WrappingInt::new(50, 100);
        x = x - 10;
        assert_eq!(x.value, 40);
        assert_eq!(x.rollover, 0);
    }

    #[test]
    fn test_sub2() {
        let mut x = WrappingInt::new(50, 100);
        x = x - 1000;
        assert_eq!(x.value, 50);
        assert_eq!(x.rollover, 10);
    }

    #[test]
    fn test_sub3() {
        let mut x = WrappingInt::new(50, 100);
        x = x - 50;
        assert_eq!(x.value, 0);
        assert_eq!(x.rollover, 1);
    }

    #[test]
    fn test_sub4() {
        let mut x = WrappingInt::new(0, 100);
        x = x - 50;
        assert_eq!(x.value, 50);
        assert_eq!(x.rollover, 0);
    }

    #[test]
    fn test_sub4_2() {
        let mut x = WrappingInt::new(1, 100);
        x = x - 50;
        assert_eq!(x.value, 51);
        assert_eq!(x.rollover, 1);
    }


    #[test]
    fn test_sub5() {
        let mut x = WrappingInt::new(0, 100);
        x = x - 50;
        x = x - 50;
        assert_eq!(x.value, 0);
        assert_eq!(x.rollover, 1);
    }

    #[test]
    fn test_sub6() {
        let mut x = WrappingInt::new(50, 100);
        x = x - 68;
        assert_eq!(x.value, 82);
        assert_eq!(x.rollover, 1);
    }


    #[test]
    fn test_1() {
        let mut x = WrappingInt::new(50, 100);
        x = x + 1000;
        x = x - 1000;
        x = x + 50;
        x = x - 50;
        assert_eq!(x.value, 50);
        assert_eq!(x.rollover, 21);
    }

    #[test]
    fn test_2() {
        let mut x = WrappingInt::new(0, 1);
        x = x + 1 + 1 + 1 + 1;
        assert_eq!(x.value, 0);
        assert_eq!(x.rollover, 4);
    }

    #[test]
    #[should_panic]
    fn test_should_panic() {
        let mut x = WrappingInt::new(0, 0);
        x = x + 1 + 1 + 1 + 1;
        assert_eq!(x.value, 0);
        assert_eq!(x.rollover, 4);
    }

}