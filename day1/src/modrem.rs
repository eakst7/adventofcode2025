pub trait ModRem {
    fn mod_rem(&self, b: u32) -> (u32, u32);
}

impl ModRem for u32 {
    fn mod_rem(&self, b: u32) -> (u32, u32) {
        let m = *self / b;
        let r = *self - (m * b);
        (m, r)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_mod_rem_u32() {
        assert_eq!(10_u32.mod_rem(5), (2, 0));
        assert_eq!(10_u32.mod_rem(1), (10, 0));
        assert_eq!(10_u32.mod_rem(3), (3, 1));

        assert_eq!(10_u32.mod_rem(11), (0, 10));

        assert_eq!(0_u32.mod_rem(5), (0, 0));
        assert_eq!(1_u32.mod_rem(5), (0, 1));
    }
}
