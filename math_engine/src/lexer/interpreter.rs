use crate::ast_expr::AstExpr;
use crate::ast_stmt::AstStmt;
use crate::complex;
use crate::literal::Literal;
use crate::runtime_error::RuntimeError;
use crate::token::Token;
use crate::token_type::TokenType;
use crate::value::Value;
use crate::environment::Environment;
use crate::complex::Complex;

pub struct Interpreter{
    variable_map: Environment
}

impl Interpreter {

    pub fn new() -> Self {
        Self {
            variable_map: Environment::new(),
        }
    }

    fn evaluate(&mut self, expr: &AstExpr) -> Result<Value, RuntimeError>{
        match expr {
            AstExpr::Literal {value} => {
                match value {
                    Literal::Number(n) => Ok(Value::Number(*n)),
                    Literal::String(s) => Ok(Value::String(s.clone())),
                    Literal::Boolean(b) => Ok(Value::Boolean(*b)),
                    Literal::Nil() => Ok(Value::Nil),
                    Literal::Complex(n) => Ok(Value::Complex(n.clone()))
                }
            }
            AstExpr::Grouping {expression} => {
                self.evaluate(expression)
            }
            AstExpr::Unary {operator, right} => {
                let right = self.evaluate(right)?;
                match operator.token_type {
                    TokenType::Minus => {
                        self.check_number_operand(operator, &right)?;
                        if let Value::Number(number) = right {
                            Ok(Value::Number(-number))
                        }else{
                            unreachable!()
                        }
                    }
                    _ => todo!(),
                }
            }
            AstExpr::Binary {left, operator, right} => {
                let left = self.evaluate(left)?;
                let right = self.evaluate(right)?;
                match operator.token_type {
                    TokenType::Minus => {
                        self.check_number_operands(operator, &left, &right);
                        if let (Value::Number(left_num), Value::Number(right_num)) = (&left, &right) {
                            Ok(Value::Number(*left_num - *right_num))
                        } else if let (Value::Complex(left_complex), Value::Complex(right_complex)) = (&left, &right) {
                            Ok(Value::Complex(left_complex.subtract(right_complex)))
                        }else{
                            Err(RuntimeError::new(
                                operator,
                                "Operands must be two numbers or two complex numbers",
                            ))
                        }
                    }
                    TokenType::Slash => {
                        self.check_number_operands(operator, &left, &right)?;;
                        if let (Value::Number(left_num), Value::Number(right_num)) = (&left, &right){
                            Ok(Value::Number(*left_num / *right_num))
                        }else{
                            unreachable!()
                        }
                    },
                    TokenType::Star => {
                        if let (Value::Number(left_num), Value::Number(right_num)) = (&left, &right) {
                            Ok(Value::Number(*left_num * *right_num))
                        }else if let (Value::Complex(left_complex), Value::Complex(right_complex)) = (&left, &right){
                            Ok(Value::Complex(left_complex.multiply(right_complex)))
                        }else{
                            Err(RuntimeError::new(
                                operator,
                                "Operands must be two numbers or two complex numbers",
                            ))
                        }
                    }
                    TokenType::Plus => {
                        match (&left, &right) {
                            (Value::Number(left_num), Value::Number(right_num)) => {
                                Ok(Value::Number(left_num + right_num))
                            }
                            (Value::String(left_str), Value::String(right_str)) => {
                                Ok(Value::String(left_str.clone() + right_str))
                            }
                            (Value::Complex(left_complex), Value::Complex(right_complex)) => {
                                Ok(Value::Complex(Complex::add(left_complex, right_complex)))
                            }
                            _ => {
                                todo!()
                            }
                        }
                    }
                    TokenType::Caret => {
                        self.check_number_operands(operator, &left, &right);
                        if let (Value::Number(left_num), Value::Number(right_num)) = (&left, &right) {
                            Ok(Value::Number(left_num.powf(*right_num)))
                        }
                        else{
                            unreachable!()
                        }
                    }
                    TokenType::Greater => {
                        self.check_number_operands(operator, &left, &right);
                        if let (Value::Number(left_num), Value::Number(right_num)) = (&left, &right) {
                            Ok(Value::Boolean(*left_num > *right_num))
                        } else {
                            unreachable!()
                        }
                    }
                    TokenType::GreaterEqual => {
                        self.check_number_operands(operator, &left, &right);
                        if let (Value::Number(left_num), Value::Number(right_num)) = (&left, &right) {
                            Ok(Value::Boolean(*left_num >= *right_num))
                        } else {
                            unreachable!()
                        }
                    }
                    TokenType::Less => {
                        self.check_number_operands(operator, &left, &right);
                        if let (Value::Number(left_num), Value::Number(right_num)) = (&left, &right) {
                            Ok(Value::Boolean(*left_num < *right_num))
                        } else {
                            unreachable!()
                        }
                    }
                    TokenType::LessEqual => {
                        self.check_number_operands(operator, &left, &right);
                        if let (Value::Number(left_num), Value::Number(right_num)) = (&left, &right) {
                            Ok(Value::Boolean(*left_num <= *right_num))
                        } else {
                            unreachable!()
                        }
                    }
                    TokenType::BangEqual => {
                        Ok(Value::Boolean(!self.is_equal(&left, &right)))
                    }
                    TokenType::EqualEqual => {
                        Ok(Value::Boolean(self.is_equal(&left, &right)))
                    }
                    _ => {
                        Ok(Value::Nil)
                    }
                }
            }
            AstExpr::Variable {name} => {
                let name_str: &str = &name.lexeme;
                let value = self.variable_map
                    .get(name_str)
                    .ok_or(RuntimeError::new(
                        name,
                        "Undefined variable",
                    ))?;
                Ok(value.clone())
            }
            AstExpr::Function {name} => {
                Err(RuntimeError::new(
                    name,
                    "Built in function must be called",
                ))
            }
            AstExpr::Call {callee, arguments} => {
                match callee.as_ref(){
                    AstExpr::Function {name} => {
                        let argument = self.evaluate(&arguments[0])?;
                        let argument = match argument {
                            Value::Number(n) => n,
                            _ => {
                                return Err(RuntimeError::new(
                                    name,
                                    "Argument must be a number",
                                ));
                            }
                        };
                        match name.token_type {
                            TokenType::Exp => {Ok(Value::Number(argument.exp()))}
                            TokenType::Sin => {Ok(Value::Number(argument.sin()))}
                            TokenType::Cos => {Ok(Value::Number(argument.cos()))}
                            TokenType::Tan => {Ok(Value::Number(argument.tan()))}
                            TokenType::Ln => {Ok(Value::Number(argument.ln()))}
                            TokenType::Log => {Ok(Value::Number(argument.log10()))}
                            TokenType::Sqrt => {Ok(Value::Number(argument.sqrt()))}
                            TokenType::Conjugate => {
                                if arguments.len() != 1 {
                                    return Err(RuntimeError::new(
                                        name,
                                        "Expected one argument",
                                    ));
                                }
                                let argument = self.evaluate(&arguments[0])?;
                                match argument {
                                    Value::Complex(c) => {
                                        Ok(Value::Complex(c.conjugate()))
                                    }
                                    _ => {
                                        Err(RuntimeError::new(
                                            name,
                                            "Argument must be a complex number",
                                        ))
                                    }
                                }
                            }
                            TokenType::Complex => {
                                if arguments.len() != 2 {
                                    return Err(RuntimeError::new(
                                        name,
                                        "expected two arguments",
                                    ));
                                }
                                let arg1 = self.evaluate(&arguments[0])?;
                                let arg2 = self.evaluate(&arguments[1])?;
                                let arg1_val = match arg1 {
                                    Value::Number(n) => n,
                                    _ => {
                                        return Err(RuntimeError::new(
                                            name,
                                            "First argument must be a number",
                                        ));
                                    }
                                };
                                let arg2_val = match arg2 {
                                    Value::Number(n) => n,
                                    _ => {
                                        return Err(RuntimeError::new(
                                            name,
                                            "First argument must be a number",
                                        ));
                                    }
                                };
                                Ok(Value::Complex(Complex::new(arg1_val, arg2_val)))
                            }
                            _ => {
                                Err(RuntimeError::new(
                                    name,
                                    "Unknown built-in function",
                                ))
                            }
                        }
                    }
                    _ => {
                        unreachable!()
                    }
                }
            }
            AstExpr::Assign {name, value} => {
                let name_str: &str = name.lexeme.as_str();
                let val = self.evaluate(value)?;
                let assigned = self.variable_map.assign(name_str, val.clone());
                if assigned {
                    Ok(val)
                }else {
                    Err(RuntimeError::new(name,
                    "Unable to assign variable to value"))
                }
            }
            _ => {
                unreachable!()
            }
        }
    }

