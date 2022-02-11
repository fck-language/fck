use crate::keywords::*;

pub const KEYWORDS: Keywords = Keywords{
    keywords:
    ["et", "ou", "non", "si", "autre", "auti", "cas", "option", "défaut",
        "répéter", "à", "import", "pas", "pendant", "déf", "rendre", "continuer", "interruption",
        "muet", "comme", "vrai", "faux"],
    var_keywords:
    ["ent", "flottante", "bool", "liste", "chaîne"],
    config_keys:
    // TODO: Work out the actual words
    ["wrapLength", "shellLanguageChange", "historyLength"],
    manifest_keys:
    ["package", "name", "version", "authors", "edition", "flavour", "dependencies"],
    flavours:
    ["pure", "counting"]
};

pub const MESSAGES: Messages = Messages{
    generic: ["La langue du shell a été changée en français"],
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
        ],
        not_here_errors: [
            ErrorMessages{ name: "", desc: "" }
        ]
    }
};
