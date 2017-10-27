use super::{GenericMutate, GenericQuery, GenericTransform, Term};
use std::collections::*;
use std::iter::FromIterator;

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

impl_trivial_term!(());
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
impl_trivial_term!(isize);
impl_trivial_term!(i8);
impl_trivial_term!(i16);
impl_trivial_term!(i32);
impl_trivial_term!(i64);

macro_rules! impl_tuple_term {
    ( $name:ident $( , $names:ident )* ) => {
        impl<$name $( , $names )* > Term for ($name $( , $names )* )
        where
            $name: Term $(, $names : Term )*
        {
            #[inline]
            #[allow(non_snake_case)]
            fn map_one_transform<FF>(self, f: &mut FF) -> Self
            where
                FF: GenericTransform,
            {
                let ( $name $( , $names )* ) = self;
                ( f.transform( $name ) $( , f.transform( $names ) )* )
            }

            #[inline]
            #[allow(non_snake_case)]
            fn map_one_query<Q, R, FF>(&self, q: &mut Q, mut each: FF)
            where
                Q: GenericQuery<R>,
                FF: FnMut(&mut Q, R),
            {
                let ( ref $name $( , ref $names )* ) = *self;
                let r = q.query( $name );
                each(q, r);
                $(
                    let r = q.query( $names );
                    each(q, r);
                )*
            }

            #[inline]
            #[allow(non_snake_case)]
            fn map_one_mutation<M, R, FF>(&mut self, m: &mut M, mut each: FF)
            where
                M: GenericMutate<R>,
                FF: FnMut(&mut M, R),
            {
                let ( ref mut $name $( , ref mut $names )* ) = *self;
                let r = m.mutate( $name );
                each(m, r);
                $(
                    let r = m.mutate( $names );
                    each(m, r);
                )*
            }
        }
    }
}

impl_tuple_term!(A, B);
impl_tuple_term!(A, B, C);
impl_tuple_term!(A, B, C, D);
impl_tuple_term!(A, B, C, D, E);
impl_tuple_term!(A, B, C, D, E, F);
impl_tuple_term!(A, B, C, D, E, F, G);
impl_tuple_term!(A, B, C, D, E, F, G, H);
impl_tuple_term!(A, B, C, D, E, F, G, H, I);
impl_tuple_term!(A, B, C, D, E, F, G, H, I, J);
impl_tuple_term!(A, B, C, D, E, F, G, H, I, J, K);
impl_tuple_term!(A, B, C, D, E, F, G, H, I, J, K, L);

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

macro_rules! impl_iter_term {
    ($iter:ty) => {
        impl <T> Term for $iter
        where
            $iter: IntoIterator<Item = T> + FromIterator<T>,
            for <'b> &'b $iter: IntoIterator<Item = &'b T>,
            for <'b> &'b mut $iter: IntoIterator<Item = &'b mut T>,
            T: Term
        {
            fn map_one_transform<F>(self, f: &mut F) -> $iter
            where
                F: GenericTransform
            {
                self.into_iter().map(|x| f.transform(x)).collect()
            }

            fn map_one_query<Q, R, F>(&self, query: &mut Q, mut each: F)
            where
                Q: GenericQuery<R>,
                F: FnMut(&mut Q, R)
            {
                self.into_iter().for_each(|t| {
                    let r = query.query(t);
                    each(query, r);
                });
            }

            fn map_one_mutation<'a, M, R, F>(&'a mut self, mutation: &mut M, mut each: F)
            where
                M: GenericMutate<R>,
                F: FnMut(&mut M, R)
            {
                self.into_iter().for_each(|t: &mut T| {
                    let r = mutation.mutate(t);
                    each(mutation, r);
                });
            }
        }
    }
}

impl_iter_term!(LinkedList<T>);
impl_iter_term!(HashSet<T>);
impl_iter_term!(BTreeSet<T>);
impl_iter_term!(BinaryHeap<T>);
impl_iter_term!(VecDeque<T>);

