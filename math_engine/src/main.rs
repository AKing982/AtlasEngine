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


use std::env;
use std::process;
use std::fs::File;
use std::io::Write;
use std::path::Path;

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
