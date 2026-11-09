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

#[path="lexer/parse_error.rs"]
mod parse_error;

#[path = "lexer/ast_expr.rs"]
mod ast_expr;

mod infinite_summation;
mod MetricTensor;

#[path="lexer/ast_printer.rs"]
mod ast_printer;

#[path="lexer/parser.rs"]
mod parser;

#[path = "lexer/scanner.rs"]
mod scanner;

#[path = "lexer/token.rs"]
mod token;

#[path = "lexer/token_type.rs"]
mod token_type;

#[path = "lexer/literal.rs"]
mod literal;

#[path="lexer/ast_stmt.rs"]
mod ast_stmt;

#[path="lexer/interpreter.rs"]
mod interpreter;

#[path="lexer/value.rs"]
mod value;

#[path="lexer/runtime_error.rs"]
mod runtime_error;

#[path="lexer/environment.rs"]
mod environment;

use std::env;
use std::process;
use std::fs::File;
use std::io::Write;
use std::path::Path;

use crate::ast_printer::AstPrinter;
use crate::ast_stmt::AstStmt;
use crate::interpreter::Interpreter;
use crate::parser::Parser;
use crate::scanner::Scanner;


fn define_ast(
    output_dir: &str,
    base_name: &str,
    types: &[&str],
) -> std::io::Result<()> {

    let path = Path::new(output_dir).join(format!("{}.rs", base_name));
    let mut writer = File::create(path)?;

    writeln!(writer, "pub enum {} {{", base_name)?;

    // The AST variants.
    for type_definition in types {
        let (class_name, fields) = type_definition
            .split_once(':')
            .expect("Invalid AST type definition");

        let class_name = class_name.trim();
        let fields = fields.trim();

        define_type(&mut writer, class_name, fields)?;
    }

    writeln!(writer, "}}")?;

    Ok(())
}

fn define_type(
    writer: &mut File,
    class_name: &str,
    field_list: &str,
) -> std::io::Result<()> {

    writeln!(writer, "    {} {{", class_name)?;

    // Fields.
    let fields: Vec<&str> = field_list.split(", ").collect();

    for field in &fields {
        let parts: Vec<&str> = field.split_whitespace().collect();

        let field_type = parts[0];
        let field_name = parts[1];

        writeln!(writer, "        {}: {},", field_name, field_type)?;
    }

    writeln!(writer, "    }},")?;

    Ok(())
}
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        eprintln!("Usage: math_engine [script]");
        process::exit(64);
    }

    if args.len() == 2 {
        run_file(&args[1]);
    } else {
        run_prompt();
    }
}

fn run(source: String) {
    let mut interpreter = Interpreter::new();
    run_with_interpreter(source, &mut interpreter);
}

fn run_file(path: &str) {
    let source = std::fs::read_to_string(path)
        .expect("Could not read source file");

    run(source);
}

fn run_prompt() {
    use std::io::{self, Write};

    let mut interpreter = Interpreter::new();

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut line = String::new();

        if io::stdin().read_line(&mut line).unwrap() == 0 {
            break;
        }

        if line.trim() == "exit" {
            break;
        }

        run_with_interpreter(line, &mut interpreter);
    }
}

fn run_with_interpreter(source: String, interpreter: &mut Interpreter) {
    // 1. Lexical analysis
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens();

    // 2. Parsing
    let mut parser = Parser::new(tokens);

    match parser.parse() {
        Ok(statements) => {
            // 3. Interpretation
            for statement in &statements {
                interpreter.interpret(statement);
            }
        }

        Err(error) => {
            eprintln!("Parsing failed: {:?}", error);
        }
    }
}