// TODO
//
// Below are all the stable `std` types that implement `Debug`, which I figure
// is either all the stable `std` types, or pretty close to them. We need to
// implement `Term` for all of these :)
//
// enum std::borrow::Cow
// enum std::cmp::Ordering
// enum std::collections::Bound
// enum std::collections::btree_map::Entry
// enum std::env::VarError
// enum std::io::CharsError
// enum std::io::ErrorKind
// enum std::io::SeekFrom
// enum std::net::IpAddr
// enum std::net::Ipv6MulticastScope
// enum std::net::Shutdown
// enum std::net::SocketAddr
// enum std::num::FpCategory
// enum std::option::Option
// enum std::os::raw::c_void
// enum std::path::Component
// enum std::path::Prefix
// enum std::result::Result
// enum std::str::pattern::SearchStep
// enum std::string::ParseError
// enum std::sync::TryLockError
// enum std::sync::atomic::Ordering
// enum std::sync::mpsc::RecvTimeoutError
// enum std::sync::mpsc::TryRecvError
// enum std::sync::mpsc::TrySendError
// enum std::thread::LocalKeyState
// struct std::any::TypeId
// struct std::ascii::EscapeDefault
// struct std::cell::BorrowError
// struct std::cell::BorrowMutError
// struct std::cell::Cell
// struct std::cell::Ref
// struct std::cell::RefCell
// struct std::cell::RefMut
// struct std::cell::UnsafeCell
// struct std::char::CharTryFromError
// struct std::char::DecodeUtf16Error
// struct std::char::DecodeUtf8
// struct std::char::EscapeDebug
// struct std::char::EscapeDefault
// struct std::char::EscapeUnicode
// struct std::cmp::Reverse
// struct std::collections::HashMap
// struct std::collections::binary_heap::BinaryHeapPlace
// struct std::collections::binary_heap::Drain
// struct std::collections::binary_heap::IntoIter
// struct std::collections::binary_heap::Iter
// struct std::collections::binary_heap::PeekMut
// struct std::collections::btree_map::BTreeMap
// struct std::collections::btree_map::IntoIter
// struct std::collections::btree_map::Iter
// struct std::collections::btree_map::IterMut
// struct std::collections::btree_map::Keys
// struct std::collections::btree_map::OccupiedEntry
// struct std::collections::btree_map::Range
// struct std::collections::btree_map::RangeMut
// struct std::collections::btree_map::VacantEntry
// struct std::collections::btree_map::Values
// struct std::collections::btree_map::ValuesMut
// struct std::collections::btree_set::Difference
// struct std::collections::btree_set::Intersection
// struct std::collections::btree_set::IntoIter
// struct std::collections::btree_set::Iter
// struct std::collections::btree_set::Range
// struct std::collections::btree_set::SymmetricDifference
// struct std::collections::btree_set::Union
// struct std::collections::hash_map::DefaultHasher
// struct std::collections::hash_map::Drain
// struct std::collections::hash_map::IterMut
// struct std::collections::hash_map::RandomState
// struct std::collections::hash_map::ValuesMut
// struct std::collections::hash_set::Difference
// struct std::collections::hash_set::Intersection
// struct std::collections::hash_set::SymmetricDifference
// struct std::collections::hash_set::Union
// struct std::collections::linked_list::BackPlace
// struct std::collections::linked_list::FrontPlace
// struct std::collections::linked_list::IntoIter
// struct std::collections::linked_list::Iter
// struct std::collections::linked_list::IterMut
// struct std::collections::vec_deque::Drain
// struct std::collections::vec_deque::IntoIter
// struct std::collections::vec_deque::Iter
// struct std::collections::vec_deque::IterMut
// struct std::collections::vec_deque::PlaceBack
// struct std::collections::vec_deque::PlaceFront
// struct std::env::Args
// struct std::env::ArgsOs
// struct std::env::JoinPathsError
// struct std::env::SplitPaths
// struct std::env::Vars
// struct std::env::VarsOs
// struct std::ffi::CStr
// struct std::ffi::CString
// struct std::ffi::FromBytesWithNulError
// struct std::ffi::IntoStringError
// struct std::ffi::NulError
// struct std::ffi::OsStr
// struct std::ffi::OsString
// struct std::fmt::Arguments
// struct std::fmt::Error
// struct std::fs::DirBuilder
// struct std::fs::DirEntry
// struct std::fs::File
// struct std::fs::FileType
// struct std::fs::Metadata
// struct std::fs::OpenOptions
// struct std::fs::Permissions
// struct std::fs::ReadDir
// struct std::hash::BuildHasherDefault
// struct std::hash::SipHasher
// struct std::hash::SipHasher13
// struct std::hash::SipHasher24
// struct std::io::BufReader
// struct std::io::Empty
// struct std::io::Error
// struct std::io::Repeat
// struct std::io::Sink
// struct std::io::Stderr
// struct std::io::StderrLock
// struct std::io::Stdin
// struct std::io::StdinLock
// struct std::io::Stdout
// struct std::io::StdoutLock
// struct std::iter::Chain
// struct std::iter::Cloned
// struct std::iter::Cycle
// struct std::iter::DeprecatedStepBy
// struct std::iter::Empty
// struct std::iter::Enumerate
// struct std::iter::Filter
// struct std::iter::FilterMap
// struct std::iter::FlatMap
// struct std::iter::Fuse
// struct std::iter::Inspect
// struct std::iter::Map
// struct std::iter::Once
// struct std::iter::Peekable
// struct std::iter::Repeat
// struct std::iter::Rev
// struct std::iter::Scan
// struct std::iter::Skip
// struct std::iter::SkipWhile
// struct std::iter::StepBy
// struct std::iter::Take
// struct std::iter::TakeWhile
// struct std::iter::Zip
// struct std::marker::PhantomData
// struct std::mem::Discriminant
// struct std::net::AddrParseError
// struct std::net::Incoming
// struct std::net::Ipv4Addr
// struct std::net::Ipv6Addr
// struct std::net::LookupHost
// struct std::net::SocketAddrV4
// struct std::net::SocketAddrV6
// struct std::net::TcpListener
// struct std::net::TcpStream
// struct std::net::UdpSocket
// struct std::num::ParseFloatError
// struct std::num::ParseIntError
// struct std::num::TryFromIntError
// struct std::num::Wrapping
// struct std::ops::Range
// struct std::ops::RangeFrom
// struct std::ops::RangeFull
// struct std::ops::RangeInclusive
// struct std::ops::RangeTo
// struct std::ops::RangeToInclusive
// struct std::option::IntoIter
// struct std::option::Iter
// struct std::option::IterMut
// struct std::os::unix::net::Incoming
// struct std::os::unix::net::SocketAddr
// struct std::os::unix::net::UnixDatagram
// struct std::os::unix::net::UnixListener
// struct std::os::unix::net::UnixStream
// struct std::panic::Location
// struct std::panic::PanicInfo
// struct std::path::Components
// struct std::path::Display
// struct std::path::Iter
// struct std::path::Path
// struct std::path::PathBuf
// struct std::path::PrefixComponent
// struct std::path::StripPrefixError
// struct std::process::Child
// struct std::process::ChildStderr
// struct std::process::ChildStdin
// struct std::process::ChildStdout
// struct std::process::Command
// struct std::process::ExitStatus
// struct std::process::Output
// struct std::process::Stdio
// struct std::rc::Rc
// struct std::rc::Weak
// struct std::result::IntoIter
// struct std::result::Iter
// struct std::result::IterMut
// struct std::slice::Chunks
// struct std::slice::ChunksMut
// struct std::slice::Iter
// struct std::slice::IterMut
// struct std::slice::RSplit
// struct std::slice::RSplitMut
// struct std::slice::RSplitN
// struct std::slice::RSplitNMut
// struct std::slice::Split
// struct std::slice::SplitMut
// struct std::slice::SplitN
// struct std::slice::SplitNMut
// struct std::slice::Windows
// struct std::str::Bytes
// struct std::str::CharIndices
// struct std::str::Chars
// struct std::str::EncodeUtf16
// struct std::str::Lines
// struct std::str::LinesAny
// struct std::str::MatchIndices
// struct std::str::Matches
// struct std::str::ParseBoolError
// struct std::str::RMatchIndices
// struct std::str::RMatches
// struct std::str::RSplit
// struct std::str::RSplitN
// struct std::str::RSplitTerminator
// struct std::str::Split
// struct std::str::SplitN
// struct std::str::SplitTerminator
// struct std::str::Utf8Error
// struct std::str::pattern::CharPredicateSearcher
// struct std::str::pattern::CharSearcher
// struct std::str::pattern::CharSliceSearcher
// struct std::str::pattern::StrSearcher
// struct std::string::Drain
// struct std::string::FromUtf16Error
// struct std::string::FromUtf8Error
// struct std::string::Splice
// struct std::string::String
// struct std::sync::Arc
// struct std::sync::Barrier
// struct std::sync::BarrierWaitResult
// struct std::sync::Condvar
// struct std::sync::Once
// struct std::sync::OnceState
// struct std::sync::PoisonError
// struct std::sync::WaitTimeoutResult
// struct std::sync::Weak
// struct std::sync::atomic::AtomicBool
// struct std::sync::atomic::AtomicI16
// struct std::sync::atomic::AtomicI32
// struct std::sync::atomic::AtomicI64
// struct std::sync::atomic::AtomicI8
// struct std::sync::atomic::AtomicIsize
// struct std::sync::atomic::AtomicPtr
// struct std::sync::atomic::AtomicU16
// struct std::sync::atomic::AtomicU32
// struct std::sync::atomic::AtomicU64
// struct std::sync::atomic::AtomicU8
// struct std::sync::atomic::AtomicUsize
// struct std::sync::mpsc::Receiver
// struct std::sync::mpsc::RecvError
// struct std::sync::mpsc::Select
// struct std::sync::mpsc::SendError
// struct std::sync::mpsc::Sender
// struct std::sync::mpsc::SyncSender
// struct std::thread::Builder
// struct std::thread::JoinHandle
// struct std::thread::LocalKey
// struct std::thread::Thread
// struct std::thread::ThreadId
// struct std::time::Duration
// struct std::time::Instant
// struct std::time::SystemTime
// struct std::time::SystemTimeError
// struct std::vec::Drain
// struct std::vec::IntoIter
// struct std::vec::PlaceBack
// struct std::vec::Splice
// trait std::any::Any
// trait std::fmt::Debug
// trait std::io::Write
// trait std::marker::Send
// trait std::marker::Sized
// union std::mem::ManuallyDrop
