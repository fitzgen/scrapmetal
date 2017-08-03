use super::{GenericMutate, GenericQuery, Term, GenericTransform};

macro_rules! impl_trivial_term {
    ( $name:ty ) => {
        impl Term for $name {
            #[inline]
            fn map_one_transform<F>(self, _: &mut F) -> Self
            where
                F: GenericTransform,
            {
                self
            }

            #[inline]
            fn map_one_query<Q, R, F>(&self, _: &mut Q, _: F)
            where
                Q: GenericQuery<R>,
                F: FnMut(&mut Q, R),
            {}

            #[inline]
            fn map_one_mutation<M, R, F>(&mut self, _: &mut M, _: F)
            where
                M: GenericMutate<R>,
                F: FnMut(&mut M, R),
            {}
        }
    }
}

impl_trivial_term!(&'static str);
impl_trivial_term!(bool);
impl_trivial_term!(char);
impl_trivial_term!(f32);
impl_trivial_term!(f64);
impl_trivial_term!(usize);
impl_trivial_term!(u8);
impl_trivial_term!(u16);
impl_trivial_term!(u32);
impl_trivial_term!(u64);

impl<T> Term for Vec<T>
where
    T: Term,
{
    #[inline]
    fn map_one_transform<F>(mut self, f: &mut F) -> Vec<T>
    where
        F: GenericTransform,
    {
        self.drain(..).map(|t| f.transform(t)).collect()
    }

    #[inline]
    fn map_one_query<Q, R, F>(&self, query: &mut Q, mut each: F)
    where
        Q: GenericQuery<R>,
        F: FnMut(&mut Q, R),
    {
        self.iter()
            .map(|t| {
                let r = query.query(t);
                each(query, r);
            })
            .count();
    }

    #[inline]
    fn map_one_mutation<M, R, F>(&mut self, mutation: &mut M, mut each: F)
    where
        M: GenericMutate<R>,
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
    #[inline]
    fn map_one_transform<F>(self, f: &mut F) -> Box<T>
    where
        F: GenericTransform,
    {
        Box::new(f.transform(*self))
    }

    #[inline]
    fn map_one_query<Q, R, F>(&self, query: &mut Q, mut each: F)
    where
        Q: GenericQuery<R>,
        F: FnMut(&mut Q, R),
    {
        let r = query.query(&**self);
        each(query, r);
    }

    #[inline]
    fn map_one_mutation<M, R, F>(&mut self, mutation: &mut M, mut each: F)
    where
        M: GenericMutate<R>,
        F: FnMut(&mut M, R),
    {
        let r = mutation.mutate(&mut **self);
        each(mutation, r);
    }
}
