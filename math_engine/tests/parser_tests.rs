

#[cfg(test)]
mod tests {
    use math_engine::ast_expr::AstExpr;
    use math_engine::ast_stmt::AstStmt;
    use math_engine::literal::Literal;
    use math_engine::parser::Parser;
    use math_engine::scanner::Scanner;
    use math_engine::token_type::TokenType;

    fn create_parser(source: String) -> Parser {
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens();
        Parser::new(tokens)
    }

    #[test]
    fn test_primary_pi() {
        let pi_string = String::from("pi");
        let mut parser = create_parser(pi_string);
        let expr = parser.primary().unwrap();
        match expr {
            AstExpr::Literal {
                value: Literal::Number(value),
            } => {
                assert!((value - std::f64::consts::PI).abs() < 1e-12);
            }
            _ => panic!("Expected pi literal"),
        }
    }

    #[test]
    fn test_primary_e(){
        let mut parser = create_parser("e".to_string());
        let expr = parser.primary().unwrap();
        match expr{
            AstExpr::Literal {
                value: Literal::Number(value),
            } => {
                assert!((value - std::f64::consts::E).abs() < 1e-12);
            }
            _ => panic!("Expected e"),
        }
    }

    #[test]
    fn test_primary_number(){
        let mut parser = create_parser("42.5".to_string());
        let expr = parser.primary().unwrap();
        assert!(matches!(
            expr,
            AstExpr::Literal {
                value: Literal::Number(value),
            }
        ));
    }

    #[test]
    fn test_primary_variable(){
        let mut parser = create_parser("x".to_string());
        let expr = parser.primary().unwrap();
        match expr {
            AstExpr::Variable {name} => {
                assert_eq!(name.lexeme, "x");
            }
            _ => panic!("Expected variable"),
        }
    }

    #[test]
    fn test_primary_boolean(){
        let mut parser = create_parser("true".to_string());
        let expr = parser.primary().unwrap();
        assert!(matches!(expr,
        AstExpr::Literal {
                value: Literal::Boolean(true)
            }));
    }

    #[test]
    fn test_primary_invalid() {
        let mut parser = create_parser("*".to_string());

        assert!(parser.primary().is_err());
    }

    #[test]
    fn test_function_call() {
        let mut parser = create_parser("sin(2)".to_string());

        let expr = parser.call().unwrap();

        match expr {
            AstExpr::Call {
                callee,
                arguments,
            } => {
                assert_eq!(arguments.len(), 1);

                assert!(matches!(
                callee.as_ref(),
                AstExpr::Function { name }
                    if name.token_type == TokenType::Sin
            ));

                assert!(matches!(
                &arguments[0],
                AstExpr::Literal {
                    value: Literal::Number(2.0)
                }
            ));
            }
            _ => panic!("Expected function call"),
        }
    }

    #[test]
    fn test_complex_function_call() {
        let mut parser = create_parser("complex(3,4)".to_string());
        let expr = parser.call().unwrap();
        match expr {
            AstExpr::Call {
                callee,
                arguments,
            } => {
                assert!(matches!(
                callee.as_ref(),
                AstExpr::Function { name }
                    if name.token_type == TokenType::Complex
            ));

                assert_eq!(arguments.len(), 2);
            }
            _ => panic!("Expected complex function call"),
        }
    }

    #[test]
    fn test_operator_precedence() {
        let mut parser = create_parser("2 + 3 * 4".to_string());

        let expr = parser.expression().unwrap();

        match expr {
            AstExpr::Binary {
                operator,
                right,
                ..
            } => {
                assert_eq!(operator.token_type, TokenType::Plus);

                assert!(matches!(
                right.as_ref(),
                AstExpr::Binary {
                    operator,
                    ..
                } if operator.token_type == TokenType::Star
            ));
            }
            _ => panic!("Expected binary expression"),
        }
    }

    #[test]
    fn test_assignment() {
        let mut parser = create_parser("x = 10".to_string());

        let expr = parser.assignment().unwrap();

        match expr {
            AstExpr::Assign { name, value } => {
                assert_eq!(name.lexeme, "x");

                assert!(matches!(
                value.as_ref(),
                AstExpr::Literal {
                    value: Literal::Number(10.0)
                }
            ));
            }
            _ => panic!("Expected assignment"),
        }
    }

