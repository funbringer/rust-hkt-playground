#![allow(incomplete_features)]
#![feature(generic_associated_types)]

mod types;
mod functor;
mod fix;

use std::fmt::Debug;
use std::sync::Arc;

use types::*;
use functor::*;
use fix::*;

trait XBuiltin {
    type Output: Debug;
}

trait XExpr: XBuiltin {}
impl<T> XExpr for T where T: XBuiltin {}

// ****************************************

#[allow(dead_code)]
#[derive(Debug)]
enum Expr<Pass: XExpr, T> {
    Builtin(<Pass as XBuiltin>::Output),
    Struct(Vec<(String, Arc<T>)>),
}

// ****************************************

#[derive(Debug)]
struct Type;

#[allow(dead_code)]
#[derive(Debug)]
enum Primitive {
    Int(i32),
}

impl XBuiltin for Type {
    type Output = Primitive;
}

// ****************************************

fn main() {
    println!("{:?}", Some(&1i32).fmap(ToString::to_string));

    // use Primitive::*;
    // use Expr::*;

    // let expr = Struct(vec![("kek".into(), Int(1))]);
    // println!("{:?}", expr);

    let tree: ArcFix<Option_> = None.embed().into();
    if let None = tree.project() {
        println!("yes");
    }

    let tree: ArcFix<Option_> = None.embed();
    if let None = tree.project() {
        println!("yes");
    }

    let none = None.embed();
    let some = |x| Some(x).embed();

    let tree: ArcFix<Option_> = some(some(some(none)));
    let value = tree.cata(|x|
        match x {
            Some(value) => value + 1,
            None => 0,
        }
    );
    println!("{:?}", value);

    let tree: ArcFix<Vec_> = vec![
        vec![
            vec![].embed(),
            vec![].embed(),
        ].embed(),
        vec![
            vec![].embed(),
        ].embed(),
    ].embed();

    let value = tree.cata(|x| {
        x.into_iter().sum::<usize>() + 1
    });
    println!("{:?}", value);
}
