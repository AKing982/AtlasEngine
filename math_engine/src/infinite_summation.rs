use crate::expr::Expr;
use crate::function::FunctionExpr;
pub(crate) struct InfiniteSummation {
    start: i32,
    seq: FunctionExpr,
    max_terms: usize,
}

impl InfiniteSummation {
    pub(crate) fn new(
        start: i32,
        seq: &str,
        expression: Expr,
        max_terms: usize,
    ) -> Self {
        Self {
            start,
            seq: FunctionExpr::new(seq, expression),
            max_terms
        }

    }

    pub(crate) fn eval(&self) -> f64{
        let mut sum = 0.0;
        for i in 0..self.max_terms {
            let n = self.start + i as i32;
            sum += self.seq.eval(n as f64);
        }
        sum
    }
}

