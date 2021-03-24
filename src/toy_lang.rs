#![allow(dead_code)]

use crate::functor::*;
use crate::types::*;

#[derive(Clone)]
enum Expr<Leaf, T> {
    Value(Leaf),
    Add(T, T),
    Mul(T, T),
    // TODO: add more ops
}

impl<Leaf, T> TypeToType for Expr<Leaf, T> {
    type Me<A> = Expr<Leaf, A>;
}

type Expr_<T> = Expr<T, !>;

impl<Leaf, T> Functor for Expr<Leaf, T> {
    type Inner = T;

    fn fmap<F, R>(self, mut f: F) -> Self::Me<R>
    where
        F: FnMut(Self::Inner) -> R,
    {
        use Expr::*;

        match self {
            Value(x) => Value(x),
            Add(a, b) => Add(f(a), f(b)),
            Mul(a, b) => Mul(f(a), f(b)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fix::*;

    #[test]
    fn test_eval() {
        use super::Expr::*;

        let value = |x| Value(x).embed();
        let add = |a, b| Add(a, b).embed();
        let mul = |a, b| Mul(a, b).embed();

        let tree: ArcFix<Expr_<i32>> = mul(add(value(1), value(2)), value(3));

        let result = tree.cata(|node| match node {
            Value(x) => x,
            Add(a, b) => a + b,
            Mul(a, b) => a * b,
        });

        assert_eq!(result, 9);
    }

    #[test]
    fn test_extract_values() {
        use super::Expr::*;

        let value = |x| Value(x).embed();
        let add = |a, b| Add(a, b).embed();
        let mul = |a, b| Mul(a, b).embed();

        let tree: ArcFix<Expr_<i32>> = mul(add(value(1), value(2)), value(3));

        let result = tree.cata(|node| match node {
            Value(x) => vec![x],
            Add(xs, ys) => [xs, ys].concat(),
            Mul(xs, ys) => [xs, ys].concat(),
        });

        assert_eq!(result, [1, 2, 3]);
    }
}
