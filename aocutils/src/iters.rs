pub struct SlidingWindows<I,T>
where 
  I: Iterator<Item = T>
{
    iter: I
}

impl<I,T> Iterator for SlidingWindows<I,T>
where
  I: Iterator<Item = T> {
    type Item = (T, T);
    
    fn next(&mut self) -> Option<Self::Item> {

        if let Some(c1) = self.iter.next() {
            if let Some(c2) = self.iter.next() {
                return Some((c1, c2));
            }
        }

        None
    }
}

pub trait SlidingWindowsExt<I,T>
where I: Iterator<Item = T>
{
    fn sliding_windows(self) -> SlidingWindows<I,T>;
}

impl<I,T> SlidingWindowsExt<I,T> for I
where I: Iterator<Item = T> {
    fn sliding_windows(self: I) -> SlidingWindows<I,T> {
        SlidingWindows { iter: self }
    }
}

#[cfg(test)]
mod tests { 
use super::*;

#[test]
fn test1() {
    let v = "abcd";

    let mut it = SlidingWindows { iter: v.chars() };

    assert_eq!(it.next().unwrap(), ('a', 'b'));
    assert_eq!(it.next().unwrap(), ('c', 'd'));
}

#[test]
fn test2() {
    let v = vec![1, 2, 3, 4];

    let mut it = SlidingWindows { iter: v.iter() };

    let (&a, &b) = it.next().unwrap();

    assert_eq!((a, b), (1, 2));
    assert_eq!(it.next().unwrap(), (&3, &4));
}

#[test]
fn test3() {
    let v = vec![1, 2, 3, 4];

    let c = v.iter().sliding_windows().collect::<Vec<(&u32, &u32)>>();
    assert_eq!(c, vec![(&1, &2), (&3, &4)]);
}

} // End mod tests