use std::mem::size_of;

use paste::paste;

use crate::*;

// NOTE: A `[T; N]` has a size of `N * size_of::<T>()`.
// https://doc.rust-lang.org/std/mem/fn.size_of.html

macro_rules! assert_size {
    ($type:ty) => {
        assert_eq!(
            size_of::<<$type as EnumArray<u8>>::Array>(),
            size_of::<u8>() * <$type>::LENGTH
        );
    };
}

#[test]
fn test_unit() {
    assert_eq!(<()>::LENGTH, 1);

    let mut iter = <()>::variants();
    assert_eq!(iter.next(), Some(()));
    assert_eq!(iter.next(), None);

    assert_size!(());
}

#[test]
fn test_bool() {
    assert_eq!(bool::LENGTH, 2);

    let mut iter = bool::variants();
    assert_eq!(iter.next(), Some(false));
    assert_eq!(iter.next(), Some(true));
    assert_eq!(iter.next(), None);

    assert_size!(bool);
}

macro_rules! test_int {
    ($type:ty) => {
        paste! {
            #[test]
            fn [<test_ $type>]() {
                assert_eq!(<$type>::LENGTH, 1 << <$type>::BITS);

                let mut iter = <$type>::variants();
                for item in <$type>::MIN..=<$type>::MAX {
                    assert_eq!(iter.next(), Some(item));
                }
                assert_eq!(iter.next(), None);

                assert_size!($type);
            }
        }
    };
}

test_int!(i8);
test_int!(u8);
