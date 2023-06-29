use enum_trait::Enum;

use crate::*;

#[derive(Clone, Copy, Debug, Enum, Eq, PartialEq)]
enum Test {
    A,
    B,
    C,
}

#[test]
fn test_enum_map() {
    let enum_map = EnumMap::<Test, usize>::default();
}