    pub(crate) fn interpret(&mut self, statement: &AstStmt) {
        match statement {
            AstStmt::Expression { expression } => {
                match self.evaluate(expression) {
                    Ok(value) => println!("{}", self.stringify(&value)),
                    Err(error) => eprintln!("{:?}", error),
                }
            }

            AstStmt::Var { name, initializer } => {
                let value = match initializer {
                    Some(expression) => self.evaluate(expression),
                    None => Ok(Value::Nil),
                };

                match value {
                    Ok(value) => {
                        self.variable_map.define(name.lexeme.to_string(), value);
                    }
                    Err(error) => {
                        eprintln!("{:?}", error);
                    }
                }
            }
        }
    }

    fn stringify(&self, value: &Value) -> String {
        match value {
            Value::Nil => String::from("nil"),
            Value::Boolean(b) => b.to_string(),
            Value::Integer(n) => n.to_string(),
            Value::Number(n) => {
                let mut text = n.to_string();
                if text.ends_with(".0") {
                    text.truncate(text.len() - 2);
                }

                text
            }
            Value::Complex(c) => {
                let real = c.real();
                let imaginary = c.imaginary();
                if imaginary < 0.0 {
                    format!("{} - {}i", real, imaginary.abs())
                }else{
                    format!("{} + {}i", real, imaginary)
                }
            }
            Value::String(s) => s.clone(),
        }
    }

    fn check_number_operand(&self, operator: &Token, operand: &Value) -> Result<(), RuntimeError>{
        if matches!(operand, Value::Number(_)){
            return Ok(())
        }
        Err(RuntimeError::new(
            operator,
            "Operand must be a number.",
        ))
    }

    fn check_number_operands(&self, operator: &Token, operand1: &Value, operand2: &Value) -> Result<(), RuntimeError>{
        if matches!(operand1, Value::Number(_)) && matches!(operand2, Value::Number(_)){
            return Ok(())
        }
        Err(RuntimeError::new(
            operator,
            "Operands must be numbers.",
        ))
    }

    fn is_equal(&self, a: &Value, b: &Value) -> bool {
        a == b
    }
}