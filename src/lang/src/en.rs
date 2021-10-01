use crate::keywords::{Keywords, Messages};

pub const KEYWORDS: Keywords = Keywords{
    keywords:
    ["and", "or", "not", "if", "else", "elif", "case", "option", "default",
        "iterate", "to", "import", "step", "while", "def", "return", "continue", "break",
        "silent", "as", "true", "false"],
    var_keywords:
    ["int", "float", "bool", "list", "str"],
    config_keys:
    ["wrapLength", "shellLanguageInfo", "historyLength"]
};

pub const MESSAGES: Messages = Messages{
    generic: ["The shell language has been changed to english"]
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
