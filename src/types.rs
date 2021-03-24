pub type Me<T, A> = <T as TypeToType>::Me<A>;

// Unfortunately, Rust doesn't have 1st class HKT
pub trait TypeToType {
    type Me<T>;
}

/// We have to plug "type holes" with something (* -> *)
type Dummy = ();

impl<T> TypeToType for Option<T> {
    type Me<A> = Option<A>;
}

/// Higer-kinded Option alias
pub type Option_ = Option<Dummy>;

impl<T> TypeToType for Vec<T> {
    type Me<A> = Vec<A>;
}

/// Higer-kinded Vec alias
pub type Vec_ = Vec<Dummy>;
