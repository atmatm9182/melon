use std::process::ExitCode;

use lexer::Lexer;
use parser::Parser;

mod lexer;
mod ast;
mod parser;

fn main() -> ExitCode {
    let mut args = std::env::args();
    args.next();
    let code = args.next();
    if code.is_none() {
        eprintln!("ERROR: not enough arguments");
        return ExitCode::FAILURE;
    }

    let code = code.unwrap();

    let lex = Lexer::from(code.as_ref());
    let parser = Parser::new(lex);
    let program = parser.parse_program().unwrap();

    println!("{program:?}");

    ExitCode::SUCCESS
}
