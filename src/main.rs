//! This is the source code for fck (pure). This is the basis for all other fck flavours
//!
//! This controls the initial instantiation of all command line options, but deals with very little
//! handling of actual data. Anything interesting always happens somewhere else

extern crate lang;
extern crate clap;

use crate::shell::shell;
use clap::{Arg, Command};
use std::{
    env::current_dir,
    io::Read,
    path::Path,
    process::exit
};
use colored::*;
use lang::get_associated_keywords;
use crate::config_file::ConfigFile;

mod bases;
mod ast;
mod nodes;
mod shell;
mod config_file;
mod err_wrn;
mod llvm;

/// Reads the local config file and parses arguments accordingly
///
/// Reads the local config file and exits if none is found. Then, using the right language,
/// parses CLAs and runs the associated piece of code, or prints the help message
fn main() {
    let config_file = config_file::read_config_file();
    // translator::translate(
    //     "Example scripts/translate_test.fck",
    //     get_associated_keywords(&*config_file.default_lang).unwrap(),
    //     "fr"
    // );
    // std::process::exit(0);
    let app = Command::new("fck (pure)")
        .version(&*format!("v{}", env!("CARGO_PKG_VERSION")))
        .arg(Arg::new("version")
            .short('v')
            .long("version")
            .takes_value(false)
            .help("Returns the current versions of fck"))
        .arg(Arg::new("file")
            .takes_value(true)
            .index(1)
            .help("File or project path to run or compile"))
        .arg(Arg::new("info")
            .short('V')
            .long("info")
            .takes_value(false)
            .help("Returns info about the current installation of fck"))
        .arg(Arg::new("dump LLVM IR")
            .long("dump-llvm")
            .takes_value(false)
            .help("Dump the LLVM IR code to a .ll file"))
        .arg(Arg::new("debug mode")
            .long("debug")
            .takes_value(false)
            .help("Dumps all possible debug things"))
        .get_matches();

    if app.is_present("info") {
        println!("{}\n{}", "fck".underline(),
                 [("Flavour", "Pure"), ("Version", &*format!("v{}", env!("CARGO_PKG_VERSION")))]
                     .iter()
                     .map(|(l, r)|
                         format!("{:<10}: {}", l, r)
                     ).collect::<Vec<String>>().join("\n")
        );
    } else if app.is_present("file") {
        run_file(
            app.value_of("file").unwrap(), config_file,
            app.is_present("dump LLVM IR") || app.is_present("debug mode"),
             app.is_present("debug mode")
        );
    } else {
        shell(config_file, app.is_present("dump tokens"), app.is_present("dump ASTs"));
    }
}

fn run_file(path: &str, config_file: ConfigFile, dump_llvm: bool, debug: bool) {
    let file_name = get_file_name(path);
    let full_file_path = format!("{}/{}", current_dir().unwrap().to_str().unwrap(), path.get(..path.len() - 4).unwrap().to_string());
    let mut file = String::new();
    if let Ok(mut file_container) = std::fs::File::open(path) {
        if file_container.read_to_string(&mut file).is_err() {
            println!("Failed to read file \"{}\"", path);
            exit(5)
        };
    } else {
        println!("File \"{}\" does not exist!", path);
        exit(2);
    }
    let keywords = get_associated_keywords(&*config_file.default_lang).unwrap();
    let tokens = match ast::Lexer::new(
        file.clone(),
        keywords,
        config_file.default_lang
    ).make_tokens() {
        Ok(toks) => toks,
        Err(e) => {
            println!("{}{}\n{}", if debug { "Token error: " } else { "" }, e, e.show_position(file));
            exit(1)
        }
    };
    if debug {
        println!("{}", keywords.debug_words.get(0).unwrap().bold().underline());
        for (i, tok) in tokens.iter().enumerate() {
            println!("{} {:5}", format!("{:>03})", i).bold(), tok);
        }
        println!();
    }
    let (ast_vec, st_vec) = match ast::Parser::new(tokens).parse() {
        Ok(asts) => asts,
        Err(e) => {
            println!("{}{}\n{}", if debug { "Parse error: " } else { "" }, e, e.show_position(file));
            exit(1)
        }
    };
    if debug {
        println!("{}", keywords.debug_words.get(1).unwrap().bold().underline());
        for (i, ast) in ast_vec.iter().enumerate() {
            println!("{}:\n{:?}", format!("{:>03}", i).bold(), ast)
        }
        println!("{}", keywords.debug_words.get(2).unwrap().bold().underline());
        for (i, st) in st_vec.iter().enumerate() {
            println!("{}) {:5?}", format!("{:>03}", i).bold(), st)
        }
        println!();
    }
    let module = llvm::ir_to_module(&*file_name, ast_vec);
    println!("{}", keywords.debug_words.get(3).unwrap());
    if dump_llvm {
        print!("{}...\r", keywords.debug_words.get(4).unwrap());
        let ll_path = format!("{}.ll", full_file_path);
        let here = Path::new(&ll_path);
        if let Err(e) = std::fs::write(here, format!("{}", module)) {
            println!("{} `{}.ll`:\n{}", keywords.debug_words.get(5).unwrap(), full_file_path, e)
        } else {
            println!("{} {}.ll", keywords.debug_words.get(6).unwrap(), full_file_path);
        };
    }
    llvm::to_object_file(module.module.to_owned(), format!("{}.o", full_file_path));
    llvm::object_to_executable(full_file_path);
}

fn get_file_name(path: &str) -> String {
    path.split("/")
        .last()
        .unwrap()
        .split(".")
        .nth(0)
        .unwrap()
        .to_string()
}
