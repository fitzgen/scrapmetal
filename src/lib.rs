//! Scrap Your Boilerplate!
//!
//! This crate provides the traversing, transforming, and querying helpers and
//! combinators from the Haskell paper "Scrap Your Boilerplate" by Lämmel and
//! Jones to Rust.
//!
#![feature(specialization)]
#![deny(missing_docs)]
#![deny(missing_debug_implementations)]

#[cfg(tests)]
mod company;

mod query;
mod term_impls;
mod transform;

pub use query::*;

pub use transform::*;

/// Dynamically cast a value to a `T`.
trait Cast<T>: Sized {
    fn cast(self) -> Result<T, Self>;
}

/// A default blanket implementation that says the value cannot be cast to `T`.
impl<T, U> Cast<T> for U {
    default fn cast(self) -> Result<T, Self> {
        Err(self)
    }
}

/// A specialization for when `Self=T` that allows the cast to succeed.
impl<T> Cast<T> for T {
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
        F: TransformAll;

    /// Perform one-layer traversal and querying of this value's direct
    /// children.
    fn map_one_query<F, R>(&self, f: &mut F) -> Vec<R>
    where
        F: QueryAll<R>;
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
