
use super::ast_expr::AstExpr;
pub struct AstPrinter;

impl AstPrinter {
    pub fn new() -> Self {
        Self
    }

    pub fn print(&self, expr: &AstExpr) -> String {
        match expr {
            AstExpr::Binary {
                left,
                operator,
                right,
            } => {
                self.parenthesize(
                    &operator.lexeme,
                    &[left.as_ref(), right.as_ref()],
                )
            }

            AstExpr::Grouping { expression } => {
                self.parenthesize("group", &[expression.as_ref()])
            }

            AstExpr::Literal { value } => {
                format!("{:?}", value)
            }
            AstExpr::Variable {name} => {
                name.lexeme.clone()
            }
            AstExpr::Unary {
                operator,
                right,
            } => {
                self.parenthesize(
                    &operator.lexeme,
                    &[right.as_ref()],
                )
            },
            AstExpr::Assign {name, value} => {
                self.parenthesize(
                    &format!("= {}", name.lexeme),
                    &[value.as_ref()],
                )
            }
        }
    }

    fn parenthesize(
        &self,
        name: &str,
        expressions: &[&AstExpr],
    ) -> String {
        let mut builder = format!("({}", name);

        for expr in expressions {
            builder.push(' ');
            builder.push_str(&self.print(expr));
        }

        builder.push(')');
        builder
    }
}