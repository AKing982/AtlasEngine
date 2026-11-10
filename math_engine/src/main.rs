mod vector;
mod function;
mod fraction;
mod limit;
mod derivative;
mod matrix;
mod summation;
mod DefiniteIntegral;
mod complex;
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

struct RunOptions {
    show_tokens: bool,
    show_ast: bool,
}

fn main() {
    let mut options = RunOptions {
        show_tokens: true,
        show_ast: false,
    };

    let mut script: Option<String> = None;

    for arg in env::args().skip(1) {
        match arg.as_str() {
            "--tokens" => options.show_tokens = true,
            "--ast" => options.show_ast = true,
            "--all" => {
                options.show_tokens = true;
                options.show_ast = true;
            }
            _ if arg.starts_with("--") => {
                eprintln!("Unknown option: {}", arg);
                process::exit(64);
            }
            _ => {
                if script.is_some() {
                    eprintln!("Usage: math_engine [--tokens] [--ast] [script]");
                    process::exit(64);
                }
                script = Some(arg);
            }
        }
    }

    if let Some(path) = script {
        run_file(&path, &options);
    } else {
        run_prompt(&options);
    }
}

fn run_file(path: &str, options: &RunOptions) {
    let source = std::fs::read_to_string(path)
        .expect("Could not read source file");

    let mut interpreter = Interpreter::new();
    run_with_interpreter(source, &mut interpreter, options);
}

fn run_prompt(options: &RunOptions) {
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

        run_with_interpreter(line, &mut interpreter, options);
    }
}

fn run_with_interpreter(
    source: String,
    interpreter: &mut Interpreter,
    options: &RunOptions,
) {
    // 1. Scanner
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens();

    if options.show_tokens {
        println!("\n=== TOKENS ===");
        for token in &tokens {
            println!("{:?}", token);
        }
    }

    // 2. Parser
    let mut parser = Parser::new(tokens.clone());

    match parser.parse() {
        Ok(statements) => {

            if options.show_ast {
                println!("\n=== SYNTAX TREE ===");

                let printer = AstPrinter::new();

                for statement in &statements {
                    match statement {
                        AstStmt::Expression { expression } => {
                            println!("{}", printer.print(expression));
                        }

                        AstStmt::Var { name, initializer } => {
                            let value = match initializer {
                                Some(expression) => printer.print(expression),
                                None => "nil".to_string(),
                            };

                            println!("(var {} {})", name.lexeme, value);
                        }
                    }
                }
            }

            // 3. Interpreter
            println!("\n=== RESULTS ===");

            for statement in &statements {
                interpreter.interpret(statement);
            }
        }

        Err(error) => {
            eprintln!("Parsing failed: {:?}", error);
        }
    }
}

