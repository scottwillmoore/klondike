//! TODO: Write documentation.
//!
//! # Overview
//!
//! The *values* of a type are all the different valid representations of that
//! type.
//!
//! ## Primitives
//!
//! A `bool` has a size and alignment of 1. Despite this, it only has 2 valid
//! *values*: `false` (`0x00`) and `true` (`0x01`). Any other representations
//! are invalid, and therefore no *values*.
//!
//! A `u8` also has a size and alignment of 1. However, it has 256 valid
//! *values*: `0` (`0x00`), `1` (`0x01`), ..., `255` (`0xFF`). There are no
//! other representations.
//!
//! ## Non-primitives
//!
//! ### Array
//!
//! TODO: Add support.
//!
//! ### Enum
//!
//! An enum declares zero or more *variants* which can be either a *unit*,
//! *tuple* or *struct*. Each *variant* has one or more *values*. A unit variant
//! has only one value, while tuple and struct variants have one or more values
//! which are a product of their *fields*.
//!
//! TODO: Add support.
//!
//! ### Struct
//!
//! A struct declare zero or more *fields*. Each *field* has one or more
//! *values*. Both are product types, such that
//!
//! TODO: Add support.
//!
//! ### Tuple
//!
//! A tuple declare zero or more *fields*. Each *field* has one or more
//! *values*. Both are product types, such that
//!
//! TODO: Add support.
//!
//! ### Union
//!
//! TODO: Add support.

#[cfg(test)]
mod test;

use std::hint::unreachable_unchecked;
use std::iter::{Iterator, Map};
use std::ops::Range;

pub use enum_trait_derive::Enum;

pub type Variants<T> = Map<Range<usize>, fn(usize) -> T>;

/// A trait to map the *values* of a type to and from bounded, unique integers.
///
/// TODO: Write documentation.
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
///
/// # Requirements
///
/// TODO: Document the unsafe contract that is required by this trait.
///
/// https://doc.rust-lang.org/std/keyword.unsafe.html
/// https://doc.rust-lang.org/book/ch19-01-unsafe-rust.html#implementing-an-unsafe-trait
pub unsafe trait Enum {
    /// TODO: Write documentation.
    const LENGTH: usize;

    // Required methods

    /// TODO: Write documentation.
    unsafe fn from_index_unchecked(index: usize) -> Self;

    /// TODO: Write documentation.
    fn to_index(self) -> usize;

    // Provided methods

    /// TODO: Write documentation.
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

    /// TODO: Write documentation.
    fn first() -> Option<Self>
    where
        Self: Sized,
    {
        Self::from_index(0)
    }

    /// TODO: Write documentation.
    fn last() -> Option<Self>
    where
        Self: Sized,
    {
        Self::from_index(Self::LENGTH - 1)
    }

    /// TODO: Write documentation.
    fn previous(self) -> Option<Self>
    where
        Self: Sized,
    {
        self.to_index().checked_sub(1).and_then(Self::from_index)
    }

    /// TODO: Write documentation.
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

    /// TODO: Write documentation.
    fn variants() -> Variants<Self>
    where
        Self: Sized,
    {
        (0..Self::LENGTH).map(|index| Self::from_index(index).unwrap())
    }
}

/// TODO: Write documentation.
///
/// # Requirements
///
/// TODO: Document the unsafe contract that is required by this trait.
///
/// https://doc.rust-lang.org/std/keyword.unsafe.html
/// https://doc.rust-lang.org/book/ch19-01-unsafe-rust.html#implementing-an-unsafe-trait
pub unsafe trait EnumArray<T>: Enum {
    type Array;
}

/// TODO: Write documentation.
#[macro_export]
macro_rules! impl_enum_array {
    ($type:ty) => {
        unsafe impl<T> $crate::EnumArray<T> for $type {
            type Array = [T; Self::LENGTH];
        }
    };
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

impl_enum_array!(());

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

impl_enum_array!(bool);

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

        impl_enum_array!($type);
    };
}

impl_int!(i8);
impl_int!(u8);
