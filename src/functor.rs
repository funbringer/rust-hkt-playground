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

impl<T, E> Functor for Result<T, E> {
    type Inner = T;

    fn fmap<F, R>(self, f: F) -> Self::Me<R>
    where
        F: FnMut(Self::Inner) -> R,
    {
        self.map(f)
    }
}

impl<T1, T> Functor for (T1, T) {
    type Inner = T;

    fn fmap<F, R>(self, mut f: F) -> Self::Me<R>
    where
        F: FnMut(Self::Inner) -> R,
    {
        let (a, b) = self;
        (a, f(b))
    }
}

impl<T1, T2, T> Functor for (T1, T2, T) {
    type Inner = T;

    fn fmap<F, R>(self, mut f: F) -> Self::Me<R>
    where
        F: FnMut(Self::Inner) -> R,
    {
        let (a, b, c) = self;
        (a, b, f(c))
    }
}

impl<T1, T2, T3, T> Functor for (T1, T2, T3, T) {
    type Inner = T;

    fn fmap<F, R>(self, mut f: F) -> Self::Me<R>
    where
        F: FnMut(Self::Inner) -> R,
    {
        let (a, b, c, d) = self;
        (a, b, c, f(d))
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

pub mod vec {
    use super::Functor;

    impl<T> Functor for Vec<T> {
        type Inner = T;

        fn fmap<F, R>(self, f: F) -> Self::Me<R>
        where
            F: FnMut(Self::Inner) -> R,
        {
            self.into_iter().map(f).collect()
        }
    }
}

pub mod collections {
    use super::Functor;
    use std::collections::*;

    impl<K: Eq + std::hash::Hash, T> Functor for HashMap<K, T> {
        type Inner = T;

        fn fmap<F, R>(self, mut f: F) -> Self::Me<R>
        where
            F: FnMut(Self::Inner) -> R,
        {
            self.into_iter().map(|(k, v)| (k, f(v))).collect()
        }
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
    fn test_result() {
        let mut value; // reuse for type

        value = Ok(&100).fmap(usize::to_string);
        assert_matches!(value.as_deref(), Ok("100"));

        value = Err(()).fmap(|x| x);
        assert_matches!(value, Err(()));
    }

    #[test]
    fn test_tuples() {
        let value = (1, 2).fmap(|x| x * 100);
        assert_eq!(value, (1, 200));

        let value = (1, 2, 3).fmap(|x| x * 100);
        assert_eq!(value, (1, 2, 300));

        let value = (1, 2, 3, 4).fmap(|x| x * 100);
        assert_eq!(value, (1, 2, 3, 400));
    }

    #[test]
    fn test_array() {
        let value: [String; 0] = [].fmap(From::<&str>::from);
        assert_matches!(value, []);

        let value = [0; 100].fmap(|x| x + 1);
        assert_eq!(value, [1; 100]);
    }

    #[test]
    fn test_vec() {
        let value = vec!["kek"].fmap(String::from);
        assert_matches!(value[0].as_ref(), "kek");
    }

    #[test]
    fn test_hashmap() {
        use std::collections::HashMap;

        let value = vec![(1, "foo"), (2, "bar")]
            .into_iter()
            .collect::<HashMap<i32, &str>>()
            .fmap(String::from);

        assert_matches!(value[&1].as_ref(), "foo");
        assert_matches!(value[&2].as_ref(), "bar");
    }
}
