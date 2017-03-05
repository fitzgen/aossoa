//! Given a struct definition, generate both an array-of-structs and
//! struct-of-arrays representation for collections of that struct.
//!
//! The `aossoa!` macro generates mutable and immutable reference types for
//! referring to a single element of both the array-of-structs and
//! struct-of-array representations. Additionally, it generates trait
//! implementations to abstract over both implementations, allowing you to
//! experiment and choose the representation that fits your memory access
//! patterns.
//!
//! # Example Usage
//!
//! ```
//! #[macro_use]
//! extern crate aossoa;
//!
//! aossoa!{
//!     /// An `Rgb` is a triple of red, green, and blue.
//!     #[derive(Debug, Clone, Copy, PartialEq, Eq)]
//!     struct Rgb {
//!         r: u8,
//!         g: u8,
//!         b: u8,
//!     }
//!
//!     /// A trait for anything that is logically a collection of `Rgb`
//!     /// instances. That is, a trait to abstract over either a
//!     /// struct-of-arrays collection of `Rgb`s or an array-of-structs
//!     /// collection of `Rgb`s.
//!     ///
//!     /// Generated associated types:
//!     ///
//!     /// * `type Ref` which must implement `RgbRef`
//!     /// * `type Mut` which must implement `RgbRefMut`
//!     ///
//!     /// Generated trait methods are:
//!     ///
//!     /// * `fn get(&self, idx: usize) -> Option<Self::Ref>`
//!     /// * `fn get_mut(&mut self, idx: usize) -> Option<Self::Mut>`
//!     collection trait RgbCollection;
//!
//!     /// Iterator struct for collection.
//!     ///
//!     /// Generated trait method for Iterator:
//!     ///
//!     /// * `fn next(&mut self) -> Option<Self::Item>`
//!     iterator struct RgbCollectionIterator;
//!
//!     /// A trait for anything that is logically an immutable, shared
//!     /// reference to an `Rgb`.
//!     ///
//!     /// Generated trait methods are:
//!     ///
//!     /// * `fn r(&self) -> &u8`
//!     /// * `fn g(&self) -> &u8`
//!     /// * `fn b(&self) -> &u8`
//!     ref trait RgbRef;
//!
//!     /// A trait for anything that is logically a mutable, unique
//!     /// reference to an `Rgb`.
//!     ///
//!     /// Generated trait methods are:
//!     ///
//!     /// * `fn r(&mut self) -> &mut u8`
//!     /// * `fn g(&mut self) -> &mut u8`
//!     /// * `fn b(&mut self) -> &mut u8`
//!     ref mut trait RgbRefMut;
//!
//!     aos {
//!         /// An array-of-structs representation of many `Rgb`s.
//!         ///
//!         /// This is laid out in memory like:
//!         ///
//!         ///    ... | r | g | b | _ | r | g | b | _ | r | g | b | _ | ...
//!         ///
//!         /// Where _ is a padding char inserted by the compiler for
//!         /// alignment. TODO FITZGEN double check this
//!         #[derive(Debug)]
//!         struct RgbAos;
//!
//!         /// An immutable, shared reference to an `Rgb` inside of a `RgbAos`.
//!         ///
//!         /// Implements the `RgbRef` trait.
//!         #[derive(Debug, Clone, Copy, PartialEq, Eq)]
//!         ref RgbAosRef;
//!
//!         /// A mutable, unique reference to an `Rgb` inside of a `RgbAos`.
//!         ///
//!         /// Implements the `RgbRef` and `RgbRefMut` traits.
//!         #[derive(Debug)]
//!         ref mut RgbAosRefMut;
//!     }
//!
//!     soa {
//!         /// A struct-of-arrays representation of many `Rgb`s.
//!         ///
//!         /// This is laid out in memory like:
//!         ///
//!         ///    ... | r | r | r | ... | g | g | g | ... | b | b | b | ...
//!         #[derive(Debug)]
//!         struct RgbSoa;
//!
//!         /// An immutable, shared reference to an `Rgb` inside of a `RgbSoa`.
//!         ///
//!         /// Implements the `RgbRef` trait.
//!         #[derive(Debug, Clone, Copy)]
//!         ref RgbSoaRef;
//!
//!         /// A mutable, unique reference to an `Rgb` inside of a `RgbSoa`.
//!         ///
//!         /// Implements the `RgbRef` and `RgbRefMut` traits.
//!         #[derive(Debug)]
//!         ref mut RgbSoaRefMut;
//!     }
//! }
//!
//! # fn main() {}
//! ```

