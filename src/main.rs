use std::env;
use std::fs;

mod lex;
mod parse;
mod token;

use lex::Lexer;
use parse::Parser;
fn main() {
    let args: Vec<_> = env::args().skip(1).collect();
    let code = match fs::read_to_string(&args[0]) {
        Ok(code) => code,
        Err(err) => {
            eprintln!("{:?}", err);
            std::process::exit(1);
        }
    };
    let mut lexer = Lexer::new(&code);
    let tokens = lexer.lex();
    println!("{:#?}", tokens);
}
