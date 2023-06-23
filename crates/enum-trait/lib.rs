#[cfg(test)]
mod test;

use std::iter::{Iterator, Map};
use std::ops::Range;

pub use enum_trait_derive::Enum;

pub type Variants<T> = Map<Range<usize>, fn(usize) -> T>;

pub trait Enum {
    const LENGTH: usize;

    // Required methods

    fn from_index(index: usize) -> Option<Self>
    where
        Self: Sized;

    fn to_index(self) -> usize;

    // Provided methods

    fn first() -> Option<Self>
    where
        Self: Sized,
    {
        Self::from_index(0)
    }

    fn last() -> Option<Self>
    where
        Self: Sized,
    {
        Self::from_index(Self::LENGTH - 1)
    }

    fn previous(self) -> Option<Self>
    where
        Self: Sized,
    {
        self.to_index().checked_sub(1).and_then(Self::from_index)
    }

    fn next(self) -> Option<Self>
    where
        Self: Sized,
    {
        self.to_index().checked_add(1).and_then(Self::from_index)
    }

    fn variants() -> Variants<Self>
    where
        Self: Sized,
    {
        (0..Self::LENGTH).map(|index| Self::from_index(index).unwrap())
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

    fn to_index(self) -> usize {
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

    fn to_index(self) -> usize {
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

            fn from_index(index: usize) -> Option<Self> {
                if index < Self::LENGTH {
                    Some((index as $type).wrapping_sub(<$type>::MIN))
                } else {
                    None
                }
            }

            fn to_index(self) -> usize {
                self.abs_diff(<$type>::MIN).into()
            }
        }
    };
}

impl_int!(i8);
impl_int!(u8);

impl_int!(i16);
impl_int!(u16);
