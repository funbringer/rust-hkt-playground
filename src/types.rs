/// `Self` is a reserved keyword, so we use `Me`
pub type Me<T, A> = <T as SelfTypeFamily>::Me<A>;

/// Unfortunately, Rust doesn't have 1st class HKT.
/// This type family allows us to emulate them.
pub trait SelfTypeFamily {
    type Me<T>;
}

/// We have to plug "type holes" with something (* -> *)
type Dummy = !;

impl<T> SelfTypeFamily for Option<T> {
    type Me<A> = Option<A>;
}

pub type Option_ = Option<Dummy>;

impl<T, E> SelfTypeFamily for Result<T, E> {
    type Me<A> = Result<A, E>;
}

pub type Result_<E> = Result<Dummy, E>;

impl<T1, T> SelfTypeFamily for (T1, T) {
    type Me<A> = (T1, A);
}

pub type Tuple2_<T1> = (T1, Dummy);

impl<T1, T2, T> SelfTypeFamily for (T1, T2, T) {
    type Me<A> = (T1, T2, A);
}

pub type Tuple3_<T1, T2> = (T1, T2, Dummy);

impl<T1, T2, T3, T> SelfTypeFamily for (T1, T2, T3, T) {
    type Me<A> = (T1, T2, T3, A);
}

pub type Tuple4_<T1, T2, T3> = (T1, T2, T3, Dummy);

impl<T, const N: usize> SelfTypeFamily for [T; N] {
    type Me<A> = [A; N];
}

pub type Array_<const N: usize> = [Dummy; N];

pub mod vec {
    use super::{Dummy, SelfTypeFamily};

    impl<T> SelfTypeFamily for Vec<T> {
        type Me<A> = Vec<A>;
    }

    pub type Vec_ = Vec<Dummy>;
}

pub mod collections {
    use super::{Dummy, SelfTypeFamily};
    use std::collections::*;

    impl<T> SelfTypeFamily for HashSet<T> {
        type Me<A> = HashSet<A>;
    }

    pub type HashSet_ = HashSet<Dummy>;

    impl<K, T> SelfTypeFamily for HashMap<K, T> {
        type Me<A> = HashMap<K, A>;
    }

    pub type HashMap_<K> = HashMap<K, Dummy>;
}
