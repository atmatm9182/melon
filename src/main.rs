use std::process::ExitCode;

use lexer::Lexer;

mod lexer;

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
    for token in lex {
        println!("{token:?}");
    }

    ExitCode::SUCCESS
}
