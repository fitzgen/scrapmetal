use super::{QueryAll, Term, TransformAll};

macro_rules! impl_trivial_term {
    ( $name:ty ) => {
        impl Term for $name {
            fn map_one_transform<F>(self, _: &mut F) -> Self
              where
                F: TransformAll,
            {
                self
            }

            fn map_one_query<F, R>(&self, _: &mut F) -> Vec<R>
                where F: QueryAll<R>
            {
                vec![]
            }
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

    fn map_one_query<F, R>(&self, q: &mut F) -> Vec<R>
    where
        F: QueryAll<R>,
    {
        self.iter().map(|t| q.query(t)).collect()
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

    fn map_one_query<F, R>(&self, q: &mut F) -> Vec<R>
    where
        F: QueryAll<R>,
    {
        vec![q.query(&**self)]
    }
}
