#[cfg(test)]
mod tests {
    use math_engine::ast_stmt::AstStmt;
    use math_engine::interpreter::Interpreter;
    use super::*;
    use math_engine::parser::Parser;
    use math_engine::runtime_error::RuntimeError;
    use math_engine::scanner::Scanner;
    use math_engine::value::Value;
    use math_engine::vector::Vector;


    fn evaluate_source(
        source: &str,
    ) -> Result<Value, RuntimeError> {
        let mut scanner = Scanner::new(source.to_string());
        let tokens = scanner.scan_tokens();

        let mut parser = Parser::new(tokens);
        let statements = parser.parse().unwrap();

        let mut interpreter = Interpreter::new();

        match &statements[0] {
            AstStmt::Expression { expression } => {
                interpreter.evaluate(expression)
            }
            _ => panic!("Expected expression statement"),
        }
    }

    #[test]
    fn test_exponentation(){
        let result = evaluate_source("2^3;").unwrap();
        assert_eq!(result, Value::Number(8.0));
    }

    #[test]
    fn test_operator_precedence() {
        let result = evaluate_source("2 + 3 * 4;").unwrap();

        assert_eq!(result, Value::Number(14.0));
    }

    #[test]
    fn test_grouping(){
        let result = evaluate_source("(2 + 3) * 4;").unwrap();
        assert_eq!(result, Value::Number(20.0));
    }



    #[test]
    fn test_sin(){
        let result = evaluate_source("sin(pi/2);").unwrap();
        assert_number_close(result, 1.0);
    }

    #[test]
    fn test_cos(){
        let result = evaluate_source("cos(0);").unwrap();
        assert_number_close(result, 1.0);
    }

    #[test]
    fn test_exp(){
        let result = evaluate_source("exp(1);").unwrap();
        assert_number_close(result, std::f64::consts::E);
    }

    #[test]
    fn test_ln() {
        let result = evaluate_source("ln(e);").unwrap();
        assert_number_close(result, 1.0);
    }

    #[test]
    fn test_log(){
        let result = evaluate_source("log(100);").unwrap();
        assert_number_close(result, 2.0);
    }

    #[test]
    fn test_sqrt(){
        let result = evaluate_source("sqrt(81);").unwrap();
        assert_number_close(result, 9.0);
    }

    #[test]
    fn test_complex_creation(){
        let result = evaluate_source("complex(2,3);").unwrap();
        assert_complex_close(result, 2.0, 3.0);
    }

    #[test]
    fn test_complex_addition(){
        let result = evaluate_source("complex(2,3) + complex(4,5);").unwrap();
        assert_complex_close(result, 6.0, 8.0);
    }

    #[test]
    fn test_complex_multiplication(){
        let result = evaluate_source("complex(2,3) * complex(4,5);").unwrap();
        assert_complex_close(result, -7.0, 22.0);
    }

    #[test]
    fn test_complex_conjugate(){
        let result = evaluate_source("conjugate(complex(2,3));").unwrap();
        assert_complex_close(result, 2.0, -3.0);
    }

    #[test]
    fn test_conjugate_complex_expression() {
        let result = evaluate_source("conjugate(2+2i);").unwrap();

        assert_complex_close(result, 2.0, -2.0);
    }

    #[test]
    fn test_double_conjugate() {
        let result = evaluate_source(
            "conjugate(conjugate(complex(2,3)));"
        ).unwrap();

        assert_complex_close(result, 2.0, 3.0);
    }

    #[test]
    fn test_limit_direct_substitution() {
        let result = evaluate_source(
            "limit(x^2,x,3);"
        ).unwrap();

        assert_number_close(result, 9.0);
    }

    #[test]
    fn test_limit_polynomial() {
        let result = evaluate_source(
            "limit(x^2 + 2*x + 1,x,2);"
        ).unwrap();

        assert_number_close(result, 9.0);
    }

