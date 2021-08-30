extern crate lang;
extern crate ErrWrn;
extern crate clap;

use crate::shell::shell;
use std::fs::read_to_string;
use std::path::Path;
use clap::{Arg, App, crate_version};
use lang::{keywords::Keywords, get_associated_keywords};
use ErrWrn::*;
use crate::bases::*;
use crate::ErrWrn::*;

use crate::ast::Parser;

mod tokens;
mod bases;
mod ast;
mod nodes;
mod shell;

fn main() {
    let raw_config_file = read_config_file();
    let config_file = raw_config_file.lines().collect::<Vec<&str>>();

    let app = App::new("fck")
        .version(crate_version!())
        .arg(Arg::with_name("version")
            .short("v")
            .long("version")
            .takes_value(false)
            .help("Prints info about the current installation of fck")
        )
        .arg(Arg::with_name("file")
            .short("f")
            .long("file")
            .takes_value(true)
            .help("File or project path to "))
        .arg(Arg::with_name("input")).get_matches();

    shell(get_associated_keywords(config_file[0]).unwrap());
}

fn read_config_file() -> String {
    // don't you dare comment on how messy this is. please
    let config_file_path = format!("{}/.fck", std::env::var("HOME").unwrap());
    let config_file = Path::new(&config_file_path);
    if !config_file.exists() {
        println!(".fck config file does not exist and is required!\n\
        This file should be in your $HOME directory");
        std::process::exit(1)
    };
    let read_res = read_to_string(config_file);
    if read_res.is_err() {
        println!("Could not read config file!\n{}", read_res.err().unwrap());
        std::process::exit(1)
    }
    read_res.ok().unwrap().clone()
}
