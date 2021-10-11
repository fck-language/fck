use crate::keywords::*;

pub const KEYWORDS: Keywords = Keywords{
    keywords:
    ["und", "oder", "nicht", "wenn", "sonst", "sonn", "falls", "option", "standard",
        "iterieren", "bis", "importieren/verwenden", /* import/use */
        "stufe", "während", "def", "zurückschicken", "fortsetzen", "ausbrechen", "still", "als",
        "wahr", "falsch"
    ],
    var_keywords:
    // TODO: Work out the actual words
    ["int", "float", "bool", "list", "str"],
    config_keys:
    // TODO: Work out the actual words
    ["wrapLength", "shellLanguageChange", "historyLength"],
    manifest_keys:
    ["package", "name", "version", "authors", "edition", "flavour", "dependencies"],
    flavours:
    ["pure", "counting"]
};

pub const MESSAGES: Messages = Messages{
    generic: ["Die Shell-Sprache wurde auf Deutsch geändert"],
    errors: ErrorHolder{
        language_errors: [
            ErrorMessages{ name: "", desc: "" },
            ErrorMessages{ name: "", desc: "" }
        ],
        unknown_errors: [
            ErrorMessages{ name: "", desc: "" },
            ErrorMessages{ name: "", desc: "" }
        ],
        expected_errors: [
            ErrorMessages{ name: "", desc: "" },
            ErrorMessages{ name: "", desc: "" },
            ErrorMessages{ name: "", desc: "" },
            ErrorMessages{ name: "", desc: "" },
            ErrorMessages{ name: "", desc: "" },
            ErrorMessages{ name: "", desc: "" },
            ErrorMessages{ name: "", desc: "" },
            ErrorMessages{ name: "", desc: "" },
            ErrorMessages{ name: "", desc: "" }
        ]
    }
};
