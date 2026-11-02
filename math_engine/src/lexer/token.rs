use crate::literal::Literal;
use crate::token_type::TokenType;

#[derive(Debug, Clone)]
pub(crate) struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: Option<Literal>,
}