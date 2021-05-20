from string import ascii_letters
from os.path import exists, expanduser

DIGITS = '0123456789'
LETTERS = ascii_letters
LETTERS_DIGITS = LETTERS + DIGITS

config_file_path = expanduser('~') + '/.fck'
options = {'wrapLength': 70}
if exists(config_file_path):
    with open(config_file_path) as f:
        for lineNum, line in enumerate(f):
            line_split = line.split('=')
            if line_split[0] in options.keys():
                if line_split[0] == 'wrapLength':
                    try:
                        options['wrapLength'] = round(float(line_split[1]))
                        if options['wrapLength'] < 25:
                            print(f'\'wrapLength\' must be larger than 25 as a minimum!\n'
                                  f'File \'.fck\', line {lineNum + 1}:\n{line}')
                    except ValueError:
                        print(f'\'{line_split[1]}\' cannot be cast as an int and has been ignored!\n'
                              f'File \'.fck\', line {lineNum + 1}:\n{line}')

del config_file_path

wrap_length = options['wrapLength']

TT_INT = "INT"
TT_FLOAT = "FLOAT"
TT_STRING = "STRING"
TT_PLUS = "PLUS"
TT_MINUS = "MINUS"
TT_MULT = "MULT"
TT_DIV = "DIV"
TT_FDIV = "FDIV"
TT_MOD = "MOD"
TT_AT = "AT"
TT_LPAREN = "LPAREN"
TT_RPAREN = "RPAREN"
TT_LPAREN_CURLY = "LPAREN_CURLY"
TT_RPAREN_CURLY = "RPAREN_CURLY"
TT_LPAREN_SQUARE = "LPAREN_SQUARE"
TT_RPAREN_SQUARE = "RPAREN_SQUARE"
TT_EOF = "EOF"
TT_POW = "POW"
TT_NOT = "NOT"
TT_SEMICOLON = "SEMICOLON"
TT_IDENTIFIER = "IDENTIFIER"
TT_KEYWORD = "KEYWORD"
TT_QUESTION_MARK = "QUESTION_MARK"
TT_DOT = "DOT"
TT_SET = "SET"
TT_SET_RET = "SET_RET"
TT_EQ = "EQ"
TT_NE = "NE"
TT_LT = "LT"
TT_GT = "GT"
TT_LTE = "LTE"
TT_GTE = "GTE"
TT_COMMA = "COMMA"
TT_NEWLINE = "NEWLINE"
TT_SET_PLUS = "SET_PLUS"
TT_SET_RET_PLUS = "SET_RET_PLUS"
TT_SET_MINUS = "SET_MINUS"
TT_SET_RET_MINUS = "SET_RET_MINUS"
TT_SET_MULT = "SET_MULT"
TT_SET_RET_MULT = "SET_RET_MULT"
TT_SET_DIV = "SET_DIV"
TT_SET_RET_DIV = "SET_RET_DIV"
TT_SET_FDIV = "SET_FDIV"
TT_SET_RET_FDIV = "SET_RET_FDIV"
TT_SET_MOD = "SET_MOD"
TT_SET_RET_MOD = "SET_RET_MOD"
TT_SET_POW = "SET_POW"
TT_SET_RET_POW = "SET_RET_POW"
TT_GLOBAL_OPT = "GLOBAL_OPT"

VAR_SET = [TT_SET, TT_SET_RET, TT_SET_PLUS, TT_SET_RET_PLUS, TT_SET_MINUS, TT_SET_RET_MINUS, TT_SET_MULT,
           TT_SET_RET_MULT, TT_SET_DIV, TT_SET_RET_DIV, TT_SET_FDIV, TT_SET_RET_FDIV, TT_SET_MOD, TT_SET_RET_MOD,
           TT_SET_POW, TT_SET_RET_POW]
VAR_SET_RET = [TT_SET_RET, TT_SET_RET_PLUS, TT_SET_RET_MINUS, TT_SET_RET_MULT, TT_SET_RET_DIV, TT_SET_RET_FDIV,
               TT_SET_RET_MOD, TT_SET_RET_POW]
VAR_EQUIV = {TT_SET_RET: TT_SET, TT_SET_RET_PLUS: TT_SET_PLUS, TT_SET_RET_MINUS: TT_SET_MINUS,
             TT_SET_RET_MULT: TT_SET_MULT, TT_SET_RET_DIV: TT_SET_DIV, TT_SET_RET_FDIV: TT_SET_FDIV,
             TT_SET_RET_MOD: TT_SET_MOD, TT_SET_RET_POW: TT_SET_POW}
VAR_KEYWORDS = ["int", "float", "bool", "list", "str", "imag"]
NON_STATIC_VAR_KEYWORDS = ['auto']

KEYWORDS = ["and",
            "or",
            "not",
            "not_char",
            "if",
            "else",
            "elif",
            "case",
            "option",
            "default",
            "iterate",
            "to",
            "import",
            "step",
            "while",
            "def",
            "return",
            "continue",
            "break",
            "silent",
            "as"
            ] + VAR_KEYWORDS + NON_STATIC_VAR_KEYWORDS


class Context:
    def __init__(self, display_name, parent=None, parent_entry_pos=None):
        self.display_name = display_name
        self.parent = parent
        self.parent_entry_pos = parent_entry_pos
        self.symbol_table = None
        self.named_loops = []


class Position:
    def __init__(self, idx, ln, col, fn, ftxt):
        self.idx = idx
        self.ln = ln
        self.col = col
        self.fn = fn
        self.ftxt = ftxt

    def generate_tok_pos(self):
        return TokenPosition(self.ln, self.col)

    def advance(self, current_char=None):
        self.idx += 1
        self.col += 1

        if current_char == '\n':
            self.ln += 1
            self.col = 0

        return self

    def devance(self):
        if self.idx != 0:
            self.idx -= 1
            self.col -= 1

        return self

    def copy(self):
        return Position(self.idx, self.ln, self.col, self.fn, self.ftxt)


class TokenPosition:
    def __init__(self, ln, col):
        self.ln = ln
        self.col = col

    def advance(self):
        self.ln += 1
        return self

    def copy(self):
        return TokenPosition(self.ln, self.col)


class SymbolTable:
    def __init__(self, parent=None):
        self.symbols = {}
        self.constant_symbols = {}
        self.options = {'math': False, 'log': True}
        self.parent = parent

    def get(self, name):
        value = self.constant_symbols.get(name, None)
        if value is not None:
            return value, True
        value = self.symbols.get(name, None)
        if value is None and self.parent:
            return self.parent.get(name)
        return value, False

    def set(self, name, value):
        self.symbols[name] = value

    def remove(self, name):
        del self.symbols[name]

    def set_const(self, name, value):
        self.constant_symbols[name] = value


class Token:
    def __init__(self, type_, value=None, pos_start: TokenPosition = None, pos_end: TokenPosition = None):
        self.type = type_
        self.value = value
        if pos_start:
            self.pos_start = pos_start.copy()
            self.pos_end = pos_start.copy().advance()
        if pos_end:
            self.pos_end = pos_end

    def matches(self, type_, value):
        return self.type == type_ and self.value == value

    def list_matches(self, type_, value_list):
        return self.type == type_ and self.value in value_list

    def __repr__(self):
        return f'{self.type}:{self.value}' if self.value else f'{self.type}'
