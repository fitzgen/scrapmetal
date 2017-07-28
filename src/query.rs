use super::{Cast, Term};
use std::marker::PhantomData;

// TODO: lighten up on `R: Clone` and allow a create-a-default function (which
// can itself default to `|| some_defualt.clone()`).

// TODO: Provide mutable querying.

// TODO: Helper that does mutable querying for when R=()

// TODO: Don't return `Vec<R>`, call a callback instead.

// TODO: benchmark the boilerplate vs scrapped versions.

/// A similar work around as `TransformAll`, but returning a query type, rather
/// than the same type. This is roughly equivalent to `for<T> FnMut(&T) -> R`.
pub trait QueryAll<R> {
    /// Call the query function on any `T`.
    fn query<T>(&mut self, t: &T) -> R
    where
        T: Term;
}

/// A query non-destructively creates some value `R` from references to a
/// `U`. It can be called on values of any type `T`, not just on values of type
/// `U`, so it requires a default `R` value for when it is called on values
/// which are not a `T`.
///
/// This essentially lifts a `FnMut(&U) -> R` into a `for<T> FnMut(&T) -> R`.
#[derive(Debug)]
pub struct Query<Q, U, R>
where
    Q: FnMut(&U) -> R,
    R: Clone,
{
    default: R,
    query: Q,
    phantom: PhantomData<fn(U) -> R>,
}

impl<Q, U, R> Query<Q, U, R>
where
    Q: FnMut(&U) -> R,
    R: Clone,
{
    /// Construct a new `Query`, with a default `R` value for when it is
    /// querying non-`U` values.
    pub fn new(default: R, query: Q) -> Query<Q, U, R> {
        Query {
            default,
            query,
            phantom: PhantomData,
        }
    }
}

impl<Q, U, R> QueryAll<R> for Query<Q, U, R>
where
    Q: FnMut(&U) -> R,
    R: Clone,
{
    fn query<T>(&mut self, t: &T) -> R
    where
        T: Term,
    {
        match Cast::<&U>::cast(t) {
            Ok(u) => (self.query)(u),
            Err(_) => self.default.clone(),
        }
    }
}

/// Recursively perform a query in a top-down, left-to-right manner across a
/// data structure. The `Q: Query<R>` queries individual values, while the `F:
/// FnMut(R, R) -> R` joins the results of multiple queries into a single
/// result.
#[derive(Debug)]
pub struct Everything<Q, R, F>
where
    Q: QueryAll<R>,
    F: FnMut(R, R) -> R,
    R: Clone,
{
    q: Q,p
    default: R,
    fold: F,
}

impl<Q, R, F> Everything<Q, R, F>
where
    Q: QueryAll<R>,
    F: FnMut(R, R) -> R,
    R: Clone,
{
    /// Construct a new `Everything` query traversal.
    pub fn new(default: R, q: Q, fold: F) -> Everything<Q, R, F> {
        Everything { q, default, fold }
    }
}

impl<Q, R, F> QueryAll<R> for Everything<Q, R, F>
where
    Q: QueryAll<R>,
    F: FnMut(R, R) -> R,
    R: Clone,
{
    fn query<T>(&mut self, t: &T) -> R
    where
        T: Term,
    {
        let r = self.q.query(t);
        let rs = t.map_one_query(self);
        rs.into_iter().fold(r, &mut self.fold)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn querying() {
        let mut char_to_u32 = Query::new(42, |c: &char| *c as u32);
        assert_eq!(char_to_u32.query(&'a'), 97);
        assert_eq!(char_to_u32.query(&'b'), 98);
        assert_eq!(char_to_u32.query(&vec![1, 2, 3]), 42);
    }
}
