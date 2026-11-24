#[path = "../src/lexer/scanner.rs"]
mod scanner;

#[path = "../src/lexer/token.rs"]
mod token;

#[path = "../src/lexer/token_type.rs"]
mod token_type;

#[path = "../src/lexer/literal.rs"]
mod literal;

#[path = "../src/complex.rs"]
mod complex;


#[cfg(test)]
mod tests{
    use crate::literal::Literal;
    use crate::scanner::Scanner;
    use crate::token_type::TokenType;
    use super::*;

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
    fn test_quadratic_polynomial(){
        let source = String::from("x^2 + y^2");
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens();
        assert_eq!(tokens.len(), 8);
        assert_eq!(tokens[0].token_type, TokenType::Identifier);
        assert_eq!(tokens[1].token_type, TokenType::Caret);
        assert_eq!(tokens[2].token_type, TokenType::Number);
        assert_eq!(tokens[3].token_type, TokenType::Plus);
        assert_eq!(tokens[4].token_type, TokenType::Identifier);
        assert_eq!(tokens[5].token_type, TokenType::Caret);
        assert_eq!(tokens[6].token_type, TokenType::Number);
    }

    #[test]
    fn test_quadratic_associativity(){
        let source = String::from("(x^2 + y^2) + 4");
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens();
        assert_eq!(tokens.len(), 12);
        assert_eq!(tokens[0].token_type, TokenType::LeftParen);
        assert_eq!(tokens[1].token_type, TokenType::Identifier);
        assert_eq!(tokens[2].token_type, TokenType::Caret);
        assert_eq!(tokens[3].token_type, TokenType::Number);
        assert_eq!(tokens[4].token_type, TokenType::Plus);
        assert_eq!(tokens[5].token_type, TokenType::Identifier);
        assert_eq!(tokens[6].token_type, TokenType::Caret);
        assert_eq!(tokens[7].token_type, TokenType::Number);
        assert_eq!(tokens[8].token_type, TokenType::RightParen);
        assert_eq!(tokens[9].token_type, TokenType::Plus);
        assert_eq!(tokens[10].token_type, TokenType::Number);
    }

    #[test]
    fn test_variable_and_expression(){
        let source = String::from("var x = 1; x^2 + 4 = 5;");
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens();
        assert_eq!(tokens.len(), 14);
        assert_eq!(tokens[0].token_type, TokenType::Var);
        assert_eq!(tokens[1].token_type, TokenType::Identifier);
        assert_eq!(tokens[2].token_type, TokenType::Equal);
        assert_eq!(tokens[3].token_type, TokenType::Number);
        assert_eq!(tokens[4].token_type, TokenType::SemiColon);
        assert_eq!(tokens[5].token_type, TokenType::Identifier);
        assert_eq!(tokens[6].token_type, TokenType::Caret);
        assert_eq!(tokens[7].token_type, TokenType::Number);
        assert_eq!(tokens[8].token_type, TokenType::Plus);
        assert_eq!(tokens[9].token_type, TokenType::Number);
        assert_eq!(tokens[10].token_type, TokenType::Equal);
        assert_eq!(tokens[11].token_type, TokenType::Number);
        assert_eq!(tokens[12].token_type, TokenType::SemiColon);
    }

    #[test]
    fn test_multiple_function_calls() {
        let source = "f = sin(x) + cos(y)";
        let mut scanner = Scanner::new(source.to_string());
        let tokens = scanner.scan_tokens();

        assert_eq!(tokens.len(), 12);

        assert_eq!(tokens[0].token_type, TokenType::Identifier); // f
        assert_eq!(tokens[1].token_type, TokenType::Equal);      // =
        assert_eq!(tokens[2].token_type, TokenType::Sin);        // sin
        assert_eq!(tokens[3].token_type, TokenType::LeftParen);  // (
        assert_eq!(tokens[4].token_type, TokenType::Identifier); // x
        assert_eq!(tokens[5].token_type, TokenType::RightParen); // )
        assert_eq!(tokens[6].token_type, TokenType::Plus);       // +
        assert_eq!(tokens[7].token_type, TokenType::Cos);        // cos
        assert_eq!(tokens[8].token_type, TokenType::LeftParen);  // (
        assert_eq!(tokens[9].token_type, TokenType::Identifier); // y
        assert_eq!(tokens[10].token_type, TokenType::RightParen);// )
        assert_eq!(tokens[11].token_type, TokenType::Eof);
    }

