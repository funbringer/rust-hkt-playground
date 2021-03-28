use crate::functor::{Functor, FunctorInner};
use crate::types::Me;

pub trait Corecursive<T>: Functor {
    type Container;

    // embed :: f (Fix f) -> Fix f
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

    // TODO: maybe we should return a reference
    // project :: Fix f -> f (Fix f)
    fn project(&self) -> Self::Projection;

    // cata :: (f b -> b) -> Fix f -> b
    fn cata<F, R>(&self, mut f: F) -> R
    where
        F: FnMut(Me<Self::Projection, R>) -> R,
        FunctorInner<Self::Projection>: Recursive<T, Projection = Self::Projection>,
    {
        let inner = self
            .project()
            .fmap(|x| x.cata::<&mut dyn FnMut(Me<Self::Projection, R>) -> R, R>(&mut f));
        f(inner)
    }
}
