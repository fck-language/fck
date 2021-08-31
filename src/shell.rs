extern crate lang;
extern crate ErrWrn;

use crate::ast;

use lang::keywords::Keywords;

pub fn shell(default_language: Keywords<'static>) {
    let mut current_language = default_language;
    loop {
        let mut given = String::new();
        match std::io::stdin().read_line(&mut given) {
            Ok(_) => {}
            Err(e) => { panic!(format!("Could not read input!\n{}", e)); }
        }

        let mut lexer = ast::Lexer::new(given, current_language.clone());
        let tokens = match lexer.make_tokens() {
            Ok(t) => t,
            Err(e) => {panic!()}
        };
        current_language = lexer.keywords.clone();

        if tokens.is_empty() {
            println!("Empty token vector");
            continue;
        }
        let mut parser = ast::Parser::new(tokens);
        let ast_list = match parser.parse() {
            Ok(t) => t,
            Err(e) => {continue}
        };
        for (i, ast) in ast_list.iter().enumerate() {
            println!("** {} **\n{:?}", i + 1, ast)
        }
    }
}
