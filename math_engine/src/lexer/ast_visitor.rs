pub trait ExprVisitor {
    fn visit_literal_expr(
        &mut self,
        expr: &AstExpr,
    ) -> Result<Value, RuntimeError>;
}