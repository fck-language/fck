extern crate lang;
extern crate ErrWrn;
extern crate clap;

use crate::shell::shell;
use clap::{Arg, App, crate_version};
use std::io::Write;


mod tokens;
mod bases;
mod ast;
mod nodes;
mod shell;
mod config_file;

fn main() {
    let config_file = config_file::read_config_file();
    println!("{:?}", config_file);

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
