use crate::functor::{Functor, FunctorInner};
use crate::types::{TypeToType, Me};
use std::borrow::Borrow;
use std::sync::Arc;

pub trait Corecursive<T> {
    type Container;

    fn embed(&self) -> Self::Container;

    // TODO: do we need this one?
    fn embed_<R>(&self) -> R
        where
            Self::Container: Into<R>
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

            FunctorInner<Self::Projection>:
                Recursive<T, Projection=Self::Projection>,
    {
        let inner = self.project().fmap(|x| x.cata_mut(f));
        f(inner)
    }

    fn cata<F, R>(&self, mut f: F) -> R
        where
            F: FnMut(Me<Self::Projection, R>) -> R,

            FunctorInner<Self::Projection>:
                Recursive<T, Projection=Self::Projection>,
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
