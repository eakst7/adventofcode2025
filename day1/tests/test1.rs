use aoc::modrem::ModRem;

#[test]
fn test1() {
    assert_eq!(5u32.mod_rem(3), (1,2));
}