#[cfg(test)]
mod test;

use std::mem::MaybeUninit;

use enum_trait::EnumArray;

struct MaybeUninitEnumArray<K, V>
where
    K: EnumArray<V>,
{
    array: MaybeUninit<K::Array>,
    index: usize,
}

impl<K, V> MaybeUninitEnumArray<K, V>
where
    K: EnumArray<V>,
{
    fn uninit() -> Self {
        Self {
            array: MaybeUninit::uninit(),
            index: 0,
        }
    }

    fn len(&self) -> usize {
        K::LENGTH
    }

    // TODO: Document the unsafe contract that is required by this function.
    unsafe fn push(&mut self, value: V) {
        let pointer = std::ptr::addr_of_mut!(self.array).cast::<V>();
        pointer.add(self.index).write(value);
        self.index += 1;
    }

    // TODO: Document the unsafe contract that is required by this function.
    unsafe fn assume_init(self) -> K::Array {
        let pointer = std::ptr::addr_of!(self.array);
        pointer.read().assume_init()
    }
}

impl<K, V> Drop for MaybeUninitEnumArray<K, V>
where
    K: EnumArray<V>,
{
    fn drop(&mut self) {
        let pointer = self.array.as_mut_ptr().cast::<V>();
        // TODO: Document the unsafe contract that is required by this function.
        unsafe { std::ptr::slice_from_raw_parts_mut(pointer, self.index).drop_in_place() };
    }
}

// TODO: Once stabilised, use the associated constant from `Enum` to define the array.
// E.g. `pub struct EnumMap<K, V>([V; K::LENGTH]) where K: Enum`.
// https://github.com/rust-lang/rust/issues/60551

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EnumMap<K, V>
where
    K: EnumArray<V>,
{
    array: K::Array,
}

// TODO: Does `K` need to be `Copy`?

impl<K, V> EnumMap<K, V>
where
    K: EnumArray<V>,
{
    pub fn from_copy(value: V) -> Self
    where
        V: Copy,
    {
        let mut enum_map = MaybeUninitEnumArray::<K, V>::uninit();

        for _ in 0..enum_map.len() {
            unsafe { enum_map.push(value) };
        }

        Self {
            array: unsafe { enum_map.assume_init() },
        }
    }

    pub fn from_clone(reference: &V) -> Self
    where
        V: Clone,
    {
        let mut enum_map = MaybeUninitEnumArray::<K, V>::uninit();

        for _ in 0..enum_map.len() {
            unsafe { enum_map.push(reference.clone()) };
        }

        Self {
            array: unsafe { enum_map.assume_init() },
        }
    }

    pub fn get(&self, key: K) -> &V {
        todo!()
    }

    pub fn get_key_value(&self, key: K) -> &V {
        todo!()
    }

    pub fn get_mut(&mut self, key: K) -> &mut V {
        todo!()
    }

    // pub fn iter(&self) -> impl Iterator<Item = (&K, &V)> {
    //     todo!()
    // }

    // pub fn iter_mut(&mut self) -> impl Iterator<Item = ()> {
    //     todo!()
    // }

    // pub fn into_keys(self) -> impl Iterator<Item = ()> {
    //     todo!()
    // }

    // pub fn keys(&self) -> impl Iterator<Item = ()> {
    //     todo!()
    // }

    // pub fn into_values(self) -> impl Iterator<Item = ()> {
    //     todo!()
    // }

    // pub fn values(&self) -> impl Iterator<Item = ()> {
    //     todo!()
    // }

    // pub fn values_mut(&mut self) -> impl Iterator<Item = ()> {
    //     todo!()
    // }

    pub fn as_array(&self) -> &K::Array {
        todo!()
    }

    pub fn as_array_mut(&mut self) -> &mut K::Array {
        todo!()
    }

    pub fn as_slice(&self) -> &[V] {
        let pointer = std::ptr::addr_of!(self.array).cast();
        // TODO: Document the unsafe contract that is required by this function.
        // https://doc.rust-lang.org/stable/core/slice/fn.from_raw_parts.html
        unsafe { std::slice::from_raw_parts(pointer, K::LENGTH) }
    }

    pub fn as_mut_slice(&mut self) -> &mut [V] {
        let pointer = std::ptr::addr_of_mut!(self.array).cast();
        // TODO: Document the unsafe contract that is required by this function.
        // https://doc.rust-lang.org/stable/core/slice/fn.from_raw_parts_mut.html
        unsafe { std::slice::from_raw_parts_mut(pointer, K::LENGTH) }
    }
}

impl<K, V> Default for EnumMap<K, V>
where
    K: EnumArray<V>,
    V: Default,
{
    fn default() -> Self {
        let mut enum_map = MaybeUninitEnumArray::<K, V>::uninit();

        for _ in 0..enum_map.len() {
            unsafe { enum_map.push(V::default()) };
        }

        Self {
            array: unsafe { enum_map.assume_init() },
        }
    }
}