    #[test]
    fn test_variable_declaration() {
        let mut parser = create_parser("var x = 10;".to_string());

        let statements = parser.parse().unwrap();

        assert_eq!(statements.len(), 1);

        match &statements[0] {
            AstStmt::Var { name, initializer } => {
                assert_eq!(name.lexeme, "x");
                assert!(initializer.is_some());
            }
            _ => panic!("Expected variable declaration"),
        }
    }

    #[test]
    fn test_power(){
        let mut parser = create_parser("x^2".to_string());
        let expr = parser.power().unwrap();
        match expr {
            AstExpr::Binary {
                left,
                operator,
                right,
            } => {
                assert_eq!(operator.token_type, TokenType::Caret);
                assert!(matches!(
                    left.as_ref(),
                    AstExpr::Variable {name} if name.lexeme == "x"
                ));
                assert!(matches!(
                    right.as_ref(),
                    AstExpr::Literal {
                        value: Literal::Number(2.0)
                    }
                ));
            }
            _ => panic!("Expected exponentiation"),
        }
    }

    #[test]
    fn test_invalid_syntax() {
        let mut parser = create_parser("2 + * 3".to_string());
        assert!(parser.parse().is_err());
    }

    #[test]
    fn test_invalid_syntax_missing_parenthesis() {
        let mut parser = create_parser("sin(2".to_string());
        assert!(parser.parse().is_err());
    }

    #[test]
    fn test_invalid_syntax_invalid_assignment() {
        let mut parser = create_parser("2 + 3 = 5".to_string());
        assert!(parser.parse().is_err());
    }

    #[test]
    fn test_invalid_syntax_missing_variable_name() {
        let mut parser = create_parser("var = 10;".to_string());
        assert!(parser.parse().is_err());
    }

    #[test]
    fn test_invalid_syntax_missing_declaration_semicolon() {
        let mut parser = create_parser("var x = 10".to_string());
        assert!(parser.parse().is_err());
    }

    #[test]
    fn test_invalid_syntax_missing_second_argument() {
        let mut parser = create_parser("complex(2,)".to_string());
        assert!(parser.parse().is_err());
    }

    #[test]
    fn test_vector_function_call() {
        let mut parser = create_parser("vector(1,2,3)".to_string());
        let expr = parser.call().unwrap();

        match expr {
            AstExpr::Call { callee, arguments } => {
                assert_eq!(arguments.len(), 3);

                assert!(matches!(
                callee.as_ref(),
                AstExpr::Function { name }
                    if name.token_type == TokenType::Vector
            ));

                for (argument, expected) in
                    arguments.iter().zip([1.0, 2.0, 3.0])
                {
                    assert!(matches!(
                    argument,
                    AstExpr::Literal {
                        value: Literal::Number(n)
                    } if *n == expected
                ));
                }
            }
            _ => panic!("Expected vector function call"),
        }
    }

    #[test]
    fn test_vector_addition() {
        let mut parser = create_parser(
            "vector(1,2,3) + vector(0,0,1)".to_string()
        );
        let expr = parser.expression().unwrap();
        match expr {
            AstExpr::Binary {
                left,
                operator,
                right,
            } => {
                assert_eq!(operator.token_type, TokenType::Plus);
                assert!(matches!(
                left.as_ref(),
                AstExpr::Call { arguments, .. }
                    if arguments.len() == 3
            ));

                assert!(matches!(
                right.as_ref(),
                AstExpr::Call { arguments, .. }
                    if arguments.len() == 3
            ));
            }
            _ => panic!("Expected vector addition expression"),
        }
    }

    #[test]
    fn test_vector_subtraction() {
        let mut parser = create_parser(
            "vector(1,2,3) - vector(0,0,1)".to_string()
        );

        let expr = parser.expression().unwrap();

        match expr {
            AstExpr::Binary {
                left,
                operator,
                right,
            } => {
                assert_eq!(operator.token_type, TokenType::Minus);

                assert!(matches!(
                left.as_ref(),
                AstExpr::Call { .. }
            ));

                assert!(matches!(
                right.as_ref(),
                AstExpr::Call { .. }
            ));
            }
            _ => panic!("Expected vector subtraction expression"),
        }
    }

    #[test]
    fn test_vector_scalar_multiplication() {
        let mut parser = create_parser(
            "vector(1,2,3) * 2".to_string()
        );

        let expr = parser.expression().unwrap();

        match expr {
            AstExpr::Binary {
                left,
                operator,
                right,
            } => {
                assert_eq!(operator.token_type, TokenType::Star);

                assert!(matches!(
                left.as_ref(),
                AstExpr::Call { .. }
            ));

                assert!(matches!(
                right.as_ref(),
                AstExpr::Literal {
                    value: Literal::Number(2.0)
                }
            ));
            }
            _ => panic!("Expected scalar multiplication"),
        }
    }

