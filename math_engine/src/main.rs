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
use std::env;
use std::process;

fn main() {
    // let source = String::from("x <= 2 + y");
    // let mut scanner = Scanner::new(source);
    // let tokens = scanner.scan_tokens();
    // for token in tokens {
    //     println!("{:#?}", token);
    // }
    let mut args = env::args();
    let _program = args.next();
    let output_dir = match args.next() {
        Some(dir) if args.next().is_none() => dir,
        _ => {
            eprintln!("Usage: generate_ast <output directory>");
            process::exit(64);
        }
    };

    println!("Output directory: {}", output_dir);
}
