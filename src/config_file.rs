use std::collections::HashMap;
use std::fs::read_to_string;
use std::path::Path;
use lang::get_associated_keywords;
use std::fmt::{Debug, Formatter};
use std::ptr::write;
use clap::Format;

#[derive(Clone)]
pub struct ConfigFile {
    default_lang: String,
    wrap_length: ConfigU8,
    shell_language_info: bool,
    history_length: ConfigU8,
}

impl ConfigFile {
    pub fn new(default_language: String) -> ConfigFile {
        ConfigFile {
            default_lang: default_language,
            wrap_length: ConfigU8::new(70, 25, u8::MAX),
            shell_language_info: false,
            history_length: ConfigU8::new(100, 0, u8::MAX),
        }
    }

    pub fn new_value(&mut self, key_index: usize, int_value: Option<u8>, _string_value: Option<String>) {
        match key_index {
            0 => self.wrap_length.set(int_value.unwrap()),
            1 => self.shell_language_info = int_value.unwrap() != 0,
            2 => self.history_length.set(int_value.unwrap()),
            _ => {}
        };
    }

    pub fn value_type(&mut self, key_index: usize) -> u8 {
        // 0 => just what the fuck
        // 1 => u8
        // 2 => bool
        match key_index {
            0 => 1,
            1 => 2,
            2 => 1,
            _ => 0
        }
    }
}

impl Debug for ConfigFile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:<20} : {}\n{:<20} : {:?}\n{:<20} : {}\n{:<20} : {:?}\n", "default_lang", self.default_lang,
               "wrap_length", self.wrap_length,
               "shell_language_info", self.shell_language_info,
               "history_length", self.history_length
        )
    }
}

#[derive(Clone)]
struct ConfigU8 {
    value: u8,
    min_value: u8,
    max_value: u8,
}

impl ConfigU8 {
    pub fn new(value: u8, min_value: u8, max_value: u8) -> ConfigU8 {
        ConfigU8 { value, min_value, max_value }
    }

    pub fn set(&mut self, mut value: u8) {
        if value < self.min_value {
            value = self.min_value.clone()
        } else if value > self.max_value {
            value = self.max_value.clone()
        }
        self.value = value
    }
}

impl Debug for ConfigU8 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({} -> {})", self.value, self.min_value, self.max_value)
    }
}

pub fn read_config_file<'a>() -> ConfigFile {
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
    let raw_file = read_res.ok().unwrap();
    let mut read_file = raw_file.lines().collect::<Vec<&str>>();

    let keyword_match = get_associated_keywords(read_file[0].clone());
    if keyword_match.is_none() {
        println!("Unknown language code in config file '{}'!\nLanguage code should be the first line in the config file", read_file[0]);
        std::process::exit(1)
    }
    let config_keys = keyword_match.unwrap();
    let mut config_keys = config_keys.config_keys.iter();
    read_file.reverse();

    let mut out = ConfigFile::new(read_file.pop().unwrap().to_string());

    for line in read_file {
        let split: Vec<&str> = line.split('=').collect::<Vec<&str>>().iter().map(|x| x.trim()).collect();
        assert_eq!(split.len(), 2, "Each line should contain a key and associated value!\n{}", line);
        let position = config_keys.position(|&x| x == split[0]);
        if position.is_none() {
            println!("Unknown config file key '{}'!", { split[0] });
            continue;
        }
        match out.value_type(position.unwrap()) {
            0 => println!("What the fuck"),
            1 => {
                match split[1].parse::<u8>() {
                    Ok(v) => out.new_value(position.unwrap(),
                                           Some(v),
                                           None),
                    Err(e) => {
                        println!("{}", e);
                        continue;
                    }
                }
            }
            2 => {
                match split[1].parse::<bool>() {
                    Ok(v) => out.new_value(position.unwrap(),
                                           Some(if v { 1 } else { 0 }),
                                           None),
                    Err(e) => {
                        println!("{}", e);
                        continue;
                    }
                }
            }
            _ => {}
        }
    }
    out
}