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

impl<T> SelfTypeFamily for Vec<T> {
    type Me<A> = Vec<A>;
}

pub type Vec_ = Vec<Dummy>;

impl<T, const N: usize> SelfTypeFamily for [T; N] {
    type Me<A> = [A; N];
}

pub type Array_<const N: usize> = [Dummy; N];
