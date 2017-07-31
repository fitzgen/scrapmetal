use super::{MutateAll, QueryAll, Term, TransformAll};

macro_rules! impl_trivial_term {
    ( $name:ty ) => {
        impl Term for $name {
            fn map_one_transform<F>(self, _: &mut F) -> Self
            where
                F: TransformAll,
            {
                self
            }

            fn map_one_query<Q, R, F>(&self, _: &mut Q, _: F)
            where
                Q: QueryAll<R>,
                F: FnMut(&mut Q, R),
            {}

            fn map_one_mutation<M, R, F>(&mut self, _: &mut M, _: F)
            where
                M: MutateAll<R>,
                F: FnMut(&mut M, R),
            {}
        }
    }
}

impl_trivial_term!(&'static str);
impl_trivial_term!(bool);
impl_trivial_term!(f64);
impl_trivial_term!(char);
impl_trivial_term!(usize);

impl<T> Term for Vec<T>
where
    T: Term,
{
    fn map_one_transform<F>(mut self, f: &mut F) -> Vec<T>
    where
        F: TransformAll,
    {
        self.drain(..).map(|t| f.transform(t)).collect()
    }

    fn map_one_query<Q, R, F>(&self, query: &mut Q, mut each: F)
    where
        Q: QueryAll<R>,
        F: FnMut(&mut Q, R),
    {
        self.iter()
            .map(|t| {
                let r = query.query(t);
                each(query, r);
            })
            .count();
    }

    fn map_one_mutation<M, R, F>(&mut self, mutation: &mut M, mut each: F)
    where
        M: MutateAll<R>,
        F: FnMut(&mut M, R),
    {
        self.iter_mut()
            .map(|t| {
                let r = mutation.mutate(t);
                each(mutation, r);
            })
            .count();
    }
}

impl<T> Term for Box<T>
where
    T: Sized + Term,
{
    fn map_one_transform<F>(self, f: &mut F) -> Box<T>
    where
        F: TransformAll,
    {
        Box::new(f.transform(*self))
    }

    fn map_one_query<Q, R, F>(&self, query: &mut Q, mut each: F)
    where
        Q: QueryAll<R>,
        F: FnMut(&mut Q, R),
    {
        let r = query.query(&**self);
        each(query, r);
    }

    fn map_one_mutation<M, R, F>(&mut self, mutation: &mut M, mut each: F)
    where
        M: MutateAll<R>,
        F: FnMut(&mut M, R),
    {
        let r = mutation.mutate(&mut **self);
        each(mutation, r);
    }
}
