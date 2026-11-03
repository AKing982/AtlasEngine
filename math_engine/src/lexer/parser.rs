pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub(crate) fn new(&self, tokens: Vec<Token>) -> Self {
        Self{
            tokens,
            usize: 0,
        }
    }

    fn expression(&self) -> Expr {
        return self.equality();
    }



    fn match_token(&self, types: &[TokenType]) -> bool {
        for token_type in types {
            if self.check(*token_type){
                self.advance();
                return true;
            }
        }
        return false;
    }

    fn check_token(&self, token_type: TokenType) -> bool {
        !self.is_at_end() && self.peek().token_type == token_type
    }

    fn advance(&mut self) -> Token {
        if self.is_at_end() {
            self.current += 1;
        }
        return self.previous();
    }

    fn previous(&mut self) -> Token {
        return self.tokens[self.current - 1];
    }

    fn peek(&self) -> Token {
        return self.tokens[self.current];
    }

    fn is_at_end(&self) -> bool {
        return self.peek().token_type == EOF;
    }

    fn primary(&mut self) -> Expr {
        if self.match_token(TokenType::False) {
            return Expr::Literal(Value::Boolean(false));
        }
        if self.match_token(&[TokenType::True]){
            return Expr::Literal(Value::Boolean(true));
        }
        if self.match_token(&[TokenType::Nil]){
            return Expr::Literal(Value::Nil);
        }
        if self.match_token(&[
            TokenType::Number,
            TokenType::String,
        ]){
            return Expr::Literal(
                Value::from(self.previous().literal.clone())
            );
        }
    }

    fn unary(&self) -> Expr {
        if self.match_token(&[
            TokenType::Bang,
            TokenType::Minus
        ]){
            let operator = self.previous().clone();
            let right = self.unary();
            return Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            }
        }
    }

    fn factor(&self) -> Expr {
        let mut expr = self.unary();
        while self.match_token(&[
            TokenType::Slash,
            TokenType::Star
        ]){
            let operator = self.previous().clone();
            let right = self.unary();
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        expr
    }

    fn comparison(&self) -> Expr {
        let mut expr = self.term();
        while self.match_token(&[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual
        ])
        {
            let operator = self.previous().clone();
            let right = self.term();
            expr = Expr::Binary{
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        expr
    }

    fn equality(&mut self) -> Expr {
        let mut expr = self.comparison();
        while self.match_token(&[
            TokenType::BangEqual,
            TokenType::EqualEqual,
        ]){
            let operator = self.previous().clone();
            let right = self.comparison();
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        expr
    }

}