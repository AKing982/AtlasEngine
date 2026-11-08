use meval::Expr;
use crate::function::FunctionExpr;

#[derive(Debug, Clone)]
pub(crate) struct Fraction {
    pub(crate) numerator: FunctionExpr,
    pub(crate) denominator: FunctionExpr,
}

impl Fraction {

    pub(crate) fn new(numerator: &str, numerator_expr: Expr, denominator: &str, denominator_expr: Expr) -> Self {
        Self{
            numerator: FunctionExpr::new(numerator, numerator_expr),
            denominator: FunctionExpr::new(denominator, denominator_expr),
        }
    }

    pub(crate) fn eval(&self, t: f64) -> f64 {
        let num_val = self.numerator.eval(t);
        let den_val = self.denominator.eval(t);
        num_val / den_val
    }
}