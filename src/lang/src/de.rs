use crate::keywords::{Keywords, Messages};

pub const KEYWORDS: Keywords = Keywords{
    keywords:
    ["und", "oder", "nicht", "wenn", "sonst", "sonn", "falls", "option", "standard",
        "iterieren", "bis", "importieren/verwenden", /* import/use */
        "stufe", "während", "def", "zurückschicken", "fortsetzen", "ausbrechen", "still", "als"],
    var_keywords:
    // TODO: Work out the actual words
    ["int", "float", "bool", "list", "str"],
    config_keys:
    // TODO: Work out the actual words
    ["wrapLength", "shellLanguageChange", "historyLength"]
};

pub const MESSAGES: Messages = Messages{
    generic: ["Die Shell-Sprache wurde auf Deutsch geändert"]
};
