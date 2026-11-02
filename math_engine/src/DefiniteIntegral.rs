use crate::expr::Expr;
use crate::function::FunctionExpr;

struct DefiniteIntegral {
    a: f64,
    b: f64,
    f: FunctionExpr,
}

impl DefiniteIntegral {
    pub(crate) fn new(a: f64, b: f64, seq: &str, expr: Expr) -> Self {
        Self {
            a,
            b,
            f: FunctionExpr::new(seq, expr),
        }
    }
    
}