//! TODO: Write documentation.

#[cfg(test)]
mod test;

use std::hint::unreachable_unchecked;
use std::iter::{Iterator, Map};
use std::ops::Range;

pub use enum_trait_derive::Enum;

pub type Variants<T> = Map<Range<usize>, fn(usize) -> T>;

// TODO: Document the trait.

// TODO: Document the unsafe contract that is required by this trait.
// https://doc.rust-lang.org/std/keyword.unsafe.html
// https://doc.rust-lang.org/book/ch19-01-unsafe-rust.html#implementing-an-unsafe-trait

/// A trait to map an `enum` to and from a `usize`.
///
/// The trait can be `derive`d.
///
/// TODO: Finish documentation.
///
/// # Conditions
///
/// 0 < index < LENGTH
/// variant <=> index
/// one-to-one
/// isize maximum?
/// `from_index_unchecked`
/// `to_index`
///
/// # Examples
///
/// ```
/// use enum_trait::Enum;
///
/// #[derive(Enum)]
/// enum Direction {
///     North,
///     East,
///     South,
///     West,
/// }
///
/// assert_eq!(Direction::North.to_index(), 0);
/// assert_eq!(Direction::East.to_index(), 1);
/// assert_eq!(Direction::South.to_index(), 2);
/// assert_eq!(Direction::West.to_index(), 3);
/// ```
pub unsafe trait Enum {
    const LENGTH: usize;

    // Required methods

    unsafe fn from_index_unchecked(index: usize) -> Self;

    fn to_index(self) -> usize;

    // Provided methods

    fn from_index(index: usize) -> Option<Self>
    where
        Self: Sized,
    {
        if index < Self::LENGTH {
            Some(unsafe { Self::from_index_unchecked(index) })
        } else {
            None
        }
    }

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

    // TODO: Once stabilised, it might be better to return a `impl Trait`.
    // `fn variants() -> impl DoubleEndedIterator<Item = Self> + ExactSizeIterator`.
    // https://github.com/rust-lang/rust/issues/91611
    // https://stackoverflow.com/questions/76503213

    fn variants() -> Variants<Self>
    where
        Self: Sized,
    {
        (0..Self::LENGTH).map(|index| Self::from_index(index).unwrap())
    }
}

// TODO: Document the trait.

// TODO: Document the unsafe contract that is required by this trait.
// https://doc.rust-lang.org/std/keyword.unsafe.html
// https://doc.rust-lang.org/book/ch19-01-unsafe-rust.html#implementing-an-unsafe-trait

pub unsafe trait EnumArray<T>: Enum {
    type Array;
}

unsafe impl Enum for () {
    const LENGTH: usize = 1;

    unsafe fn from_index_unchecked(index: usize) -> Self {
        match index {
            0 => (),
            _ => unreachable_unchecked(),
        }
    }

    fn to_index(self) -> usize {
        0
    }
}

unsafe impl<T> EnumArray<T> for () {
    type Array = [T; Self::LENGTH];
}

unsafe impl Enum for bool {
    const LENGTH: usize = 2;

    unsafe fn from_index_unchecked(index: usize) -> Self {
        match index {
            0 => false,
            1 => true,
            _ => unreachable_unchecked(),
        }
    }

    fn to_index(self) -> usize {
        match self {
            false => 0,
            true => 1,
        }
    }
}

unsafe impl<T> EnumArray<T> for bool {
    type Array = [T; Self::LENGTH];
}

macro_rules! impl_int {
    ($type:ty) => {
        unsafe impl Enum for $type {
            const LENGTH: usize = 1 << Self::BITS;

            unsafe fn from_index_unchecked(index: usize) -> Self {
                (index as Self).wrapping_sub(Self::MIN)
            }

            fn to_index(self) -> usize {
                self.abs_diff(Self::MIN).into()
            }
        }

        unsafe impl<T> EnumArray<T> for $type {
            type Array = [T; Self::LENGTH];
        }
    };
}

impl_int!(i8);
impl_int!(u8);
