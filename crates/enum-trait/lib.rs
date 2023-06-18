use std::hint::unreachable_unchecked;
use std::iter::{Iterator, Map};
use std::ops::Range;

pub use enum_trait_derive::Enum;

pub type Variants<T> = Map<Range<usize>, fn(usize) -> T>;

pub trait Enum {
    const LENGTH: usize;

    fn from_index(index: usize) -> Option<Self>
    where
        Self: Sized;

    unsafe fn from_index_unchecked(index: usize) -> Self;

    fn into_index(self) -> usize;

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

    fn from_index(index: usize) -> Option<Self> {
        match index {
            0 => Some(()),
            _ => None,
        }
    }

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

    fn from_index(index: usize) -> Option<Self> {
        match index {
            0 => Some(false),
            1 => Some(true),
            _ => None,
        }
    }

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

// Should this be extended to support signed integers?
macro_rules! impl_uint {
    ($type:ty) => {
        impl Enum for $type {
            const LENGTH: usize = 1 << <$type>::BITS;

            fn from_index(index: usize) -> Option<Self> {
                if index <= <$type>::MAX as usize {
                    Some(index as $type)
                } else {
                    None
                }
            }

            unsafe fn from_index_unchecked(index: usize) -> Self {
                if index <= <$type>::MAX as usize {
                    index as $type
                } else {
                    core::hint::unreachable_unchecked();
                }
            }

            fn into_index(self) -> usize {
                self as usize
            }
        }
    };
}

// Should this be implemented for larger integers?
impl_uint!(u8);
impl_uint!(u16);

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

    macro_rules! test_uint {
        ($type:ty) => {
            paste! {
                #[test]
                fn [<test_variants_ $type>]() {
                    let mut iter = u8::variants();
                    for item in u8::MIN..=u8::MAX {
                        assert_eq!(iter.next(), Some(item));
                    }
                    assert_eq!(iter.next(), None);
                }
            }
        };
    }

    test_uint!(u8);
    test_uint!(u16);
}
