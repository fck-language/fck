extern crate lang;
extern crate ErrWrn;
use ErrWrn::*;

use crate::bases::*;
use crate::ErrWrn::*;
use crate::ast::Parser;
use std::path::Path;
use std::fs::read_to_string;

mod tokens;
mod bases;
mod ast;
mod nodes;

fn main() {
    let file = read_to_string("./timed tests/fibonacci/fibonacci.fck");
    let mut given = String::new();
    match std::io::stdin().read_line(&mut given) {
        Ok(_) => {}
        Err(_) => {panic!("Could not read input!");}
    }
    let mut lexer: ast::Lexer = ast::Lexer::new(given, lang::en::KEYWORDS_EN);
    let tokens = lexer.make_tokens();

    let mut parser = Parser::new(tokens);
    let ast_list = parser.parse();
    for (i, ast) in ast_list.ast.iter().enumerate() {
        println!("** {} **\n{:?}", i + 1, ast)
    }
    // println!("{}", tokens.iter().fold(String::new(), |acc, arg| acc + &arg.to_string() + "\n"));
}
