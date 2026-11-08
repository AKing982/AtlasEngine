use crate::ast_expr::AstExpr;
use crate::token::Token;

pub enum AstStmt {
    Expression {
        expression: AstExpr,
    },
    Var {
        name: Token,
        initializer: Option<AstExpr>,
    }
}