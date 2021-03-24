// TODO: do something with these

use std::fmt::Debug;
use std::sync::Arc;

trait XBuiltin {
    type Output: Debug;
}

trait XExpr: XBuiltin {}
impl<T> XExpr for T where T: XBuiltin {}

#[allow(dead_code)]
#[derive(Debug)]
enum Expr<Pass: XExpr, T> {
    Builtin(<Pass as XBuiltin>::Output),
    Struct(Vec<(String, Arc<T>)>),
}

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