#[macro_export]
macro_rules! aossoa {
    (
        $( #[$attr:meta] )*
        struct $name:ident {
            $( $field_name:ident : $field_ty:ty , )*
        }

        $( #[$trait_attr:meta] )*
        collection trait $collection_trait_name:ident ;

        $( #[$iterator_struct_attr:meta] )*
        iterator struct $iterator_struct_name:ident ;

        $( #[$ref_trait_attr:meta] )*
        ref trait $ref_trait_name:ident ;

        $( #[$ref_mut_trait_attr:meta] )*
        ref mut trait $ref_mut_trait_name:ident ;

        aos {
            $( #[$aos_attr:meta] )*
            struct $aos_name:ident ;

            $( #[$aos_ref_attr:meta] )*
            ref $aos_ref_name:ident ;

            $( #[$aos_ref_mut_attr:meta] )*
            ref mut $aos_ref_mut_name:ident ;
        }

        soa {
            $( #[$soa_attr:meta] )*
            struct $soa_name:ident ;

            $( #[$soa_ref_attr:meta] )*
            ref $soa_ref_name:ident ;

            $( #[$soa_ref_mut_attr:meta] )*
            ref mut $soa_ref_mut_name:ident ;
        }

    ) => {

        // Struct //////////////////////////////////////////////////////////////

        $( #[$attr] )*
        pub struct $name {
            $( $field_name : $field_ty , )*
        }

        // Traits //////////////////////////////////////////////////////////////

        $( #[$trait_attr] )*
        pub trait $collection_trait_name<'a>: Sized {
            /// The associated shared, immutable reference type.
            type Ref: $ref_trait_name;

            /// The associated unique, mutable reference type.
            type Mut: $ref_mut_trait_name;

            // /// TODO FITZGEN
            // type Iter: Iterator<Item = Self::Ref>;

            // /// TODO FITZGEN
            // type IterMut: Iterator<Item = Self::Mut>;

            /// Construct a new, empty instance of this collection.
            fn new() -> Self {
                Self::with_capacity(0)
            }

            /// Construct a new, empty instance of this collection, with
            /// pre-allocated storage for at least `capacity` members.
            fn with_capacity(capacity: usize) -> Self;

            /// Returns the number of elements this collection can hold without
            /// reallocating.
            fn capacity(&self) -> usize;

            /// Reserves capacity for at least `additional` more elements to be
            /// inserted into this collection.
            fn reserve(&mut self, additional: usize);

            /// Shortens this collection, keeping the first `len` elements and
            /// dropping the rest.
            fn truncate(&mut self, len: usize);

            /// Appends an element to the back of the collection.
            fn push(&mut self, value: $name);

            /// Removes the last element from the collection and returns it, or
            /// `None` if the collection is empty.
            fn pop(&mut self) -> Option<$name>;

            /// Clears the collection, removing all values.
            fn clear(&mut self) {
                self.truncate(0)
            }

            /// Returns the number of elements in the collection.
            fn len(&self) -> usize;

            /// Returns true if the collection contains no elements.
            fn is_empty(&self) -> bool {
                self.len() == 0
            }

            /// Get a shared, immutable reference to the item at the given
            /// index, or `None` if the index is out of bounds.
            fn get(&'a self, idx: usize) -> Option<Self::Ref>;

            /// Get a unique, mutable reference to the item at the given index,
            /// or `None` if the index is out of bounds.
            fn get_mut(&'a mut self, idx: usize) -> Option<Self::Mut>;

            fn iter(&'a self) -> $iterator_struct_name <'a, Self> {
                $iterator_struct_name::<'a, Self> { collection: &self, index: 0}
            }

            // /// TODO FITZGEN
            // fn iter_mut(&'a mut self) -> Self::IterMut;
        }

        // TODO FITZGEN: IntoIterator for &Collection and &mut Collection

        $( #[$ref_trait_attr] )*
        pub trait $ref_trait_name {
            $(
                fn $field_name (&self) -> & $field_ty;
            )*
        }

        $( #[$ref_trait_attr] )*
        pub trait $ref_mut_trait_name : $ref_trait_name {
            $(
                fn $field_name (&mut self) -> &mut $field_ty;
            )*
        }

        // Iterator ////////////////////////////////////////////////////////////

        pub struct $iterator_struct_name<'a, T>
            where T: 'a + $collection_trait_name<'a>
        {
            collection: &'a T,
            index: usize,
        }

        impl<'a, T> Iterator for $iterator_struct_name<'a, T>
            where T: 'a + $collection_trait_name<'a>
        {
            type Item = T::Ref;

            fn next(&mut self) -> Option<Self::Item> {
                let value = self.collection.get(self.index);
                // TODO: We could probably reuse the index in the Ref type somehow
                //       Possibly by making the Ref type the iterator
                self.index += 1;
                value
            }
        }

        // AOS /////////////////////////////////////////////////////////////////

        $( #[$aos_attr] )*
        pub struct $aos_name(Vec<$name>);

        impl<'a> $collection_trait_name<'a> for $aos_name {
            type Ref = $aos_ref_name<'a>;
            type Mut = $aos_ref_mut_name<'a>;

            // type Iter: Iterator<Item = Self::Ref>;
            // type IterMut: Iterator<Item = Self::Mut>;

            fn with_capacity(capacity: usize) -> Self {
                $aos_name(Vec::with_capacity(capacity))
            }

            fn capacity(&self) -> usize {
                self.0.capacity()
            }

            fn reserve(&mut self, additional: usize) {
                self.0.reserve(additional);
            }

            fn truncate(&mut self, len: usize) {
                self.0.truncate(len);
            }

            fn push(&mut self, value: $name) {
                self.0.push(value);
            }

            fn pop(&mut self) -> Option<$name> {
                self.0.pop()
            }

            fn len(&self) -> usize {
                self.0.len()
            }

            fn get(&'a self, idx: usize) -> Option<Self::Ref> {
                self.0.get(idx).map(|r| $aos_ref_name { r: r })
            }

            fn get_mut(&'a mut self, idx: usize) -> Option<Self::Mut> {
                self.0.get_mut(idx).map(|r| $aos_ref_mut_name { r: r })
            }

            // /// TODO FITZGEN
            // fn iter(&'a self) -> Self::Iter;

            // /// TODO FITZGEN
            // fn iter_mut(&'a mut self) -> Self::IterMut;
        }

        impl ::std::iter::FromIterator<$name> for $aos_name
        {
            fn from_iter<I>(iter: I) -> Self
                where I: IntoIterator<Item = $name>
            {
                let mut me = Self::new();
                for x in iter {
                    me.push(x);
                }
                me
            }
        }

        $( #[$aos_ref_attr] )*
        pub struct $aos_ref_name <'a> {
            r: &'a $name
        }

        impl<'a> $ref_trait_name for $aos_ref_name<'a> {
            $(
                fn $field_name (&self) -> & $field_ty {
                    &self.r. $field_name
                }
            )*
        }

        $( #[$aos_ref_mut_attr] )*
        pub struct $aos_ref_mut_name <'a> {
            r: &'a mut $name
        }

        impl<'a> $ref_trait_name for $aos_ref_mut_name<'a> {
            $(
                fn $field_name (&self) -> & $field_ty {
                    &self.r. $field_name
                }
            )*
        }

        impl<'a> $ref_mut_trait_name for $aos_ref_mut_name<'a> {
            $(
                fn $field_name (&mut self) -> &mut $field_ty {
                    &mut self.r. $field_name
                }
            )*
        }

        // SOA /////////////////////////////////////////////////////////////////

        $( #[$soa_attr] )*
        pub struct $soa_name {
            $( $field_name : Vec<$field_ty> , )*
        }

        impl<'a> $collection_trait_name<'a> for $soa_name {
            type Ref = $soa_ref_name<'a>;
            type Mut = $soa_ref_mut_name<'a>;

            // type Iter: Iterator<Item = Self::Ref>;
            // type IterMut: Iterator<Item = Self::Mut>;

            fn with_capacity(capacity: usize) -> Self {
                $soa_name {
                    $( $field_name: Vec::with_capacity(capacity), )*
                }
            }

            #[allow(unreachable_code)]
            fn capacity(&self) -> usize {
                $(
                    return self.$field_name.capacity();
                )*
                // TODO FITZGEN: us a macro pattern to enforce at least one
                // field.
                unimplemented!()
            }

            fn reserve(&mut self, additional: usize) {
                $(
                    self.$field_name.reserve(additional);
                )*
            }

            fn truncate(&mut self, len: usize) {
                $(
                    self.$field_name.truncate(len);
                )*
            }

            fn push(&mut self, value: $name) {
                $(
                    self.$field_name.push(value.$field_name);
                )*
            }

            fn pop(&mut self) -> Option<$name> {
                if self.len() == 0 {
                    None
                } else {
                    Some($name {
                        $( $field_name: self.$field_name.pop().unwrap(), )*
                    })
                }
            }

            #[allow(unreachable_code)]
            fn len(&self) -> usize {
                $(
                    return self.$field_name.len();
                )*
                // TODO FITZGEN: us a macro pattern to enforce at least one
                // field.
                unimplemented!()
            }

            fn get(&'a self, idx: usize) -> Option<Self::Ref> {
                if idx >= self.len() {
                    return None;
                }

                Some($soa_ref_name {
                    soa: self,
                    idx: idx,
                })
            }

            fn get_mut(&'a mut self, idx: usize) -> Option<Self::Mut> {
                if idx >= self.len() {
                    return None;
                }

                Some($soa_ref_mut_name {
                    soa: self,
                    idx: idx,
                })
            }

            // /// TODO FITZGEN
            // fn iter(&'a self) -> Self::Iter;

            // /// TODO FITZGEN
            // fn iter_mut(&'a mut self) -> Self::IterMut;
        }

        impl ::std::iter::FromIterator<$name> for $soa_name
        {
            fn from_iter<I>(iter: I) -> Self
                where I: IntoIterator<Item = $name>
            {
                let mut me = Self::new();
                for x in iter {
                    me.push(x);
                }
                me
            }
        }

        $( #[$soa_ref_attr] )*
        pub struct $soa_ref_name<'a> {
            soa: &'a $soa_name,
            idx: usize,
        }

        impl<'a> $ref_trait_name for $soa_ref_name<'a> {
            $(
                fn $field_name (&self) -> & $field_ty {
                    &self.soa. $field_name [self.idx]
                }
            )*
        }

        $( #[$soa_ref_mut_attr] )*
        pub struct $soa_ref_mut_name<'a> {
            soa: &'a mut $soa_name,
            idx: usize,
        }

        impl<'a> $ref_trait_name for $soa_ref_mut_name<'a> {
            $(
                fn $field_name (&self) -> & $field_ty {
                    &self.soa. $field_name [self.idx]
                }
            )*
        }

        impl<'a> $ref_mut_trait_name for $soa_ref_mut_name<'a> {
            $(
                fn $field_name (&mut self) -> &mut $field_ty {
                    &mut self.soa. $field_name [self.idx]
                }
            )*
        }
    }
}

#[cfg(test)]
mod tests {
    use std::iter::FromIterator;

    aossoa!{
        #[derive(Clone, Copy, Debug)]
        struct Rgb {
            r: u8,
            g: u8,
            b: u8,
        }

        collection trait RgbCollection;
        iterator struct RgbCollectionIterator;
        ref trait RgbRef;
        ref mut trait RgbRefMut;

        aos {
            struct RgbAos;
            ref RgbAosRef;
            ref mut RgbAosRefMut;
        }

        soa {
            struct RgbSoa;
            ref RgbSoaRef;
            ref mut RgbSoaRefMut;
        }
    }

    fn sum_all_rgb<'a, T: RgbCollection<'a>>(rgbs: &'a T) -> usize {
        let mut sum = 0;
        let mut i = 0;
        while let Some(rgb) = rgbs.get(i) {
            sum += *rgb.r() as usize;
            sum += *rgb.g() as usize;
            sum += *rgb.b() as usize;
            i += 1;
        }
        sum
    }

    fn sum_all_rgb_iter<'a, T: RgbCollection<'a>>(rgbs: &'a T) -> usize {
        let mut sum = 0;
        for rgb in rgbs.iter() {
            sum += *rgb.r() as usize;
            sum += *rgb.g() as usize;
            sum += *rgb.b() as usize;
        }
        sum
    }

    #[test]
    fn sum_all_rgb_test() {
        let aos = RgbAos::from_iter([
            Rgb { r: 1, g: 2, b: 3 },
            Rgb { r: 1, g: 2, b: 3 },
            Rgb { r: 1, g: 2, b: 3 },
            Rgb { r: 1, g: 2, b: 3 },
            Rgb { r: 1, g: 2, b: 3 },
            Rgb { r: 1, g: 2, b: 3 },
        ].iter().cloned());

        let soa = RgbSoa::from_iter([
            Rgb { r: 1, g: 2, b: 3 },
            Rgb { r: 1, g: 2, b: 3 },
            Rgb { r: 1, g: 2, b: 3 },
            Rgb { r: 1, g: 2, b: 3 },
            Rgb { r: 1, g: 2, b: 3 },
            Rgb { r: 1, g: 2, b: 3 },
        ].iter().cloned());

        assert_eq!(sum_all_rgb(&aos), 36);
        assert_eq!(sum_all_rgb(&soa), 36);
        assert_eq!(sum_all_rgb_iter(&aos), 36);
        assert_eq!(sum_all_rgb_iter(&soa), 36);
    }
}
