mod vector;
mod function;
mod fraction;
mod limit;
mod derivative;
mod matrix;
mod summation;
mod DefiniteIntegral;
mod Complex;
mod Antiderivative;
mod expr;
mod infinite_summation;
mod MetricTensor;
#[path = "lexer/scanner.rs"]
mod scanner;

#[path = "lexer/token.rs"]
mod token;

#[path = "lexer/token_type.rs"]
mod token_type;

#[path = "lexer/literal.rs"]
mod literal;

use crate::scanner::Scanner;

fn main() {
    let source = String::from(".456");
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens();
    for token in tokens {
        println!("{:#?}", token);
    }
}
