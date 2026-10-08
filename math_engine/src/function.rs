use crate::expr::Expr;
#[derive(Debug, Clone)]
pub(crate) struct FunctionExpr {
    expr: meval::Expr,
    pub(crate) expr_str: String,
    expression: Expr
}

impl FunctionExpr {

    pub(crate) fn new(exp_str: &str, expression: Expr) -> Self {
        let expr = exp_str.parse::<meval::Expr>().unwrap();
        let expression_str = exp_str.to_string();

        Self {
            expr,
            expr_str: expression_str,
            expression,
        }
    }

    pub(crate) fn eval(&self, t: f64) -> f64 {
        let f = self.expr.clone().bind("t").unwrap();
        f(t)
    }
}