    #[test]
    fn test_invalid_complex_argument_count() {
        let result = evaluate_source("complex(2);");

        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_limit_argument_count() {
        let result = evaluate_source("limit(x^2,x);");

        assert!(result.is_err());
    }

    #[test]
    fn test_sin_without_arguments_returns_error() {
        let result = evaluate_source("sin();");

        assert!(result.is_err());
    }

    #[test]
    fn test_vector_creation(){
        let result = evaluate_source("vector(1,2,3);").unwrap();
        match result {
            Value::Vector(v) => {
                assert_eq!(v.dimension(), 3);
                assert_eq!(v.components(), &[1.0, 2.0, 3.0]);
            }
            _ => panic!("Expected vector")
        }
    }


    #[test]
    fn test_vector_norm() {
        let result = evaluate_source("norm(vector(1,2,3));").unwrap();

        match result {
            Value::Number(n) => {
                let expected = 14.0_f64.sqrt();

                assert!(
                    (n - expected).abs() < 1e-10,
                    "Expected {}, got {}",
                    expected,
                    n
                );
            }
            _ => panic!("Expected a numerical norm"),
        }
    }

    #[test]
    fn test_vector_addition(){
        let result = evaluate_source("vector(1,2,3) + vector(0,0,1);").unwrap();
        match result{
            Value::Vector(v) => {
                assert_eq!(v.dimension(), 3);
                let expected = [1.0, 2.0, 4.0];
                for(actual, expected) in v.components()
                    .iter()
                    .zip(expected.iter())
                {
                    assert!((actual - expected).abs() < 1e-10,
                    "Expected {}, got {}",
                    expected,
                    actual);
                };
            }
            _ => panic!("Expected a vector")
        }
    }

    #[test]
    fn test_vector_subtraction() {
        let result = evaluate_source(
            "vector(1,2,3) - vector(0,0,1);"
        ).unwrap();

        match result {
            Value::Vector(v) => {
                assert_eq!(v.dimension(), 3);

                let expected = [1.0, 2.0, 2.0];

                for (actual, expected) in v.components()
                    .iter()
                    .zip(expected.iter())
                {
                    assert!(
                        (actual - expected).abs() < 1e-10,
                        "Expected {}, got {}",
                        expected,
                        actual
                    );
                }
            }

            _ => panic!("Expected Value::Vector"),
        }
    }

    #[test]
    fn test_vector_scalar_multiplication() {
        let result = evaluate_source(
            "vector(1,2,3) * 2;"
        ).unwrap();

        match result {
            Value::Vector(v) => {
                assert_eq!(v.dimension(), 3);

                let expected = [2.0, 4.0, 6.0];

                for (actual, expected) in v.components()
                    .iter()
                    .zip(expected.iter())
                {
                    assert!(
                        (actual - expected).abs() < 1e-10,
                        "Expected {}, got {}",
                        expected,
                        actual
                    );
                }
            }
            _ => panic!("Expected Value::Vector"),
        }
    }

    

    fn assert_number_close(actual: Value, expected: f64){
        match actual {
            Value::Number(value) => {
                assert!(
                    (value - expected).abs() < 1e-10,
                    "Expected {}, get {}",
                    expected,
                    value
                );
            }
            _ => panic!("Expected number"),
        }
    }


    #[test]
    fn test_scalar_vector_multiplication() {
        let result = evaluate_source(
            "2 * vector(1,2,3);"
        ).unwrap();

        match result {
            Value::Vector(v) => {
                assert_eq!(v.components(), &[2.0, 4.0, 6.0]);
            }
            _ => panic!("Expected Value::Vector"),
        }
    }

    #[test]
    fn test_vector_dot_product() {
        let result = evaluate_source(
            "dot(vector(1,2,3), vector(4,5,6));"
        ).unwrap();

        match result {
            Value::Number(n) => {
                assert!((n - 32.0).abs() < 1e-10);
            }
            _ => panic!("Expected Value::Number"),
        }
    }

    #[test]
    fn test_matrix_creation_3x3() {
        let result = evaluate_source(
            "matrix([1,2,3],[4,5,6],[7,8,9]);"
        ).unwrap();

        match result {
            Value::Matrix(m) => {
                assert_eq!(m.rows(), 3);
                assert_eq!(m.cols(), 3);

                assert_eq!(
                    m.components(),
                    &[
                        vec![1.0, 2.0, 3.0],
                        vec![4.0, 5.0, 6.0],
                        vec![7.0, 8.0, 9.0],
                    ]
                );
            }
            _ => panic!("Expected matrix"),
        }
    }

    #[test]
    fn test_matrix_creation_rectangular() {
        let result = evaluate_source(
            "matrix([1,2,3],[4,5,6]);"
        ).unwrap();

        match result {
            Value::Matrix(m) => {
                assert_eq!(m.rows(), 2);
                assert_eq!(m.cols(), 3);
            }
            _ => panic!("Expected matrix"),
        }
    }

    #[test]
    fn test_matrix_single_row() {
        let result = evaluate_source(
            "matrix([1,2,3]);"
        ).unwrap();

        match result {
            Value::Matrix(m) => {
                assert_eq!(m.rows(), 1);
                assert_eq!(m.cols(), 3);
            }
            _ => panic!("Expected matrix"),
        }
    }

    #[test]
    fn test_matrix_single_column() {
        let result = evaluate_source(
            "matrix([1],[2],[3]);"
        ).unwrap();

        match result {
            Value::Matrix(m) => {
                assert_eq!(m.rows(), 3);
                assert_eq!(m.cols(), 1);
            }
            _ => panic!("Expected matrix"),
        }
    }

    #[test]
    fn test_matrix_expression_elements() {
        let result = evaluate_source(
            "matrix([1+2,3*4],[2^3,sqrt(81)]);"
        ).unwrap();

        match result {
            Value::Matrix(m) => {
                assert_eq!(
                    m.components(),
                    &[
                        vec![3.0, 12.0],
                        vec![8.0, 9.0],
                    ]
                );
            }
            _ => panic!("Expected matrix"),
        }
    }

    #[test]
    fn test_matrix_unequal_row_lengths() {
        let result = evaluate_source(
            "matrix([1,2,3],[4,5]);"
        );

        assert!(result.is_err());
    }

    #[test]
    fn test_dimension_vector_2d() {
        let result = evaluate_source("dim(vector(1,2));").unwrap();

        assert_number_close(result, 2.0);
    }

    #[test]
    fn test_dimension_vector_3d() {
        let result = evaluate_source("dim(vector(1,2,3));").unwrap();

        assert_number_close(result, 3.0);
    }
    #[test]
    fn test_dimension_vector_10d() {
        let result = evaluate_source(
            "dim(vector(1,2,3,4,5,6,7,8,9,10));"
        ).unwrap();

        assert_number_close(result, 10.0);
    }

    #[test]
    fn test_dimension_zero_vector() {
        let result = evaluate_source(
            "dim(vector(0,0,0));"
        ).unwrap();

        assert_number_close(result, 3.0);
    }

    #[test]
    fn test_dimension_negative_components() {
        let result = evaluate_source(
            "dim(vector(-1,-2,-3,-4));"
        ).unwrap();

        assert_number_close(result, 4.0);
    }

    #[test]
    fn test_dimension_vector_addition() {
        let result = evaluate_source(
            "dim(vector(1,2,3) + vector(4,5,6));"
        ).unwrap();

        assert_number_close(result, 3.0);
    }

    #[test]
    fn test_dimension_vector_subtraction() {
        let result = evaluate_source(
            "dim(vector(1,2,3) - vector(4,5,6));"
        ).unwrap();

        assert_number_close(result, 3.0);
    }

    #[test]
    fn test_dimension_vector_scalar_multiplication() {
        let result = evaluate_source(
            "dim(vector(1,2,3) * 5);"
        ).unwrap();

        assert_number_close(result, 3.0);
    }

    #[test]
    fn test_dimension_vector_zero_scalar() {
        let result = evaluate_source(
            "dim(vector(1,2,3) * 0);"
        ).unwrap();

        assert_number_close(result, 3.0);
    }

    #[test]
    fn test_dimension_arithmetic() {
        let result = evaluate_source(
            "dim(vector(1,2,3)) + 5;"
        ).unwrap();

        assert_number_close(result, 8.0);
    }

    #[test]
    fn test_dimension_vector_expressions() {
        let result = evaluate_source(
            "dim(vector(1+2, 3*4, sin(pi), 2^3));"
        ).unwrap();

        assert_number_close(result, 4.0);
    }

    fn assert_complex_close(actual: Value, expected_real: f64, expected_imaginary: f64){
        match actual {
            Value::Complex(c) => {
                assert!((c.real - expected_real).abs() < 1e-10);
                assert!((c.imaginary() - expected_imaginary).abs() < 1e-10);
            }
            _ => panic!("Expected complex"),
        }
    }



}