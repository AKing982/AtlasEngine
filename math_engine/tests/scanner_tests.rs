#[path = "../src/lexer/scanner.rs"]
mod scanner;

#[path = "../src/lexer/token.rs"]
mod token;

#[path = "../src/lexer/token_type.rs"]
mod token_type;

#[path = "../src/lexer/literal.rs"]
mod literal;


#[cfg(test)]
mod tests{
    use crate::literal::Literal;
    use crate::scanner;
    use crate::scanner::Scanner;
    use crate::token_type::TokenType;
    #[test]
    fn test_integer() {
        let source = String::from("123");
        let mut scanner = scanner::Scanner::new(source);
        let tokens = scanner.scan_tokens();

        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0].token_type, TokenType::Number);
        assert_eq!(tokens[0].lexeme, "123");
    }

    #[test]
    fn test_decimal(){
        let source = String::from("123.456");
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens();
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0].token_type, TokenType::Number);
        assert_eq!(tokens[0].lexeme, "123.456");
    }

    #[test]
    fn test_addition(){
        let source = String::from("x + y");
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens();
        assert_eq!(tokens.len(), 4);
        assert_eq!(tokens[0].token_type, TokenType::Identifier);
        assert_eq!(tokens[1].token_type, TokenType::Plus);
        assert_eq!(tokens[2].token_type, TokenType::Identifier);
        assert_eq!(tokens[3].token_type, TokenType::Eof);
    }

    #[test]
    fn test_multiple_function_calls() {
        let source = "f = sin(x) + cos(y)";
        let mut scanner = Scanner::new(source.to_string());
        let tokens = scanner.scan_tokens();

        assert_eq!(tokens.len(), 12);

        assert_eq!(tokens[0].token_type, TokenType::Identifier);
        assert_eq!(tokens[1].token_type, TokenType::Equal);
        assert_eq!(tokens[2].token_type, TokenType::Identifier);
        assert_eq!(tokens[3].token_type, TokenType::LeftParen);
        assert_eq!(tokens[4].token_type, TokenType::Identifier);
        assert_eq!(tokens[5].token_type, TokenType::RightParen);
        assert_eq!(tokens[6].token_type, TokenType::Plus);
        assert_eq!(tokens[7].token_type, TokenType::Identifier);
        assert_eq!(tokens[8].token_type, TokenType::LeftParen);
        assert_eq!(tokens[9].token_type, TokenType::Identifier);
        assert_eq!(tokens[10].token_type, TokenType::RightParen);
        assert_eq!(tokens[11].token_type, TokenType::Eof);
    }

    #[test]
    fn test_nested_function_calls() {
        let source = "y = tan(sqrt(2)) / sin(i)";
        let mut scanner = Scanner::new(source.to_string());
        let tokens = scanner.scan_tokens();

        assert_eq!(tokens.len(), 13);

        assert_eq!(tokens[0].token_type, TokenType::Identifier); // y
        assert_eq!(tokens[1].token_type, TokenType::Equal);      // =
        assert_eq!(tokens[2].token_type, TokenType::Identifier); // tan
        assert_eq!(tokens[3].token_type, TokenType::LeftParen);  // (
        assert_eq!(tokens[4].token_type, TokenType::Identifier); // sqrt
        assert_eq!(tokens[5].token_type, TokenType::LeftParen);  // (
        assert_eq!(tokens[6].token_type, TokenType::Number);     // 2
        assert_eq!(tokens[7].token_type, TokenType::RightParen); // )
        assert_eq!(tokens[8].token_type, TokenType::RightParen); // )
        assert_eq!(tokens[9].token_type, TokenType::Slash);      // /
        assert_eq!(tokens[10].token_type, TokenType::Identifier);// sin
        assert_eq!(tokens[11].token_type, TokenType::LeftParen); // (
        assert_eq!(tokens[12].token_type, TokenType::Identifier);// i
    }

    #[test]
    fn test_numeric_literals(){
        let cases = vec![
            "0",
            "123",
            "123.456",
            "0.25",
            "999.99",
        ];
        for input in cases {
            let mut scanner = Scanner::new(input.to_string());
            let tokens = scanner.scan_tokens();
            assert_eq!(
                tokens[0].token_type,
                TokenType::Number,
                "Failed for input: {}",
                input
            );
        }
    }

    #[test]
    fn test_less_than() {
        let source = "x < 5";
        let mut scanner = Scanner::new(source.to_string());
        let tokens = scanner.scan_tokens();

        assert_eq!(tokens.len(), 4);
        assert_eq!(tokens[0].token_type, TokenType::Identifier);
        assert_eq!(tokens[1].token_type, TokenType::Less);
        assert_eq!(tokens[2].token_type, TokenType::Number);
        assert_eq!(tokens[3].token_type, TokenType::Eof);
    }

    #[test]
    fn test_less_equal() {
        let source = "x <= 5";
        let mut scanner = Scanner::new(source.to_string());
        let tokens = scanner.scan_tokens();

        assert_eq!(tokens.len(), 4);
        assert_eq!(tokens[1].token_type, TokenType::LessEqual);
    }

    #[test]
    fn test_greater_than() {
        let source = "y > 10";
        let mut scanner = Scanner::new(source.to_string());
        let tokens = scanner.scan_tokens();

        assert_eq!(tokens.len(), 4);
        assert_eq!(tokens[1].token_type, TokenType::Greater);
    }


    #[test]
    fn test_greater_equal() {
        let source = "y >= 10";
        let mut scanner = Scanner::new(source.to_string());
        let tokens = scanner.scan_tokens();

        assert_eq!(tokens.len(), 4);
        assert_eq!(tokens[1].token_type, TokenType::GreaterEqual);
    }

    #[test]
    fn test_equal_equal() {
        let source = "x == 5";
        let mut scanner = Scanner::new(source.to_string());
        let tokens = scanner.scan_tokens();

        assert_eq!(tokens.len(), 4);
        assert_eq!(tokens[1].token_type, TokenType::EqualEqual);
    }

    #[test]
    fn test_assignment_equal() {
        let source = "x = 5";
        let mut scanner = Scanner::new(source.to_string());
        let tokens = scanner.scan_tokens();

        assert_eq!(tokens.len(), 4);
        assert_eq!(tokens[1].token_type, TokenType::Equal);
    }


    #[test]
    fn test_decimal_without_leading_zero() {
        let source = String::from(".456");

        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens();

        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0].token_type, TokenType::Number);
        assert_eq!(tokens[0].lexeme, ".456");
        assert_eq!(tokens[0].literal, Some(Literal::Number(0.456)));
        assert_eq!(tokens[1].token_type, TokenType::Eof);
    }

    #[test]
    fn test_function_assignments() {
        let source = String::from("x = sin(2), y = tan(3)");

        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens();

        let expected = vec![
            TokenType::Identifier,
            TokenType::Equal,
            TokenType::Identifier,
            TokenType::LeftParen,
            TokenType::Number,
            TokenType::RightParen,
            TokenType::Comma,
            TokenType::Identifier,
            TokenType::Equal,
            TokenType::Identifier,
            TokenType::LeftParen,
            TokenType::Number,
            TokenType::RightParen,
            TokenType::Eof,
        ];

        let actual: Vec<TokenType> = tokens
            .iter()
            .map(|token| token.token_type)
            .collect();

        assert_eq!(actual, expected);
    }


}