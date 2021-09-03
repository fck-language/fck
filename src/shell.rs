extern crate lang;
extern crate ErrWrn;

use crate::ast;

use lang::keywords::Keywords;
use lang::{get_associated_keywords, get_associated_messages};
use crate::config_file::ConfigFile;

pub fn shell(config_file: ConfigFile) {
    let mut current_language = get_associated_keywords(&*config_file.default_lang).unwrap();
    let mut keyword_code = config_file.default_lang;
    loop {
        let mut given = String::new();
        match std::io::stdin().read_line(&mut given) {
            Ok(_) => {}
            Err(e) => { panic!("Could not read input!\n{}", e); }
        }

        let mut lexer = ast::Lexer::new(given, current_language.clone(), keyword_code.clone());
        let tokens = match lexer.make_tokens() {
            Ok(t) => t,
            Err(_) => { panic!() }
        };
        current_language = lexer.keywords.clone();
        keyword_code = match keyword_code == lexer.keyword_code {
            true => keyword_code,
            false => {
                if config_file.shell_language_info {
                    match get_associated_messages(&*lexer.keyword_code.clone()) {
                        Some(m) => println!("{}", m.generic[0]),
                        None => {}
                    };
                }
                lexer.keyword_code.clone()
            }
        };

        if tokens.is_empty() {
            println!("Empty token vector");
            continue;
        }

        println!("{}~~~  Tokens End  ~~~\n", tokens.iter().fold(String::from("~~~ Tokens Start ~~~\n"), |acc, arg| acc + &*format!("{}", arg) + "\n"));


        let mut parser = ast::Parser::new(tokens);
        let ast_list = match parser.parse() {
            Ok(t) => t,
            Err(_) => {
                println!("Error in parser");
                continue;
            }
        };
        for (i, ast) in ast_list.iter().enumerate() {
            println!("** {} **\n{:?}", i + 1, ast)
        }
    }
}
