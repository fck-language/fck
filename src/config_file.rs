//! Config file handling
//!
//! Parses the config file and determines appropriate languages and options for the user
use std::fs::read_to_string;
use std::path::Path;
use lang::get_associated_keywords;
use std::fmt::{Debug, Formatter};
use lang::keywords::Keywords;

/// Config file struct that holds all the configurable options
#[derive(Clone)]
pub struct ConfigFile {
    /// The language to use if no language is specified. Also determines the language to parse the
    /// config file in
	pub default_lang: String,
    /// Wrap length of errors and warnings
    pub wrap_length: ConfigU8,
    /// When `true` this will inform the user when the shell language has been changed
    pub shell_language_info: bool,
    /// Number of previous lines to save when using the shell
    pub history_length: ConfigU8,
    /// Profile name
    pub name: Option<String>,
    /// Profile github account
    pub github: Option<String>,
    /// Profile email
    pub email: Option<String>,
}

impl ConfigFile {
    /// Creates a new ConfigFile strict from a given initial language code with default values
    pub fn new() -> ConfigFile {
        // don't you dare comment on how messy this is. please
        let config_file_path = format!("{}/.fck", std::env::var("HOME").unwrap());
        let config_file = Path::new(&config_file_path);
        if !config_file.exists() {
            println!(".fck config file does not exist and is required!\n\
            This file should be in your $HOME directory");
            std::process::exit(1)
        };
        let file_res = read_to_string(config_file);
        if file_res.is_err() {
            println!("{}", file_res.err().unwrap());
            std::process::exit(1)
        }
        let file = match read_to_string(config_file) {
            Ok(contents) => contents,
            Err(e) => {
                println!("{}", e);
                std::process::exit(1)
            }
        };
        let mut lines = file.lines();
        let lang;
        match lines.nth(0) {
            Some(l) => lang = l.to_string().to_owned(),
            None => {
                println!("Lang code needed");
                std::process::exit(1)
            }
        }
    
        let config_keys = match get_associated_keywords(&*lang) {
            None => {
                println!("Unknown language code in config file '{}'!\nLanguage code should be the first line in the config file", lang);
                std::process::exit(1)
            }
            Some(k) => k.config_keys
        };
    
        
        let mut out = ConfigFile {
            default_lang: lang,
            wrap_length: ConfigU8::new(70, 25, u8::MAX),
            shell_language_info: false,
            history_length: ConfigU8::new(100, 0, u8::MAX),
            name: None, github: None, email: None
        };
    
        for line in lines {
            if line.trim() == "" || line.trim().chars().nth(0).unwrap() == '#'{
                continue;
            }
            let split: Vec<&str> = line.split('=').collect::<Vec<&str>>().iter().map(|x| x.trim()).collect();
            assert_eq!(split.len(), 2, "Each line should contain a key and associated value!\n{}", line);
            match config_keys.iter().position(|&x| x == split[0].trim()) {
                None => {
                    println!("Unknown config file key '{}'!", split[0]);
                }
                Some(p) => {
                    match out.value_type(p) {
                        0 => {
                            let res = split[1].trim().parse::<u8>();
                            if res.is_err() {
                                println!("Cannot parse {} as integer", split[1].trim())
                            } else {
                                out.set(p, split[1].trim())
                            }
                        }
                        1 => {
                            let res = split[1].trim().parse::<bool>();
                            if res.is_err() {
                                println!("Cannot parse {} as integer", split[1].trim())
                            } else {
                                out.set(p, split[1].trim())
                            }
                        }
                        2 => out.set(p, split[1].trim()),
                        _ => unreachable!()
                    }
                }
            }
        }
        out
    }

    /// Set a certain config option, indexed by value:
    /// 0. wrap length
    /// 1. shell language info
    /// 2. shell history length
    /// 3. profile name
    /// 4. profile github
    /// 5. profile email
    pub fn set(&mut self, key_index: usize, value: &str) {
        match key_index {
            0 => self.wrap_length.set(value.parse::<u8>().unwrap()),
            1 => self.shell_language_info = value.parse::<bool>().unwrap(),
            2 => self.history_length.set(value.parse::<u8>().unwrap()),
            3 => self.name = Some(value.to_string()),
            4 => self.github = Some(value.to_string()),
            5 => self.email = Some(value.to_string()),
            _ => {}
        };
    }

    /// Function to determine what type of value each configurable option takes. Returns from 0 to 2
    /// inclusive for `u8`, `bool`, and `String` respectively
    pub fn value_type(&mut self, key_index: usize) -> u8 {
        // 0 => u8
        // 1 => bool
        // 2 => String
        match key_index {
            0 | 2 => 0,
            1 => 1,
            _ => 2
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

/// Special type of u8 with a minimum and maximum value. Used for numerical values in the config
/// file that have specific ranges
#[derive(Clone)]
pub struct ConfigU8 {
    pub(crate) value: u8,
    min_value: u8,
    max_value: u8,
}

impl ConfigU8 {
    /// Creates a new ConfigU8 from a current, min, and max value
    pub fn new(value: u8, min_value: u8, max_value: u8) -> ConfigU8 {
        ConfigU8 { value, min_value, max_value }
    }

    /// Sets a new value. If this new value is outside of the range for the struct it's set to the
    /// min or max depending on which way the value is outside the range
    pub fn set(&mut self, mut value: u8) {
        self.value = value.max(self.min_value).min(self.max_value)
    }
}

impl Debug for ConfigU8 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({} -> {})", self.value, self.min_value, self.max_value)
    }
}