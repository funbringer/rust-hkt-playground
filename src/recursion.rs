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
    fn project(&self) -> Self::Projection;

    // cata :: (f b -> b) -> Fix f -> b
    // cata f = f . fmap (cata f) . project
    fn cata_mut<F, R>(&self, f: &mut F) -> R
    where
        F: FnMut(Me<Self::Projection, R>) -> R,
        FunctorInner<Self::Projection>: Recursive<T, Projection = Self::Projection>,
        // Explore alternative:
        // Self::Projection: Functor<Inner = Self>,
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
