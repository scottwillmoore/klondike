# `enum-trait`

## Goals

- Get an `enum` discriminant, although may not be possible when niches are filled

Values...

- Iterate over all values of an `enum`
- Get a bounded index (0 <= index < VALUE_COUNT) of `enum` values
- Get the first, last value of an `enum`
- Get the next, previous value of an `enum`

Variants...

- Get the name of a variant
- Iterate over all variants (discriminants) of an `enum`
- Get a bounded index (0 <= index < VARIANT_COUNT) of `enum` variants
- Get the first, last variant of an `enum`
- Get the next, previous variant of an `enum`

NOTE: Niches can be filled, therefore an `enum` may have no discriminant.

NOTE: A bounded index requires a lookup table if the `enum` discriminant has gaps.

NOTE: Should an `enum` implement `Eq` or `Ord`?

### Types

- Empty: No variants. Equivalent to the `!` type.
- Fieldless: Variants, but none have fields.
- Data: Variants, but some have fields.

### Representations

- Rust representation.
- Rust representation with specific discriminant: `#[repr(Int)]`.
- C representation: `#[repr(C)]`.
- C representation with specific discriminant: `#[repr(C, Int)]`.
- Transparent representation: `#[repr(transparent)]`.

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
