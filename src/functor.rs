use crate::types::TypeToType;

pub type FunctorInner<T> = <T as Functor>::Inner;

pub trait Functor: TypeToType {
    type Inner;

    fn fmap<F, R>(self, f: F) -> Self::Me<R>
    where
        F: FnMut(Self::Inner) -> R;
}

impl<T> Functor for Option<T> {
    type Inner = T;

    fn fmap<F, R>(self, f: F) -> Self::Me<R>
    where
        F: FnMut(Self::Inner) -> R,
    {
        self.map(f)
    }
}

impl<T> Functor for Vec<T> {
    type Inner = T;

    fn fmap<F, R>(self, f: F) -> Self::Me<R>
    where
        F: FnMut(Self::Inner) -> R,
    {
        self.into_iter().map(f).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let value = Some(&100i32).fmap(ToString::to_string);
        assert_matches!(value.unwrap().as_ref(), "100");
    }
}
