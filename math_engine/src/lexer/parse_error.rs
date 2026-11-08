use crate::token::Token;

#[derive(Debug)]
pub struct ParseError {
    message: String,
}

impl ParseError {
    pub(crate) fn new(message: &str) -> Self {
        Self {
            message: message.to_string(),
        }
    }
}