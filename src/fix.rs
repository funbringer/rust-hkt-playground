pub use crate::recursion::{Corecursive, Recursive};

use crate::functor::Functor;
use crate::types::SelfTypeFamily;
use std::borrow::Borrow;
use std::sync::Arc;

pub type ArcFix<T> = Arc<Fix<T>>;
pub type Unfix<T> = <T as SelfTypeFamily>::Me<ArcFix<T>>;

// newtype Fix f = Fix {unFix :: f (Fix f)}
#[repr(transparent)]
pub struct Fix<F: SelfTypeFamily>(Unfix<F>);

impl<T> Corecursive<T> for Unfix<T>
where
    T: SelfTypeFamily,
    Unfix<T>: Clone + Functor,
{
    type Container = ArcFix<T>;

    fn embed(&self) -> Self::Container {
        Fix(self.clone()).into()
    }
}

impl<T, Tree> Recursive<T> for Tree
where
    T: SelfTypeFamily,
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
    fn test_bare_fix() {
        // We don't have to wrap top Fix
        let tree: Fix<Option_> = Fix(None);

        if let None = tree.project() {}

        tree.cata(|x| {
            let x: ArcFix<Option_> = x.embed();
            x
        });
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
