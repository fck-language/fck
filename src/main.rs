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
    env::{current_dir, args},
    io::Read,
    process::exit,
    fs::{File, write, create_dir_all, create_dir, read_to_string, canonicalize},
    ffi::OsStr,
    path::PathBuf
};
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
mod translator;

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
            if path.is_file() {
                match path.extension() {
                    None => unimplemented!("the fuck kinda file has no file extension"),
                    Some(t) => match t.to_str() {
                        None => {}
                        Some("fck") => run_file(path.clone(), config_file, llvm),
                        Some("hck") => todo!("Can't run a header file darling xx"),
                        Some(v) => todo!("error because you cant run a .{} file", v)
                    }
                }
            }
        }
        Command::Run { path, llvm, no_build } => {
            if path.is_file() {
                if no_build {
                    todo!("warning because not allowed fucker")
                }
                match path.extension() {
                    None => unimplemented!("the fuck kinda file has no file extension"),
                    Some(t) => match t.to_str() {
                        None => {}
                        Some("fck") => {
                            run_file(path.clone(), config_file, llvm);
                            let full_name = path.to_str().unwrap().to_string();
                            match std::process::Command::new(
                                format!("{}{}", full_name.get(0..full_name.len() - 4).unwrap(), std::env::consts::EXE_SUFFIX)
                            ).spawn() {
                                Ok(child) => {
                                    match child.wait_with_output() {
                                        Ok(res) => {
                                            if !res.stdout.is_empty() {
                                                println!("{}", res.stdout.iter().fold(
                                                    String::new(),
                                                    |acc, c| format!("{}{}", acc, char::from(*c)))
                                                );
                                            }
                                            println!("{}", res.status.code().unwrap())
                                        }
                                        Err(e) => {
                                            println!("{:?}", e);
                                            exit(1)
                                        }
                                    }
                                },
                                Err(e) => {
                                    println!("{:?}", e);
                                    exit(1)
                                }
                            };
                        },
                        Some("hck") => todo!("Can't run a header file darling xx"),
                        Some(v) => todo!("error because you cant run a .{} file", v)
                    }
                }
            } else {
                unimplemented!("Haven't added in running projects yet sorry")
            }
        }
        Command::Test { .. } => { unimplemented!() }
        Command::Info => {}
        Command::Lint { .. } => { unimplemented!() }
        Command::Raw { .. } => {}
        Command::Doc { .. } => {}
        Command::Translate { path, output, target_language, comment } => {
            if path.is_file() {
                if path.extension() == Some(&OsStr::new("fck")) {
                    let contents = read_to_string(path.clone()).unwrap();
                    match translator::translate(contents, lang, config_file.default_lang, &*target_language, comment) {
                        Ok(f) => {
                            if !output.exists() { create_dir(output.parent().unwrap()); }
                            if path != output {
                                write(output, f);
                            }
                        }
                        Err(e) => {
                            println!("{:?}", e);
                            exit(1)
                        }
                    }
                }
            }
        }
    }
}

// The idea was that in order to ensure the file had been built to an executable, the result of this would have to be Ok(()). It doesn't.
// If something goes wrong then the code will exit. therefore if nothing goes wrong then the file was made properly tada all is well in the world xx
fn run_file(path: PathBuf, config_file: ConfigFile, dump_llvm: bool) {
    let mut cwd = current_dir().unwrap();
    cwd.push(path.clone());
	let full_file_path = cwd.to_str().unwrap().to_string();
    let full_file_path = full_file_path.get(0..full_file_path.len() - 4).unwrap().to_string();
	let file_name = path.file_name().unwrap().to_str().unwrap().to_string();
	let mut file = String::new();
    if let Ok(mut file_container) = File::open(cwd.clone()) {
        if file_container.read_to_string(&mut file).is_err() {
            println!("Failed to read file \"{}\"", canonicalize(cwd).unwrap().to_str().unwrap());
            exit(5)
        };
    } else {
        println!("File \"{}\" does not exist!", canonicalize(cwd).unwrap().to_str().unwrap());
        exit(2);
    }
    let keywords = get_associated_keywords(&*config_file.default_lang).unwrap();
    let tokens = match ast::Lexer::new(
        file.clone(),
        keywords,
        config_file.default_lang.clone(),
        false
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
        let here = PathBuf::from(&ll_path);
        if let Err(e) = write(here, format!("{}", module)) {
            println!("{} `{}`:\n{}", keywords.debug_words.get(5).unwrap(), canonicalize(format!("{}.ll", full_file_path)).unwrap().to_str().unwrap(), e)
        } else {
            println!("{} {}", keywords.debug_words.get(6).unwrap(), canonicalize(format!("{}.ll", full_file_path)).unwrap().to_str().unwrap());
        };
    }
    llvm::to_object_file(module.module.to_owned(), format!("{}.o", full_file_path));
    llvm::object_to_executable(full_file_path);
}
