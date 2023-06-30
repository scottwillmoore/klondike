# `enum-trait`

## Definitions

An [**enumeration**](https://doc.rust-lang.org/reference/items/enumerations.html), which is usually just referred to as an **enum** is a ["nominal, heterogeneous disjoint union type"](https://doc.rust-lang.org/reference/types/enum.html). In functional programming literature it is a form of [algebraic data type](https://en.wikipedia.org/wiki/Algebraic_data_type) (ADT). An enum is defined defined with the [`enum`](https://doc.rust-lang.org/std/keyword.enum.html) keyword.

An enum declares zero or more **variants** which have unique names. The `Direction` enum contains four variants: `North`, `East`, `South` and `West`. These are **unit** variants as they do not contain data.

```rust
enum Direction {
    North,
    East,
    South,
    West,
}
```

There are also **tuple** and **struct** variants which can contain data. The `Name` enum contains two variants: `First` and `Full`. The `First` variant also contains data in the form of a tuple. The `Full` variant contains data in the form of a struct.

```rust
enum Name {
    First(String),
    Full {
        first: String,
        last: String,
    },
}
```

Enums can be classified into these categories:

- An **empty** enum have no variants and can therefore never be instantiated. This is equivalent to the ["never"](https://doc.rust-lang.org/std/primitive.never.html) type.

- A **fieldless** enum have one or more variants where no variants contain any fields in the form of a tuple or struct.

- A **data** enum have one or more variants where one or more of these variants contain fields in the form of a tuple of struct.

```rust
enum Empty {}

enum Fieldless {
    A,
    B,
}

enum Data {
    A,
    B(u32),
    C {
        a: u32,
        b: u32,
    },
}
```

An enum provides **constructors** which can be used to create **instances**.

```rust
let direction = Direction::North;

let first_name = Name::First("Scott".to_string());

let full_name = Name::Full {
    first: "Scott".to_string(),
    last: "Moore".to_string(),
};
```

In order to distinguish instances, each variant is assigned a **discriminant**. This discriminant is used to determine the variant of the instance, and therefore what data it may contain. For each enum, the discriminant is a unique integer which identifies the variants. By default the discriminant is a `isize`, however the compiler is free to use a smaller integer in the memory layout of the enum.

The memory representation of an enum can be classified into these categories:

- The Rust representation.

- The Rust representation with specific discriminant: `#[repr(Int)]`.

- The C representation: `#[repr(C)]`.

- The C representation with specific discriminant: `#[repr(C, Int)]`.

- The transparent representation: `#[repr(transparent)]`.

TODO:

- Representations.
- Discriminant elision.
- Niches.
- Single variant?

https://play.rust-lang.org/?version=nightly&mode=release&edition=2021&gist=61eb426addd0bda0f0bdf52055bcc37d

## Goals

### Discriminants

- Get the the discriminant of an `enum`.
- Iterate over all discriminants of an `enum`?
- Get the first, last variant of an `enum`.
- Get the next, previous variant of an `enum`.

NOTE: Niches can be filled, therefore an `enum` may have no discriminant.

### Values

- Iterate over all values of an `enum`.
- Get a unique bounded index (0 <= index < VALUE_COUNT) from each `enum` value?
- Get the first, last value of an `enum`.
- Get the next, previous value of an `enum`.

NOTE: Iteration could be ordered by definition, or by discriminant, or neither.

NOTE: A bounded index requires a lookup table if the `enum` discriminant has gaps.

```rust
pub trait Values {
    fn values() -> impl Iterator<Item = Self>;

    fn first() -> Option<Self>;
    fn previous(&self) -> Option<Self>;
    fn next(&self) -> Option<Self>;
    fn last() -> Option<Self>;
}

pub unsafe trait IndexValues {
    const MAX: usize;

    unsafe fn from_index_unchecked(index: usize) -> Self;
    fn from_index(index: usize) -> Option<Self>;

    fn to_index(self) -> usize;
}

impl<T> Values for T where T: IndexValues;
```

### Variants

- Get the name of a variant.
- Iterate over all variants of an `enum`.
- Get a unique bounded index (0 <= index < VARIANT_COUNT) from each `enum` variant?
- Get the first, last variant of an `enum`.
- Get the next, previous variant of an `enum`.

```rust
pub trait Variants {
    const LENGTH: usize;

    type Variant: Values;

    fn variants() -> impl Iterator<Item = Variant>;

    fn first() -> Option<Variant>;
    fn previous(&self) -> Option<Variant>;
    fn next(&self) -> Option<Variant>;
    fn last() -> Option<Variant>;
}
```

NOTE: Iteration could be ordered by definition, or by discriminant, or neither.

NOTE: A bounded index requires a lookup table if the `enum` discriminant has gaps.

## References

### Documents

- https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html
- https://doc.rust-lang.org/nomicon/exotic-sizes.html
- https://doc.rust-lang.org/nomicon/other-reprs.html
- https://doc.rust-lang.org/nomicon/repr-rust.html
- https://doc.rust-lang.org/reference/attributes/type_system.html#the-non_exhaustive-attribute
- https://doc.rust-lang.org/reference/items/enumerations.html
- https://doc.rust-lang.org/reference/type-layout.html
- https://doc.rust-lang.org/rust-by-example/custom_types/enum.html
- https://doc.rust-lang.org/std/keyword.enum.html
- https://rust-lang.github.io/unsafe-code-guidelines/layout/enums.html

### Libraries

- https://docs.rs/enum-iterator/latest/enum_iterator/
- https://docs.rs/enum-kinds/latest/enum_kinds/
- https://docs.rs/enum-map/latest/enum_map/
- https://docs.rs/enum_dispatch/latest/enum_dispatch/
- https://docs.rs/num_enum/latest/num_enum/
- https://docs.rs/strum/latest/strum/

### Languages

- https://learn.microsoft.com/en-us/dotnet/api/system.enum
- https://docs.oracle.com/en/java/javase/20/docs/api/java.base/java/lang/Enum.html

### Posts

- https://togglebit.io/posts/enums/
- https://fasterthanli.me/articles/peeking-inside-a-rust-enum

### RFCs

- https://rust-lang.github.io/rfcs/0234-variants-namespace.html
- https://rust-lang.github.io/rfcs/0390-enum-namespacing.html
- https://rust-lang.github.io/rfcs/0418-struct-variants.html
- https://rust-lang.github.io/rfcs/0639-discriminant-intrinsic.html
- https://rust-lang.github.io/rfcs/1696-discriminant.html
- https://rust-lang.github.io/rfcs/2008-non-exhaustive.html
- https://rust-lang.github.io/rfcs/2195-really-tagged-unions.html
- https://rust-lang.github.io/rfcs/2338-type-alias-enum-variants.html
- https://rust-lang.github.io/rfcs/2363-arbitrary-enum-discriminant.html
- https://rust-lang.github.io/rfcs/2645-transparent-unions.html
- https://rust-lang.github.io/rfcs/3107-derive-default-enum.html
- https://rust-lang.github.io/rfcs/3323-restrictions.html
