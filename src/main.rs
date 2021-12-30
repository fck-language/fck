//! This is the source code for fck(pure). This is the basis for all other fck flavours

extern crate lang;
extern crate clap;

use crate::shell::shell;
use clap::{Arg, App, crate_version, SubCommand};
use std::{env::current_dir, io::Write, fs::read_to_string};
use std::path::Path;


mod tokens;
mod bases;
mod ast;
mod nodes;
mod shell;
mod config_file;
mod err_wrn;
mod interpreter;
mod types;

fn main() {
    //! Reads the local config file and parses arguments accordingly
    //!
    //! Reads the local config file and exits if none is found. Then, using the right language,
    //! parses CLAs and runs the associated piece of code, or prints the help message
    let config_file = config_file::read_config_file();
    let app = App::new("fck (pure)")
        .version(&*format!("v{}", crate_version!()))
        .arg(Arg::with_name("version")
            .short("v")
            .long("version")
            .takes_value(false)
            .help("Returns the current versions of fck"))
        .arg(Arg::with_name("file")
            .short("f")
            .long("file")
            .takes_value(true)
            .help("File or project path to run or compile"))
        .arg(Arg::with_name("info")
            .short("V")
            .long("info")
            .takes_value(false)
            .help("Returns info about the current installation of fck")).get_matches();

    if app.is_present("version") {

    } else if app.is_present("info") {

    } else if app.is_present("file") {

    } else {
        shell(config_file);
    }
}
