use crate::keywords::Keywords;

pub const KEYWORDS_DE: Keywords = Keywords{
    keywords:
    ["und", "oder", "nicht", "wenn", "sonst", "sonn", "falls", "option", "standard",
        "iterieren", "bis", "importieren/verwenden", /* import/use */
        "stufe", "während", "def", "zurückschicken", "fortsetzen", "ausbrechen", "still", "als"],
    var_keywords:
    ["int", "float", "bool", "list", "str"]
};
