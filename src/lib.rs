//! Scrap Your Boilerplate!
//!
//! This crate provides the traversing, transforming, and querying helpers and
//! combinators from the Haskell paper "Scrap Your Boilerplate: A Practical
//! Design Pattern for Generic Programming" by Lämmel and Peyton Jones to Rust.
//!
#![feature(specialization)]
#![deny(missing_docs)]
#![deny(missing_debug_implementations)]

mod mutation;
mod query;
mod term_impls;
mod transform;

pub use mutation::*;

pub use query::*;

pub use transform::*;

/// Dynamically cast a value to a `T`.
trait Cast<T>: Sized {
    fn cast(self) -> Result<T, Self>;
}

/// A default blanket implementation that says the value cannot be cast to `T`.
impl<T, U> Cast<T> for U {
    #[inline(always)]
    default fn cast(self) -> Result<T, Self> {
        Err(self)
    }
}

/// A specialization for when `Self=T` that allows the cast to succeed.
impl<T> Cast<T> for T {
    #[inline(always)]
    fn cast(self) -> Result<T, Self> {
        Ok(self)
    }
}

/// A `Term` is a value that can be mapped or queried.
pub trait Term: Sized {
    /// Perform one-layer traversal and transformation of this value's direct
    /// children.
    fn map_one_transform<F>(self, f: &mut F) -> Self
    where
        F: TransformForAll;

    /// Perform one-layer traversal and immutable querying of this value's
    /// direct children, calling `each` on each of the query result for each
    /// direct child.
    fn map_one_query<Q, R, F>(&self, query: &mut Q, each: F)
    where
        Q: QueryForAll<R>,
        F: FnMut(&mut Q, R);

    /// Perform one-layer traversal and mutable querying of this value's direct
    /// children, calling `each` on each of the query result for each direct
    /// child.
    fn map_one_mutation<M, R, F>(&mut self, mutation: &mut M, each: F)
    where
        M: MutateForAll<R>,
        F: FnMut(&mut M, R);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn casting() {
        assert_eq!(Cast::<bool>::cast(1), Err(1));
        assert_eq!(Cast::<bool>::cast(true), Ok(true));
    }
}