    #[test]
    fn test_nested_function_calls() {
        let source = "y = tan(sqrt(2)) / sin(i)";
        let mut scanner = Scanner::new(source.to_string());
        let tokens = scanner.scan_tokens();

        assert_eq!(tokens.len(), 15);

        assert_eq!(tokens[0].token_type, TokenType::Identifier);
        assert_eq!(tokens[1].token_type, TokenType::Equal);
        assert_eq!(tokens[2].token_type, TokenType::Tan);
        assert_eq!(tokens[3].token_type, TokenType::LeftParen);
        assert_eq!(tokens[4].token_type, TokenType::Sqrt);
        assert_eq!(tokens[5].token_type, TokenType::LeftParen);
        assert_eq!(tokens[6].token_type, TokenType::Number);
        assert_eq!(tokens[7].token_type, TokenType::RightParen);
        assert_eq!(tokens[8].token_type, TokenType::RightParen);
        assert_eq!(tokens[9].token_type, TokenType::Slash);
        assert_eq!(tokens[10].token_type, TokenType::Sin);
        assert_eq!(tokens[11].token_type, TokenType::LeftParen);
        assert_eq!(tokens[12].token_type, TokenType::I);
        assert_eq!(tokens[13].token_type, TokenType::RightParen);
        assert_eq!(tokens[14].token_type, TokenType::Eof);
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
            TokenType::Identifier, // x
            TokenType::Equal,      // =
            TokenType::Sin,        // sin
            TokenType::LeftParen,  // (
            TokenType::Number,     // 2
            TokenType::RightParen, // )
            TokenType::Comma,      // ,
            TokenType::Identifier, // y
            TokenType::Equal,      // =
            TokenType::Tan,        // tan
            TokenType::LeftParen,  // (
            TokenType::Number,     // 3
            TokenType::RightParen, // )
            TokenType::Eof,
        ];

        let actual: Vec<TokenType> = tokens
            .iter()
            .map(|token| token.token_type.clone())
            .collect();

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_vector_creation_tokens() {
        let tokens = scan("vector(1,2,3);");

        assert_eq!(tokens, vec![
            TokenType::Vector,
            TokenType::LeftParen,
            TokenType::Number,
            TokenType::Comma,
            TokenType::Number,
            TokenType::Comma,
            TokenType::Number,
            TokenType::RightParen,
            TokenType::SemiColon,
            TokenType::Eof,
        ]);
    }

    #[test]
    fn test_vector_addition_tokens() {
        let tokens = scan("vector(1,2,3) + vector(0,0,1);");

        assert_eq!(tokens, vec![
            TokenType::Vector,
            TokenType::LeftParen,
            TokenType::Number,
            TokenType::Comma,
            TokenType::Number,
            TokenType::Comma,
            TokenType::Number,
            TokenType::RightParen,
            TokenType::Plus,
            TokenType::Vector,
            TokenType::LeftParen,
            TokenType::Number,
            TokenType::Comma,
            TokenType::Number,
            TokenType::Comma,
            TokenType::Number,
            TokenType::RightParen,
            TokenType::SemiColon,
            TokenType::Eof,
        ]);
    }

    #[test]
    fn test_vector_subtraction_tokens() {
        let tokens = scan("vector(1,2,3) - vector(0,0,1);");

        assert_eq!(tokens, vec![
            TokenType::Vector,
            TokenType::LeftParen,
            TokenType::Number,
            TokenType::Comma,
            TokenType::Number,
            TokenType::Comma,
            TokenType::Number,
            TokenType::RightParen,
            TokenType::Minus,
            TokenType::Vector,
            TokenType::LeftParen,
            TokenType::Number,
            TokenType::Comma,
            TokenType::Number,
            TokenType::Comma,
            TokenType::Number,
            TokenType::RightParen,
            TokenType::SemiColon,
            TokenType::Eof,
        ]);
    }

    #[test]
    fn test_vector_scalar_multiplication_tokens() {
        let tokens = scan("vector(1,2,3) * 2;");

        assert_eq!(tokens, vec![
            TokenType::Vector,
            TokenType::LeftParen,
            TokenType::Number,
            TokenType::Comma,
            TokenType::Number,
            TokenType::Comma,
            TokenType::Number,
            TokenType::RightParen,
            TokenType::Star,
            TokenType::Number,
            TokenType::SemiColon,
            TokenType::Eof,
        ]);
    }

    #[test]
    fn test_scalar_vector_multiplication_tokens() {
        let tokens = scan("2 * vector(1,2,3);");

        assert_eq!(tokens, vec![
            TokenType::Number,
            TokenType::Star,
            TokenType::Vector,
            TokenType::LeftParen,
            TokenType::Number,
            TokenType::Comma,
            TokenType::Number,
            TokenType::Comma,
            TokenType::Number,
            TokenType::RightParen,
            TokenType::SemiColon,
            TokenType::Eof,
        ]);
    }
    #[test]
    fn test_dot_product_tokens() {
        let tokens = scan(
            "dot(vector(1,2,3),vector(4,5,6));"
        );

        assert_eq!(tokens, vec![
            TokenType::DotProduct,
            TokenType::LeftParen,
            TokenType::Vector,
            TokenType::LeftParen,
            TokenType::Number,
            TokenType::Comma,
            TokenType::Number,
            TokenType::Comma,
            TokenType::Number,
            TokenType::RightParen,
            TokenType::Comma,
            TokenType::Vector,
            TokenType::LeftParen,
            TokenType::Number,
            TokenType::Comma,
            TokenType::Number,
            TokenType::Comma,
            TokenType::Number,
            TokenType::RightParen,
            TokenType::RightParen,
            TokenType::SemiColon,
            TokenType::Eof,
        ]);
    }

