use super::{Cast, Term};
use std::marker::PhantomData;

// TODO: Provide mutable querying.

// TODO: Helper that does mutable querying for when R=()

// TODO: Don't return `Vec<R>`, call a callback instead.

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
pub struct Query<Q, U, D, R>
where
    Q: FnMut(&U) -> R,
    D: FnMut() -> R,
{
    make_default: D,
    query: Q,
    phantom: PhantomData<fn(&U) -> R>,
}

impl<Q, U, R> Query<Q, U, fn() -> R, R>
where
    Q: FnMut(&U) -> R,
    R: Default,
{
    /// Construct a new `Query`, returning `R::default()` for the cases where we
    /// query a value whose type is not `U`.
    pub fn new(query: Q) -> Query<Q, U, fn() -> R, R> {
        Query {
            make_default: Default::default,
            query,
            phantom: PhantomData,
        }
    }
}

impl<Q, U, D, R> Query<Q, U, D, R>
where
    Q: FnMut(&U) -> R,
    D: FnMut() -> R,
{
    /// Construct a new `Query`, returning `make_default()` for the cases where
    /// we query a value whose type is not `U`.
    pub fn or_else(make_default: D, query: Q) -> Query<Q, U, D, R> {
        Query {
            make_default,
            query,
            phantom: PhantomData,
        }
    }
}

impl<Q, U, D, R> QueryAll<R> for Query<Q, U, D, R>
where
    Q: FnMut(&U) -> R,
    D: FnMut() -> R,
{
    fn query<T>(&mut self, t: &T) -> R
    where
        T: Term,
    {
        match Cast::<&U>::cast(t) {
            Ok(u) => (self.query)(u),
            Err(_) => (self.make_default)(),
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
    q: Q,
    fold: F,
    phantom: PhantomData<fn(R, R) -> R>,
}

impl<Q, R, F> Everything<Q, R, F>
where
    Q: QueryAll<R>,
    F: FnMut(R, R) -> R,
    R: Clone,
{
    /// Construct a new `Everything` query traversal.
    pub fn new(q: Q, fold: F) -> Everything<Q, R, F> {
        Everything {
            q,
            fold,
            phantom: PhantomData,
        }
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
        let mut char_to_u32 = Query::or_else(|| 42, |c: &char| *c as u32);
        assert_eq!(char_to_u32.query(&'a'), 97);
        assert_eq!(char_to_u32.query(&'b'), 98);
        assert_eq!(char_to_u32.query(&vec![1, 2, 3]), 42);
    }
}