    #[test]
    fn test_scalar_vector_multiplication() {
        let mut parser = create_parser(
            "2 * vector(1,2,3)".to_string()
        );

        let expr = parser.expression().unwrap();

        match expr {
            AstExpr::Binary {
                left,
                operator,
                right,
            } => {
                assert_eq!(operator.token_type, TokenType::Star);

                assert!(matches!(
                left.as_ref(),
                AstExpr::Literal {
                    value: Literal::Number(2.0)
                }
            ));

                assert!(matches!(
                right.as_ref(),
                AstExpr::Call { .. }
            ));
            }
            _ => panic!("Expected scalar-vector multiplication"),
        }
    }

    #[test]
    fn test_nested_norm_vector_call() {
        let mut parser = create_parser(
            "norm(vector(1,2,3))".to_string()
        );

        let expr = parser.expression().unwrap();

        match expr {
            AstExpr::Call { callee, arguments } => {
                assert!(matches!(
                callee.as_ref(),
                AstExpr::Function { name }
                    if name.token_type == TokenType::Norm
            ));

                assert_eq!(arguments.len(), 1);

                assert!(matches!(
                &arguments[0],
                AstExpr::Call {
                    callee,
                    arguments
                } if matches!(
                    callee.as_ref(),
                    AstExpr::Function { name }
                        if name.token_type == TokenType::Vector
                ) && arguments.len() == 3
            ));
            }
            _ => panic!("Expected nested norm function call"),
        }
    }

    #[test]
    fn test_nested_conjugate_call() {
        let mut parser = create_parser(
            "conjugate(complex(2,3))".to_string()
        );

        let expr = parser.expression().unwrap();

        match expr {
            AstExpr::Call { callee, arguments } => {
                assert!(matches!(
                callee.as_ref(),
                AstExpr::Function { name }
                    if name.token_type == TokenType::Conjugate
            ));

                assert_eq!(arguments.len(), 1);

                assert!(matches!(
                &arguments[0],
                AstExpr::Call { arguments, .. }
                    if arguments.len() == 2
            ));
            }
            _ => panic!("Expected conjugate function call"),
        }
    }

    #[test]
    fn test_dot_product_call() {
        let mut parser = create_parser(
            "dot(vector(1,2,3),vector(4,5,6))".to_string()
        );

        let expr = parser.expression().unwrap();
        match expr {
            AstExpr::Call { callee, arguments } => {
                assert!(matches!(
                callee.as_ref(),
                AstExpr::Function { name }
                    if name.token_type == TokenType::DotProduct
            ));

                assert_eq!(arguments.len(), 2);

                for argument in &arguments {
                    assert!(matches!(
                    argument,
                    AstExpr::Call {
                        callee,
                        arguments
                    } if matches!(
                        callee.as_ref(),
                        AstExpr::Function { name }
                            if name.token_type == TokenType::Vector
                    ) && arguments.len() == 3
                ));
                }
            }
            _ => panic!("Expected dot product function call"),
        }
    }

    #[test]
    fn test_invalid_vector_missing_argument() {
        let mut parser = create_parser(
            "vector(1,2,)".to_string()
        );

        assert!(parser.parse().is_err());
    }

    #[test]
    fn test_invalid_vector_missing_parenthesis() {
        let mut parser = create_parser(
            "vector(1,2,3".to_string()
        );

        assert!(parser.parse().is_err());
    }

    #[test]
    fn test_invalid_nested_function() {
        let mut parser = create_parser(
            "norm(vector(1,2,3)".to_string()
        );

        assert!(parser.parse().is_err());
    }

    #[test]
    fn test_matrix_function_call() {
        let mut parser = create_parser(
            "matrix([1,2,3],[4,5,6],[7,8,9])".to_string()
        );
        let expr = parser.expression().unwrap();
        match expr {
            AstExpr::Call { callee, arguments } => {
                assert!(matches!(
                callee.as_ref(),
                AstExpr::Function { name }
                    if name.token_type == TokenType::Matrix
            ));
                assert_eq!(arguments.len(), 3);
                for row in &arguments {
                    assert!(matches!(
                    row,
                    AstExpr::List { elements }
                        if elements.len() == 3
                ));
                }
            }
            _ => panic!("Expected matrix function call"),
        }
    }




}