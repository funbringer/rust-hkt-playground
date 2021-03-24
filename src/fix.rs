use crate::functor::{Functor, FunctorInner};
use crate::types::{Me, TypeToType};
use std::borrow::Borrow;
use std::sync::Arc;

pub trait Corecursive<T> {
    type Container;

    fn embed(&self) -> Self::Container;

    // TODO: do we need this one?
    fn embed_<R>(&self) -> R
    where
        Self::Container: Into<R>,
    {
        self.embed().into()
    }
}

pub trait Recursive<T> {
    type Projection: Functor;

    fn project(&self) -> Self::Projection;

    // cata :: (f b -> b) -> Fix f -> b
    // cata f = f . fmap (cata f) . project
    fn cata_mut<F, R>(&self, f: &mut F) -> R
    where
        F: FnMut(Me<Self::Projection, R>) -> R,
        FunctorInner<Self::Projection>: Recursive<T, Projection = Self::Projection>,
    {
        let inner = self.project().fmap(|x| x.cata_mut(f));
        f(inner)
    }

    fn cata<F, R>(&self, mut f: F) -> R
    where
        F: FnMut(Me<Self::Projection, R>) -> R,
        FunctorInner<Self::Projection>: Recursive<T, Projection = Self::Projection>,
    {
        // TODO: find a way to merge these methods
        self.cata_mut(&mut f)
    }

    // TODO: add para
}

pub type ArcFix<T> = Arc<Fix<T>>;
pub type Unfix<T> = <T as TypeToType>::Me<ArcFix<T>>;

// newtype Fix f = Fix {unFix :: f (Fix f)}
pub struct Fix<F: TypeToType>(Unfix<F>);

impl<T> Corecursive<T> for Unfix<T>
where
    T: TypeToType,
    Unfix<T>: Clone,
{
    type Container = ArcFix<T>;

    fn embed(&self) -> Self::Container {
        Fix(self.clone()).into()
    }
}

impl<T, Tree> Recursive<T> for Tree
where
    T: TypeToType,
    Tree: Borrow<Fix<T>>,
    Unfix<T>: Functor + Clone,
{
    type Projection = Unfix<T>;

    fn project(&self) -> Self::Projection {
        let Fix(inner) = self.borrow();
        inner.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{Option_, Vec_};

    #[test]
    fn test_embed_project() {
        let tree: ArcFix<Option_> = None.embed().into();
        if let None = tree.project() {}

        let tree: ArcFix<Option_> = None.embed_();
        if let None = tree.project() {}

        let tree: ArcFix<Option_> = None.embed();
        if let None = tree.project() {}
    }

    #[test]
    fn test_tree_option() {
        let none = None.embed();
        let some = |x| Some(x).embed();

        let tree: ArcFix<Option_> = some(some(some(none)));

        let value = tree.cata(|x| match x {
            Some(value) => value + 1,
            None => 0,
        });

        assert_eq!(value, 3);
    }

    #[test]
    fn test_tree_vec() {
        let tree: ArcFix<Vec_> = vec![
            vec![vec![].embed(), vec![].embed()].embed(),
            vec![vec![].embed()].embed(),
        ]
        .embed();

        let value = tree.cata(|x| x.into_iter().sum::<usize>() + 1);

        assert_eq!(value, 6);
    }
}
