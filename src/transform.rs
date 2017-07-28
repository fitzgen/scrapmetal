use super::{Cast, Term};
use std::marker::PhantomData;

/// Work around Rust's lack of higher-rank type polymorphism with a trait that
/// has a generic `fn transform<T>` method. Essentially, we'd really prefer
/// taking arguments of type `F: for<T> FnMut(T) -> T` rather than `F:
/// TransformAll` but Rust doesn't support them yet (ever?).
pub trait TransformAll {
    /// Call the transform function on any `T`.
    fn transform<T>(&mut self, t: T) -> T
    where
        T: Term;
}

/// A transformation takes some value `U` and returns a new, transformed version
/// of it. It can be called on values of *any* type `T`, not just on values of
/// type `U`, in which case it is simply the identity function.
///
/// This essentially lifts a `FnMut(U) -> U` into a `for<T> FnMut(T) -> T`.
#[derive(Debug)]
pub struct Transformation<F, U>
where
    F: FnMut(U) -> U,
{
    f: F,
    phantom: PhantomData<fn(U) -> U>,
}

impl<F, U> Transformation<F, U>
where
    F: FnMut(U) -> U,
{
    /// Construct a new `Transformation` from the given function.
    pub fn new(f: F) -> Transformation<F, U> {
        Transformation {
            f,
            phantom: ::std::marker::PhantomData,
        }
    }
}

impl<F, U> TransformAll for Transformation<F, U>
where
    F: FnMut(U) -> U,
{
    fn transform<T>(&mut self, t: T) -> T {
        match Cast::<U>::cast(t) {
            Ok(u) => match Cast::<T>::cast((self.f)(u)) {
                Ok(t) => t,
                Err(_) => unreachable!(
                    "If T=U, then U=T. Cast isn't pub, so there aren't any \
                     future specializations that could wreck this for us."
                ),
            },
            Err(t) => t,
        }
    }
}

/// Recursively perform a transformation in a bottom up manner across a complete
/// data structure.
#[derive(Debug)]
pub struct Everywhere<F>
where
    F: TransformAll,
{
    f: F,
}

impl<F> Everywhere<F>
where
    F: TransformAll,
{
    /// Construct a new transformation traversal.
    pub fn new(f: F) -> Everywhere<F> {
        Everywhere { f }
    }
}

impl<F> TransformAll for Everywhere<F>
where
    F: TransformAll,
{
    fn transform<T>(&mut self, t: T) -> T
    where
        T: Term,
    {
        let t = t.map_one_transform(self);
        self.f.transform(t)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn transformation() {
        let mut not = Transformation::new(|b: bool| !b);
        assert_eq!(not.transform(true), false);
        assert_eq!(not.transform("string"), "string");
    }
}
