use paste::paste;

use crate::*;

#[test]
fn test_unit() {
    let mut iter = <()>::variants();
    assert_eq!(iter.next(), Some(()));
    assert_eq!(iter.next(), None);
}

#[test]
fn test_bool() {
    let mut iter = bool::variants();
    assert_eq!(iter.next(), Some(false));
    assert_eq!(iter.next(), Some(true));
    assert_eq!(iter.next(), None);
}

macro_rules! test_int {
    ($type:ty) => {
        paste! {
            #[test]
            fn [<test_ $type>]() {
                let mut iter = <$type>::variants();
                for item in <$type>::MIN..=<$type>::MAX {
                    assert_eq!(iter.next(), Some(item));
                }
                assert_eq!(iter.next(), None);
            }
        }
    };
}

test_int!(i8);
test_int!(u8);
test_int!(i16);
test_int!(u16);
