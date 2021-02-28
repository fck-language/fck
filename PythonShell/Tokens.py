TT_INT = "INT"
TT_FLOAT = "FLOAT"
TT_PLUS = "PLUS"
TT_MINUS = "MINUS"
TT_MULT = "MULT"
TT_DIV = "DIV"
TT_FDIV = "FDIV"
TT_MOD = "MOD"
TT_LPAREN = "LPAREN"
TT_RPAREN = "RPAREN"
TT_LPAREN_CURLY = "LPAREN_CURLY"
TT_RPAREN_CURLY = "RPAREN_CURLY"
TT_EOF = "EOF"
TT_POW = "POW"
TT_IDENTIFIER = "IDENTIFIER"
TT_KEYWORD = "KEYWORD"
TT_SET = "SET"
TT_SET_RET = "SET_RET"
TT_EQ = "EQ"
TT_NE = "NE"
TT_LT = "LT"
TT_GT = "GT"
TT_LTE = "LTE"
TT_GTE = "GTE",
TT_COMMA = "COMMA"

KEYWORDS = [
    "var",
    "int",
    "float",
    "bool",
    "and",
    "or",
    "not",
    "not_char",
    "if",
    "else",
    "elif",
    "iterate",
    "to",
    "step",
    "while",
    "def"
]


class Token:
    def __init__(self, type_, value=None, pos_start=None, pos_end=None):
        self.type = type_
        self.value = value
        if pos_start:
            self.pos_start = pos_start.copy()
            self.pos_end = pos_start.copy().advance()
        if pos_end:
            self.pos_end = pos_end

    def matches(self, type_, value):
        return self.type == type_ and self.value == value

    def __repr__(self):
        return f'{self.type}:{self.value}' if self.value else f'{self.type}'
