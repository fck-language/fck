//! This is the source code for fck (pure). This is the basis for all other fck flavours
//!
//! This controls the initial instantiation of all command line options, but deals with very little
//! handling of actual data. Anything interesting always happens somewhere else

extern crate lang;

use crate::shell::shell;
use std::{
    env::{ current_dir, args },
    io::Read,
    path::Path,
    process::exit,
	fs::{ File, write, create_dir_all, create_dir }
};
use colored::*;
use lang::{ get_associated_keywords, get_associated_messages, keywords::Keywords };
use lang::keywords::ErrorHolder;
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
	let flag = |t: &str| format!("--{}", t);
    let config_file = ConfigFile::new();
	let lang = match get_associated_keywords(&*config_file.default_lang) {
		None => {
			println!("unknown language {}", config_file.default_lang);
			exit(1);
		}
		Some(k) => k
	};
	let mut args = args().collect::<Vec<String>>().get(1..).unwrap().to_vec();
	match args.clone().first() {
			None => shell(config_file, args.contains(&flag("debug"))),
			Some(cmd) => {
				args.remove(0);
				match lang.cli_commands.iter().position(|&t| t == &*cmd) {
					None => {
						println!("{}", "error message goes here");
						exit(1);
					},
					Some(i) => {
						match i {
							0 => unimplemented!("Testing not yet implemented"),
							1 => todo!("Info"),
							2 | 3 => {
								let debug;
								if let Some(p) = args.iter().position(|t| t == &flag(lang.cli_args[0])) {
									debug = true;
									args.remove(p);
								} else {
									debug = false
								}
								let dump_llvm;
								if let Some(p) = args.iter().position(|t| t == &flag(lang.cli_args[1])) {
									dump_llvm = true;
									args.remove(p);
								} else {
									dump_llvm = false
								}
								match args.len() {
									0 => unimplemented!("Running a project not yet worked out"),
									1 => {
										run_file(args.pop().unwrap(), config_file, dump_llvm || debug, debug);
										if i == 2 {
										
										}
									}
									_ => {
										exit(1)
									}
								}
							}
							// 	Self::Run(
							// 	args().contains(&*format!("--{}", keywords.cli_args[0])),
							// 	args().contains(&*format!("--{}", keywords.cli_args[1]))
							// ),
							4 => unimplemented!("Linting not yet implemented"),
							5 => todo!("Raw"),
							6 => shell(config_file, args.contains(&flag("debug"))),
							7 => unimplemented!("Doc generation not yet written"),
							8 => {
								if let Some(name) = args.pop() {
									let mut cwd = current_dir().unwrap();
									cwd.push(name.clone());
									if cwd.exists() {
										return
									}
									create_dir_all(cwd.clone());
									cwd.push("3.yml");
									File::create(cwd.clone());
									write(cwd.clone(), format!(
										"0: {}\n{}:\n      {}: {:?}\n      {}: {{{}: \"\"}}\n      {}: [\n{}      ]",
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
									write(cwd, format!("!!{}\n\n\n", config_file.default_lang));
								} else {
									println!("{}", "error message goes here");
									exit(1);
								}
							}
							_ => unreachable!()
						}
					}
				}
			}
		}
    // translator::translate(
    //     "Example scripts/translate_test.fck",
    //     get_associated_keywords(&*config_file.default_lang).unwrap(),
    //     "fr"
    // );
    // std::process::exit(0);
    

    // if app.is_present("info") {
    //     println!("{}\n{}", "fck".underline(),
    //              [("Flavour", "Pure"), ("Version", &*format!("v{}", env!("CARGO_PKG_VERSION")))]
    //                  .iter()
    //                  .map(|(l, r)|
    //                      format!("{:<10}: {}", l, r)
    //                  ).collect::<Vec<String>>().join("\n")
    //     );
    // } else if app.is_present("file or raw code") {
    //     run_file(
    //         app.value_of("file or raw code").unwrap(), config_file,
    //         app.is_present("dump LLVM IR") || app.is_present("debug mode"),
    //          app.is_present("debug mode")
    //     );
    // } else {
    //     shell(config_file, app.is_present("dump tokens"), app.is_present("dump ASTs"));
    // }
}

fn run_file(path: String, config_file: ConfigFile, dump_llvm: bool, debug: bool) {
    let mut cwd = current_dir().unwrap();
	cwd.extend(Path::new(&path));
	let full_file_path = cwd.file_stem().unwrap().to_str().unwrap().to_string();
	let file_name = get_file_name(path.clone());
	let mut file = String::new();
    if let Ok(mut file_container) = std::fs::File::open(cwd) {
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
        config_file.default_lang.clone()
    ).make_tokens() {
        Ok(toks) => toks,
        Err(e) => {
            println!("{}{}", if debug { "Token error: " } else { "" }, e);
            println!("{}\n{}",
                     get_associated_messages(&*config_file.default_lang).unwrap().errors.get_name(e.error_index - 1),
                     e.show_position(file));
            exit(1)
        }
    };
    if debug {
        let w = (tokens.len() as f64).log10() as usize + 1;
        println!("{}", format!("{}({})", keywords.debug_words.get(0).unwrap(), tokens.len()).bold().underline());
        for (i, tok) in tokens.iter().enumerate() {
            println!("{} {:5?}", format!("{:0>w$})", i).bold(), tok);
        }
        println!();
    }
    let (ast_vec, st_vec) = match ast::Parser::new(tokens).parse() {
        Ok(asts) => asts,
        Err(e) => {
            println!("{}{}", if debug { "Parse error: " } else { "" }, e);
            println!("{}\n{}",
                     get_associated_messages(&*config_file.default_lang.clone()).unwrap().errors.get_name(e.error_index - 1),
                     e.show_position(file));
            exit(1)
        }
    };
    if debug {
        println!("{}", format!("{}({})", keywords.debug_words.get(1).unwrap(), ast_vec.len()).bold().underline());
        for (i, ast) in ast_vec.iter().enumerate() {
            println!("{}:\n{:?}", format!("{:>03}", i).bold(), ast)
        }
        println!("{}", format!("{}({})", keywords.debug_words.get(2).unwrap(), st_vec.len()).bold().underline());
        for (i, st) in st_vec.iter().enumerate() {
            println!("{}) {:5?}", format!("{:>03}", i).bold(), st)
        }
        println!();
    }
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

fn get_file_name(path: String) -> String {
    path.split("/")
        .last()
        .unwrap()
        .split(".")
        .nth(0)
        .unwrap()
        .to_string()
}