    #[test]
    fn test_norm_tokens() {
        let tokens = scan("norm(vector(1,2,3));");

        assert_eq!(tokens, vec![
            TokenType::Norm,
            TokenType::LeftParen,
            TokenType::Vector,
            TokenType::LeftParen,
            TokenType::Number,
            TokenType::Comma,
            TokenType::Number,
            TokenType::Comma,
            TokenType::Number,
            TokenType::RightParen,
            TokenType::RightParen,
            TokenType::SemiColon,
            TokenType::Eof,
        ]);
    }

    #[test]
    fn test_norm_variable_tokens() {
        let tokens = scan("norm(v);");

        assert_eq!(tokens, vec![
            TokenType::Norm,
            TokenType::LeftParen,
            TokenType::Identifier,
            TokenType::RightParen,
            TokenType::SemiColon,
            TokenType::Eof,
        ]);
    }

    #[test]
    fn test_norm_arithmetic_tokens() {
        let tokens = scan("norm(vector(3,4)) + 2;");

        assert_eq!(tokens, vec![
            TokenType::Norm,
            TokenType::LeftParen,
            TokenType::Vector,
            TokenType::LeftParen,
            TokenType::Number,
            TokenType::Comma,
            TokenType::Number,
            TokenType::RightParen,
            TokenType::RightParen,
            TokenType::Plus,
            TokenType::Number,
            TokenType::SemiColon,
            TokenType::Eof,
        ]);
    }

    #[test]
    fn test_matrix_tokens() {
        let tokens = scan("matrix([1,2,3],[4,5,6],[7,8,9]);");

        assert_eq!(tokens, vec![
            TokenType::Matrix,
            TokenType::LeftParen,

            // First row
            TokenType::LeftBracket,
            TokenType::Number,
            TokenType::Comma,
            TokenType::Number,
            TokenType::Comma,
            TokenType::Number,
            TokenType::RightBracket,
            TokenType::Comma,

            // Second row
            TokenType::LeftBracket,
            TokenType::Number,
            TokenType::Comma,
            TokenType::Number,
            TokenType::Comma,
            TokenType::Number,
            TokenType::RightBracket,
            TokenType::Comma,

            // Third row
            TokenType::LeftBracket,
            TokenType::Number,
            TokenType::Comma,
            TokenType::Number,
            TokenType::Comma,
            TokenType::Number,
            TokenType::RightBracket,

            TokenType::RightParen,
            TokenType::SemiColon,
            TokenType::Eof,
        ]);
    }

    #[test]
    fn test_dimension_vector_tokens() {
        let tokens = scan("dim(vector(1,2,3));");

        assert_eq!(tokens, vec![
            TokenType::Dimension,
            TokenType::LeftParen,
            TokenType::Vector,
            TokenType::LeftParen,
            TokenType::Number,
            TokenType::Comma,
            TokenType::Number,
            TokenType::Comma,
            TokenType::Number,
            TokenType::RightParen,
            TokenType::RightParen,
            TokenType::SemiColon,
            TokenType::Eof,
        ]);
    }

    #[test]
    fn test_dimension_variable_tokens() {
        let tokens = scan("dim(v);");

        assert_eq!(tokens, vec![
            TokenType::Dimension,
            TokenType::LeftParen,
            TokenType::Identifier,
            TokenType::RightParen,
            TokenType::SemiColon,
            TokenType::Eof,
        ]);
    }

    #[test]
    fn test_dimension_arithmetic_tokens() {
        let tokens = scan("dim(v) + 2;");

        assert_eq!(tokens, vec![
            TokenType::Dimension,
            TokenType::LeftParen,
            TokenType::Identifier,
            TokenType::RightParen,
            TokenType::Plus,
            TokenType::Number,
            TokenType::SemiColon,
            TokenType::Eof,
        ]);
    }

    #[test]
    fn test_dimension_vector_addition_tokens() {
        let tokens = scan("dim(vector(1,2) + vector(3,4));");

        assert_eq!(tokens, vec![
            TokenType::Dimension,
            TokenType::LeftParen,

            TokenType::Vector,
            TokenType::LeftParen,
            TokenType::Number,
            TokenType::Comma,
            TokenType::Number,
            TokenType::RightParen,

            TokenType::Plus,

            TokenType::Vector,
            TokenType::LeftParen,
            TokenType::Number,
            TokenType::Comma,
            TokenType::Number,
            TokenType::RightParen,

            TokenType::RightParen,
            TokenType::SemiColon,
            TokenType::Eof,
        ]);
    }

    #[test]
    fn test_dimension_empty_arguments_tokens() {
        let tokens = scan("dim();");

        assert_eq!(tokens, vec![
            TokenType::Dimension,
            TokenType::LeftParen,
            TokenType::RightParen,
            TokenType::SemiColon,
            TokenType::Eof,
        ]);
    }

    fn scan(source: &str) -> Vec<TokenType> {
        let mut scanner = Scanner::new(source.to_string());
        scanner
            .scan_tokens()
            .iter()
            .map(|token| token.token_type.clone())
        .collect()
    }


}