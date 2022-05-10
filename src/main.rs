//! This is the source code for fck (pure). This is the basis for all other fck flavours
//!
//! This controls the initial instantiation of all command line options, but deals with very little
//! handling of actual data. Anything interesting always happens somewhere else

#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![doc(html_logo_url = "https://raw.githubusercontent.com/fck-language/fck/master/img/logo/icon.jpeg")]

extern crate lang;
extern crate git2;

use crate::shell::shell;
use std::{
    env::{ current_dir, args },
    io::Read,
    path::Path,
    process::exit,
	fs::{ File, write, create_dir_all, create_dir }
};
use std::borrow::BorrowMut;
use std::path::PathBuf;
use colored::*;
use git2::Repository;
use lang::{Command, get_associated_keywords, get_associated_messages, get_cli, keywords::Keywords};
use lang::keywords::ErrorHolder;
use fck::err_wrn::Error;
use type_things::symbol_tables::CompSymbolTable;
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
    let config_file = ConfigFile::new();
	let lang = match get_associated_keywords(&*config_file.default_lang) {
		None => {
			println!("unknown language {}", config_file.default_lang);
			exit(1);
		}
		Some(k) => k
	};
    match get_cli(&*config_file.default_lang).unwrap() {
        Command::New { path, git } => {
            let mut cwd = current_dir().unwrap();
            cwd.push(path.clone());
            let name = path.file_name().unwrap().to_str().unwrap();
            if cwd.exists() {
                return
            }
            create_dir_all(cwd.clone());
            cwd.push("3.yml");
            File::create(cwd.clone());
            write(cwd.clone(), format!(
                "0: {}\n{}:\n      {}: {:?}\n      {}: {{{}: \"\"}}\n      {}: [\n{}      ]\n",
                config_file.default_lang, lang.manifest_keys[0],
                lang.manifest_keys[1], name, lang.manifest_keys[3],
                lang.manifest_keys[2], lang.manifest_keys[4],
                if config_file.name.is_some() || config_file.github.is_some() || config_file.email.is_some() {
                    let f = |k: &str, v: String| format!("{}: {:?}", k, v);
                    let mut out = vec![];
                    if let Some(v) = config_file.name.clone() {
                        out.push(f(lang.manifest_keys[1], v))
                    }
                    if let Some(v) = config_file.github.clone() {
                        out.push(f(lang.manifest_keys[5], v))
                    }
                    if let Some(v) = config_file.email.clone() {
                        out.push(f(lang.manifest_keys[6], v))
                    }
                    format!("            {{{}}}\n", out.join(", "))
                } else {
                    String::new()
                }
            ));
            cwd.pop();
            cwd.push("0");
            create_dir(cwd.clone());
            cwd.push("0.fck");
            File::create(cwd.clone());
            write(cwd.clone(), format!("!!{}\n\n\n", config_file.default_lang));
            cwd.pop();
            cwd.pop();
            if git { Repository::init(cwd); }
        }
        Command::Shell => shell(config_file, false),
        Command::Build { path, llvm} => {
            run_file(path, config_file, llvm)
        }
        Command::Run { path, llvm, no_build } => {
            if !no_build {
                run_file(path, config_file, llvm)
            }
            if no_build {
                unimplemented!("Haven't yet added in the --no-build option")
            }
        }
        Command::Test { .. } => { unimplemented!() }
        Command::Info => {}
        Command::Lint { .. } => { unimplemented!() }
        Command::Raw { .. } => {}
        Command::Doc { .. } => {}
    }
}

// TODO: Need to refactor this function to return Result<(), Error>
fn run_file(path: PathBuf, config_file: ConfigFile, dump_llvm: bool) {
    let mut cwd = current_dir().unwrap();
    cwd.push(path.clone());
	let full_file_path = cwd.file_stem().unwrap().to_str().unwrap().to_string();
	let file_name = path.file_name().unwrap().to_str().unwrap().to_string();
	let mut file = String::new();
    if let Ok(mut file_container) = File::open(cwd.clone()) {
        if file_container.read_to_string(&mut file).is_err() {
            println!("Failed to read file \"{}\"", cwd.to_str().unwrap());
            exit(5)
        };
    } else {
        println!("File \"{}\" does not exist!", cwd.to_str().unwrap());
        exit(2);
    }
    let keywords = get_associated_keywords(&*config_file.default_lang).unwrap();
    let tokens = match ast::Lexer::new(
        file.clone(),
        keywords,
        config_file.default_lang.clone()
    ).make_tokens() {
        Ok(toks) => toks,
        Err(e) => {
            // Uncomment this if you're developing the project. Useful for debugging
            // dbg!("Token Error");
            println!("{}", e);
            println!("{}\n{}",
                     get_associated_messages(&*config_file.default_lang).unwrap().errors.get_name(e.error_index - 1),
                     e.show_position(file));
            exit(1)
        }
    };
    
    // Uncomment this if you're developing the project. Useful for debugging
    // let w = (tokens.len() as f64).log10() as usize + 1;
    // println!("{}", format!("{}({})", keywords.debug_words.get(0).unwrap(), tokens.len()).bold().underline());
    // for (i, tok) in tokens.iter().enumerate() {
    //     println!("{} {:5?}", format!("{:0>w$})", i).bold(), tok);
    // }
    // println!();
    
    let (ast_vec, st_vec) = match ast::Parser::new(tokens).parse() {
        Ok(asts) => asts,
        Err(e) => {
            // Uncomment this if you're developing the project. Useful for debugging
            // dbg!("Parse Error");
            println!("{}", e);
            println!("{}\n{}",
                     get_associated_messages(&*config_file.default_lang.clone()).unwrap().errors.get_name(e.error_index - 1),
                     e.show_position(file));
            exit(1)
        }
    };
    
    // Uncomment this if you're developing the project. Useful for debugging
    // println!("{}", format!("{}({})", keywords.debug_words.get(1).unwrap(), ast_vec.len()).bold().underline());
    // for (i, ast) in ast_vec.iter().enumerate() {
    //     println!("{}:\n{:?}", format!("{:>03}", i).bold(), ast)
    // }
    // println!("{}", format!("{}({})", keywords.debug_words.get(2).unwrap(), st_vec.len()).bold().underline());
    // for (i, st) in st_vec.iter().enumerate() {
    //     println!("{}) {:5?}", format!("{:>03}", i).bold(), st)
    // }
    // println!();
    
    let module;
    unsafe {
        module = llvm::ir_to_module(
            &*file_name, ast_vec,
            st_vec.iter().map(
                |t| t.into()
            ).collect::<Vec<CompSymbolTable>>()
        );
    }
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
