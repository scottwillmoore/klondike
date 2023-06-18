use std::hint::unreachable_unchecked;
use std::iter::{Iterator, Map};
use std::ops::Range;

pub use enum_trait_derive::Enum;

pub type Variants<T> = Map<Range<usize>, fn(usize) -> T>;

pub trait Enum {
    const LENGTH: usize;

    // Required methods

    unsafe fn from_index_unchecked(index: usize) -> Self;

    fn into_index(self) -> usize;

    // Provided methods

    fn is_index(index: usize) -> bool {
        index < Self::LENGTH
    }

    fn from_index(index: usize) -> Option<Self>
    where
        Self: Sized,
    {
        if Self::is_index(index) {
            Some(unsafe { Self::from_index_unchecked(index) })
        } else {
            None
        }
    }

    fn first() -> Self
    where
        Self: Sized,
    {
        unsafe { Self::from_index_unchecked(0) }
    }

    fn last() -> Self
    where
        Self: Sized,
    {
        unsafe { Self::from_index_unchecked(Self::LENGTH - 1) }
    }

    fn variants() -> Variants<Self>
    where
        Self: Sized,
    {
        (0..Self::LENGTH).map(|index| unsafe { Self::from_index_unchecked(index) })
    }
}

impl Enum for () {
    const LENGTH: usize = 1;

    unsafe fn from_index_unchecked(index: usize) -> Self {
        match index {
            0 => (),
            _ => unreachable_unchecked(),
        }
    }

    fn into_index(self) -> usize {
        0
    }
}

impl Enum for bool {
    const LENGTH: usize = 2;

    unsafe fn from_index_unchecked(index: usize) -> Self {
        match index {
            0 => false,
            1 => true,
            _ => unreachable_unchecked(),
        }
    }

    fn into_index(self) -> usize {
        match self {
            false => 0,
            true => 1,
        }
    }
}

macro_rules! impl_int {
    ($type:ty) => {
        impl Enum for $type {
            const LENGTH: usize = 1 << <$type>::BITS;

            unsafe fn from_index_unchecked(index: usize) -> Self {
                if Self::is_index(index) {
                    (index as $type).wrapping_sub(<$type>::MIN)
                } else {
                    core::hint::unreachable_unchecked();
                }
            }

            fn into_index(self) -> usize {
                self.abs_diff(<$type>::MIN).into()
            }
        }
    };
}

impl_int!(i8);
impl_int!(u8);
impl_int!(i16);
impl_int!(u16);

#[cfg(test)]
mod tests {
    use paste::paste;

    use super::*;

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
}
