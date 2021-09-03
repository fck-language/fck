use crate::keywords::{Keywords, Messages};

pub const KEYWORDS: Keywords = Keywords{
    keywords:
    ["et", "ou", "non", "si", "autre", "auti", "cas", "option", "défaut",
        "répéter", "à", "import", "pas", "pendant", "déf", "rendre", "continuer", "interruption",
        "muet", "comme"],
    var_keywords:
    ["ent", "flottante", "bool", "liste", "chaîne"],
    config_keys:
    // TODO: Work out the actual words
    ["wrapLength", "shellLanguageChange", "historyLength"]
};

pub const MESSAGES: Messages = Messages{
    generic: ["La langue du shell a été changée en français"]
};
