//! Language file for English
//!
//! Encoding: UTF-8

use crate::keywords::*;

pub const KEYWORDS: Keywords = Keywords{
    keywords:
    ["and", "or", "not", "if", "else", "elif", "case", "option", "default",
        "iterate", "to", "import", "step", "while", "def", "return", "continue", "break",
        "silent", "as", "true", "false"],
    var_keywords:
    ["int", "float", "bool", "list", "str"],
    config_keys:
    ["wrapLength", "shellLanguageInfo", "historyLength"],
    manifest_keys:
    ["[package]", "name", "version", "authors", "edition", "flavour", "[dependencies]"],
    flavours:
    ["pure", "counting"]
};

pub const MESSAGES: Messages = Messages{
    generic: ["The shell language has been changed to English"],
    errors: ErrorHolder{
        language_errors: [
            ErrorMessages{ name: "Unknown language code", desc: "Returned when an unknown language code is specified" },
            ErrorMessages{ name: "Incomplete language file", desc: "Returned when trying to use an incomplete language file" }
        ],
        unknown_errors: [
            ErrorMessages{ name: "Unknown character", desc: "Returned when you use a character that fck doesn't understand" },
            ErrorMessages{ name: "Unknown operator", desc: "You tried some sort of operation that I just don't know" }
        ],
        expected_errors: [
            ErrorMessages{ name: "Expected newline", desc: "Expected a newline or end or file" },
            ErrorMessages{ name: "Expected a condition", desc: "Expected a conditional statement here" },
            ErrorMessages{ name: "Expected opening bracket", desc: "Expected an opening bracket here" },
            ErrorMessages{ name: "Expected identifier", desc: "Expected an identifier" },
            ErrorMessages{ name: "Expected expression", desc: "" },
            ErrorMessages{ name: "Expected assignment operator", desc: "" },
            ErrorMessages{ name: "Expected colon (:)", desc: "" },
            ErrorMessages{ name: "Expected closing bracket", desc: "" },
            ErrorMessages{ name: "Expected type identifier", desc: "" }
        ],
        not_here_errors: [
            ErrorMessages{ name: "Cannot use a keyword here", desc: "Need to use an identifier that's not a keyword" }
        ],
        type_errors: [
            ErrorMessages{ name: "Expected type {} got {}", desc: "Returned when one type was found that cannot be cast into the required type" }
        ],
    }
};

//  0 "and"
//  1 "or"
//  2 "not"
//  3 "if"
//  4 "else"
//  5 "elif"
//  6 "case"
//  7 "option"
//  8 "default"
//  9 "iterate"
// 10 "to"
// 11 "import"
// 12 "step"
// 13 "while"
// 14 "def"
// 15 "return"
// 16 "continue"
// 17 "break"
// 18 "silent"
// 19 "as"
// 20 "true"
// 21 "false"
