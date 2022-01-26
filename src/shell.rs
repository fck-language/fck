//! Shell file using JIT compilation
//!
//! This file only holds the shell function which is a JIT compiler
extern crate lang;

use crate::ast;

use lang::{get_associated_keywords, get_associated_messages};
use crate::config_file::ConfigFile;

/// JIT shell
///
/// Shell that uses JIT compilation with options to dump tokens and ASTs for each input
pub fn shell(config_file: ConfigFile, dt: bool, dast: bool) {
    let mut current_language = get_associated_keywords(&*config_file.default_lang).unwrap();
    let mut keyword_code = config_file.default_lang;

    // History setup
    let mut history: Vec<String> = Vec::new();
    let mut history_not_full = true;
    let mut current_index = 0u8;
    // let mut cursor_index = current_index;

    loop {
        let mut given = String::new();
        match std::io::stdin().read_line(&mut given) {
            Ok(_) => {}
            Err(e) => { panic!("Could not read input!\n{}", e); }
        }

        if history_not_full {
            history.push(given.clone());
            if history.len() as u8 == config_file.history_length.value {
                history_not_full = false;
            }
        } else {
            history[current_index as usize] = given.clone();
        }
        current_index += 1;
        current_index %= config_file.history_length.value;
        // cursor_index = current_index;

        let mut lexer = ast::Lexer::new(given, current_language.clone(), keyword_code.clone());
        let tokens = match lexer.make_tokens() {
            Ok(t) => t,
            Err(e) => {
                println!("{}", e);
                continue
            }
        };
        if dt {
            for t in tokens.iter() {
                println!("{}", t)
            }
        }
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
            continue;
        }


        let mut parser = ast::Parser::new(tokens);
        let ast_list = match parser.parse() {
            Ok(t) => t,
            Err(e) => {
                println!("{}", e);
                continue;
            }
        };
        if dast {
            for (i, ast) in ast_list.iter().enumerate() {
                println!("** {} **\n{:?}", i + 1, ast)
            }
        }
    }
}
