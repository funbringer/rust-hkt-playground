use crate::types::SelfTypeFamily;

pub type FunctorInner<T> = <T as Functor>::Inner;

pub trait Functor: SelfTypeFamily {
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

impl<T, const N: usize> Functor for [T; N] {
    type Inner = T;

    fn fmap<F, R>(self, mut f: F) -> Self::Me<R>
    where
        F: FnMut(Self::Inner) -> R,
    {
        use std::mem::MaybeUninit;

        let mut result: [MaybeUninit<R>; N] = MaybeUninit::uninit_array();

        for (i, x) in std::array::IntoIter::new(self).enumerate() {
            result[i] = MaybeUninit::new(f(x));
        }

        // SAFETY: we have initialized all elements
        unsafe { MaybeUninit::array_assume_init(result) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_option() {
        let value = Some(&100).fmap(i32::to_string);
        assert_matches!(value.as_deref(), Some("100"));
    }

    #[test]
    fn test_vec() {
        let value = vec!["kek"].fmap(String::from);
        assert_matches!(value[0].as_ref(), "kek");
    }

    #[test]
    fn test_array() {
        let value: [String; 0] = [].fmap(From::<&str>::from);
        assert_matches!(value, []);
    }
}
