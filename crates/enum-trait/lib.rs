use std::iter::{Iterator, Map};
use std::ops::Range;

pub use enum_trait_derive::Enum;

pub trait Enum {
    const LENGTH: usize;

    unsafe fn from_index_unchecked(index: usize) -> Self;

    fn into_index(self) -> usize;
}

impl Enum for () {
    const LENGTH: usize = 1;

    unsafe fn from_index_unchecked(index: usize) -> Self {
        match index {
            0 => (),
            _ => core::hint::unreachable_unchecked(),
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
            _ => core::hint::unreachable_unchecked(),
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

type Variants<T> = Map<Range<usize>, fn(usize) -> T>;

pub fn variants<T>() -> Variants<T>
where
    T: Enum,
{
    (0..T::LENGTH).map(|index| unsafe { T::from_index_unchecked(index) })
}

#[cfg(test)]
mod tests {
    use paste::paste;

    use super::*;

    #[test]
    fn test_unit() {
        let mut iter = variants::<()>();
        assert_eq!(iter.next(), Some(()));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_bool() {
        let mut iter = variants::<bool>();
        assert_eq!(iter.next(), Some(false));
        assert_eq!(iter.next(), Some(true));
        assert_eq!(iter.next(), None);
    }

    macro_rules! test_uint {
        ($type:ty) => {
            paste! {
                #[test]
                fn [<test_variants_ $type>]() {
                    let mut iter = variants::<u8>();
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

pub unsafe fn first_unchecked<T>() -> T
where
    T: Enum,
{
    debug_assert!(T::LENGTH > 0);
    T::from_index_unchecked(0)
}

pub unsafe fn last_unchecked<T>() -> T
where
    T: Enum,
{
    debug_assert!(T::LENGTH > 0);
    T::from_index_unchecked(T::LENGTH - 1)
}
