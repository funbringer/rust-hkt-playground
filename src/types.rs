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

impl<T> SelfTypeFamily for Vec<T> {
    type Me<A> = Vec<A>;
}

pub type Vec_ = Vec<Dummy>;
