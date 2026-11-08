use meval::Expr;
use crate::function::FunctionExpr;

struct Summation {
    i: i32,
    n: i32,
    seq: FunctionExpr,
}

impl Summation {

    pub(crate) fn new(i: i32, n: i32, seq: &str, expression: Expr) -> Self {
        Self {
            i,
            n,
            seq: FunctionExpr::new(seq, expression),
        }
    }

    pub(crate) fn eval(&self) -> f64 {
        let mut sum = 0.0;
        for i in self.i..=self.n{
            sum += self.seq.eval(i as f64);
        }
        sum
    }
}