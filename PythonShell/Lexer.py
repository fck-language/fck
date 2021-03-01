from Tokens import *
from Errors import *
from string import ascii_letters


##################################################
# CONSTANTS                                      #
##################################################

DIGITS = '0123456789'
LETTERS = ascii_letters
LETTERS_DIGITS = LETTERS + DIGITS


class Lexer:
    def __init__(self, fn, text):
        self.fn = fn
        self.text = text
        self.pos = Position(-1, 0, -1, fn, text)
        self.current_char = None
        self.advance()
        self.single_char_token_names = {'+': TT_PLUS, '%': TT_MOD, '(': TT_LPAREN, ')': TT_RPAREN, '{': TT_LPAREN_CURLY,
                                        '}': TT_RPAREN_CURLY, ',': TT_COMMA, '-': TT_MINUS, '[': TT_LPAREN_SQUARE,
                                        ']': TT_RPAREN_SQUARE, "\n": TT_NEWLINE, ';': TT_NEWLINE}
        self.multi_char_token_methods = {'!': self.make_not_equals, '=': self.make_equals, '<': self.make_less_than,
                                         '>': self.make_greater_than, '*': self.make_mult_pow, ':': self.make_set,
                                         '/': self.make_div, '"': self.make_string}

    def advance(self) -> None:
        self.pos.advance(self.current_char)
        self.current_char = self.text[self.pos.idx] if self.pos.idx < len(self.text) else None

    def make_tokens(self) -> (list, Error):
        tokens = []

        while self.current_char is not None:
            if self.current_char in ' \t':
                self.advance()
            elif self.current_char in DIGITS:
                tokens.append(self.make_number())
            elif self.current_char in LETTERS:
                tokens.append(self.make_identifier())
            else:
                found = False
                for i, n in self.single_char_token_names.items():
                    if self.current_char == i:
                        tokens.append(Token(n, pos_start=self.pos))
                        self.advance()
                        found = True
                        break
                for i, n in self.multi_char_token_methods.items():
                    if self.current_char == i:
                        tok, error = n()
                        if error: return [], error
                        tokens.append(tok)
                        found = True
                if not found:
                    pos_start = self.pos.copy()
                    char = self.current_char
                    self.advance()
                    return [], IllegalCharError(pos_start, self.pos, f'\'{char}\'')

        tokens.append(Token(TT_EOF, pos_start=self.pos))
        return tokens, None

    def make_number(self) -> Token:
        num_str = ''
        dot_count = 0
        pos_start = self.pos.copy()

        while self.current_char is not None and self.current_char in DIGITS + '.':
            if self.current_char == '.':
                if dot_count == 1:
                    break
                dot_count += 1
                num_str += '.'
            else:
                num_str += self.current_char
            self.advance()

        if dot_count:
            return Token(TT_FLOAT, float(num_str), pos_start=pos_start, pos_end=self.pos)
        else:
            return Token(TT_INT, int(num_str), pos_start=pos_start, pos_end=self.pos)

    def make_string(self):
        string = ""
        pos_start = self.pos.copy()
        escape_character = False
        self.advance()

        escape_characters = {'n': '\n', 't': '\t'}

        while self.current_char is not None and (self.current_char != '"' or escape_character):
            if escape_character:
                string += escape_characters.get(self.current_char, self.current_char)
                escape_character = False
            else:
                if self.current_char == '\\':
                    escape_character = True
                else:
                    string += self.current_char
            self.advance()

        self.advance()
        return Token(TT_STRING, string, pos_start, self.pos), None

    def make_identifier(self):
        id_str = ""
        pos_start = self.pos.copy()

        while self.current_char is not None and self.current_char in LETTERS_DIGITS + "_":
            id_str += self.current_char
            self.advance()

        tok_type = TT_KEYWORD if id_str in KEYWORDS else TT_IDENTIFIER
        return Token(tok_type, id_str, pos_start, self.pos)

    def make_not_equals(self):
        pos_start = self.pos.copy()
        self.advance()

        if self.current_char == '=':
            self.advance()
            return Token(TT_NE, pos_start=pos_start, pos_end=self.pos), None
        elif self.current_char in LETTERS + "_":
            return Token(TT_KEYWORD, 'not', pos_start=pos_start, pos_end=self.pos), None

        self.advance()
        return None, ExpectedCharError(pos_start, self.pos, "Expected '!='")

    def make_equals(self):
        pos_start = self.pos.copy()
        self.advance()

        if self.current_char == '=':
            self.advance()
            return Token(TT_EQ, pos_start=pos_start, pos_end=self.pos), None

        self.advance()
        return None, ExpectedCharError(pos_start, self.pos, "Expected '=='")

    def make_less_than(self):
        tok_type = TT_LT
        pos_start = self.pos.copy()
        self.advance()

        if self.current_char == '=':
            self.advance()
            tok_type = TT_LTE

        return Token(tok_type, pos_start=pos_start, pos_end=self.pos), None

    def make_greater_than(self):
        tok_type = TT_GT
        pos_start = self.pos.copy()
        self.advance()

        if self.current_char == '=':
            self.advance()
            tok_type = TT_GTE

        return Token(tok_type, pos_start=pos_start, pos_end=self.pos), None

    def make_mult_pow(self):
        tok_type = TT_MULT
        pos_start = self.pos.copy()
        self.advance()

        if self.current_char == '*':
            self.advance()
            tok_type = TT_POW

        return Token(tok_type, pos_start=pos_start, pos_end=self.pos), None

    def make_div(self):
        tok_type = TT_DIV
        pos_start = self.pos.copy()
        self.advance()

        if self.current_char == '/':
            self.advance()
            tok_type = TT_FDIV

        return Token(tok_type, pos_start=pos_start, pos_end=self.pos), None

    def make_set(self):
        pos_start = self.pos.copy()
        self.advance()
        operation_type = None
        set_op = {"+": [TT_SET_PLUS, TT_SET_RET_PLUS], "-": [TT_SET_MINUS, TT_SET_RET_MINUS],
                  "%": [TT_SET_MOD, TT_SET_RET_MOD]}
        set_op_double = {"/": [[TT_SET_DIV, TT_SET_RET_DIV], [TT_SET_FDIV, TT_SET_RET_FDIV]],
                         "*": [[TT_SET_MULT, TT_SET_RET_MULT], [TT_SET_POW, TT_SET_RET_POW]]}

        if self.current_char == ':':
            self.advance()
            return Token(TT_SET, pos_start=pos_start, pos_end=self.pos), None
        elif self.current_char == '>':
            self.advance()
            return Token(TT_SET_RET, pos_start=pos_start, pos_end=self.pos), None
        else:
            found = False
            for i, n in set_op.items():
                if self.current_char == i:
                    operation_type = n
                    self.advance()
                    found = True
                    break
            if not found:
                for i, n in set_op_double.items():
                    if self.current_char == i:
                        operation_type = n
                        found = True
                        self.advance()
                        if self.current_char == i:
                            operation_type = operation_type[1]
                            self.advance()
                        else:
                            operation_type = operation_type[0]
                        break
            if not found:
                return None, ExpectedCharError(pos_start, self.pos, "Expected assignment operator")

        if self.current_char not in (':', ">"):
            return None, ExpectedCharError(pos_start, self.pos, "Expected assignment operator")

        operation_type = operation_type[0 if self.current_char == ":" else 1]

        self.advance()
        return Token(operation_type, pos_start=pos_start, pos_end=self.pos), None
