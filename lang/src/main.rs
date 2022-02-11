use std::fs::{File, write};
use std::path::Path;
use core::slice::Iter;
use std::io::Write;

pub mod keywords;
pub mod en;
pub mod fr;
pub mod de;
pub mod ko;

use std::str::FromStr;

fn csv(vector: Vec<&str>) -> String {
    let init = String::from(*vector.first().unwrap());
    vector[1..].iter().fold(init, |acc, arg| format!("{},{}", acc, arg))
}

fn write_formatted(code: &str, keywords: keywords::Keywords, messages: keywords::Messages) {
    let mut file_to_write_to = File::create(format!("./fckl equivelents/{}.fckl", code)).unwrap();
    let mut str = String::new();
    // Keywords
    str += &*csv(keywords.keywords.to_vec());
    str += "\n";
    str += &*csv(keywords.var_keywords.to_vec());
    str += "\n";
    str += &*csv(keywords.config_keys.to_vec());
    str += "\n";
    str += &*csv(keywords.manifest_keys.to_vec());
    str += "\n";
    str += &*csv(keywords.flavours.to_vec());
    // Messages
    str += "\n";
    str += &*csv(messages.generic.to_vec());
    file_to_write_to.write(str.as_ref());
}

fn main() {
    write_formatted("en", en::KEYWORDS, en::MESSAGES);
    write_formatted("de", de::KEYWORDS, de::MESSAGES);
    write_formatted("fr", fr::KEYWORDS, fr::MESSAGES);
    write_formatted("ko", ko::KEYWORDS, ko::MESSAGES);
}
