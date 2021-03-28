from re import split

from Values import *
from Results import *
from Nodes import *


#######################################
# LEXER
#######################################

class Lexer:
    def __init__(self, fn, text):
        self.fn = fn
        self.text = text
        self.pos = Position(-1, 0, -1, fn, text)
        self.current_char = None
        self.advance()
        self.single_char_token_names = {'+': TT_PLUS, '%': TT_MOD, '(': TT_LPAREN, ')': TT_RPAREN, '{': TT_LPAREN_CURLY,
                                        '}': TT_RPAREN_CURLY, ',': TT_COMMA, '-': TT_MINUS, '[': TT_LPAREN_SQUARE,
                                        ']': TT_RPAREN_SQUARE, "\n": TT_NEWLINE, ';': TT_NEWLINE, '?': TT_QUESTION_MARK,
                                        '@': TT_AT}
        self.multi_char_token_methods = {'!': self.make_not_equals, '=': self.make_equals, '<': self.make_less_than,
                                         '>': self.make_greater_than, '*': self.make_mult_pow, ':': self.make_set,
                                         '/': self.make_div, '"': self.make_string, "'": self.make_string,
                                         '#': self.skip_comment}

    def advance(self) -> None:
        self.pos.advance(self.current_char)
        self.current_char = self.text[self.pos.idx] if self.pos.idx < len(self.text) else None

    def devance(self) -> None:
        self.pos.devance()
        self.current_char = self.text[self.pos.idx] if self.pos.idx < len(self.text) else None

    def make_tokens(self) -> (list, Error):
        tokens = []

        while self.current_char is not None:
            if self.current_char in ' \t':
                self.advance()
            elif self.current_char in DIGITS + '.':
                tokens.append(self.make_number())
            elif self.current_char in LETTERS:
                tokens.extend(self.make_identifier())
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
                        res = n()
                        if res:
                            tok, error = res
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
        dot_count = 0
        pos_start = self.pos.copy()
        num_str = ''

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
        string += self.current_char
        self.advance()

        escape_characters = {'n': '\n', 't': '\t'}

        while self.current_char is not None and (self.current_char not in '"\'' or escape_character):
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
        id_str += self.current_char
        self.advance()

        while self.current_char is not None and self.current_char in LETTERS_DIGITS + "_":
            id_str += self.current_char
            self.advance()

        out = [Token(TT_KEYWORD if id_str in KEYWORDS else TT_IDENTIFIER, id_str, pos_start, self.pos)]
        pos_start = self.pos.copy()

        if self.current_char == '.':
            self.advance()
            out.append(Token(TT_DOT, pos_start=pos_start, pos_end=self.pos))

        return out

    def make_not_equals(self):
        pos_start = self.pos.copy()
        self.advance()

        if self.current_char == '=':
            self.advance()
            return Token(TT_NE, pos_start=pos_start, pos_end=self.pos), None
        elif self.current_char in LETTERS + "_":
            return Token(TT_NOT, pos_start=pos_start, pos_end=self.pos), None
        elif self.current_char == '!':
            return Token(TT_NOT, pos_start=pos_start, pos_end=self.pos), None

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
                return Token(TT_SEMICOLON, pos_start=pos_start, pos_end=self.pos), None

        if self.current_char not in (':', ">"):
            if operation_type not in ([TT_SET_PLUS, TT_SET_RET_PLUS], [TT_SET_MINUS, TT_SET_RET_MINUS]):
                return None, ExpectedCharError(pos_start, self.pos, "Expected assignment operator")
            self.devance()
            return Token(TT_SEMICOLON, pos_start=pos_start, pos_end=self.pos), None

        operation_type = operation_type[0 if self.current_char == ":" else 1]

        self.advance()
        return Token(operation_type, pos_start=pos_start, pos_end=self.pos), None

    def skip_comment(self):
        self.advance()

        while self.current_char != '\n':
            self.advance()

        self.advance()


#######################################
# PARSER
#######################################

class Parser:
    def __init__(self, tokens):
        self.tokens = tokens
        self.tok_idx = -1
        self.advance()

    def advance(self):
        self.tok_idx += 1
        self.update_current_tok()
        return self.current_tok

    def reverse(self, amount=1):
        self.tok_idx -= amount
        self.update_current_tok()
        return self.current_tok

    def update_current_tok(self):
        if 0 <= self.tok_idx < len(self.tokens):
            self.current_tok = self.tokens[self.tok_idx]

    def parse(self):
        res = self.statements()
        if not res.error and self.current_tok.type != TT_EOF:
            return res.failure(InvalidSyntaxError(
                self.current_tok.pos_start, self.current_tok.pos_end,
                "Token cannot appear after previous tokens"
            ))
        return res

    ###################################

    def statements(self):
        res = ParseResult()
        statements = []
        pos_start = self.current_tok.pos_start.copy()

        while self.current_tok.type == TT_NEWLINE:
            res.register_advancement()
            self.advance()

        statement = res.register(self.statement())
        if res.error: return res
        statements.append(statement)

        more_statements = True

        while True:
            newline_count = 0
            while self.current_tok.type == TT_NEWLINE:
                res.register_advancement()
                self.advance()
                newline_count += 1
            if newline_count == 0:
                more_statements = False

            if not more_statements: break
            statement = res.try_register(self.statement())
            if not statement:
                self.reverse(res.to_reverse_count)
                more_statements = False
                continue
            statements.append(statement)

        return res.success(ListNode(
            statements,
            pos_start,
            self.current_tok.pos_end.copy()
        ))

    def statement(self):
        res = ParseResult()
        pos_start = self.current_tok.pos_start.copy()

        if self.current_tok.matches(TT_KEYWORD, 'return'):
            res.register_advancement()
            self.advance()

            expr = res.try_register(self.expr())
            if not expr:
                self.reverse(res.to_reverse_count)
            return res.success(ReturnNode(expr, pos_start, self.current_tok.pos_start.copy()))

        if self.current_tok.matches(TT_KEYWORD, 'continue'):
            res.register_advancement()
            self.advance()
            return res.success(ContinueNode(pos_start, self.current_tok.pos_start.copy()))

        if self.current_tok.matches(TT_KEYWORD, 'break'):
            res.register_advancement()
            self.advance()
            if self.current_tok.type == TT_AT:
                res.register_advancement()
                self.advance()
                if not self.current_tok.type == TT_IDENTIFIER:
                    return res.failure(InvalidSyntaxError(pos_start, self.current_tok.pos_end,
                                                          'Expected loop identifier after \'@\''))
                loop_name = self.current_tok.value
                res.register_advancement()
                self.advance()
                return res.success(BreakNode(pos_start, self.current_tok.pos_start.copy(), loop_name))
            return res.success(BreakNode(pos_start, self.current_tok.pos_start.copy()))

        expr = res.register(self.expr())
        if res.error:
            return res.failure(InvalidSyntaxError(
                self.current_tok.pos_start, self.current_tok.pos_end,
                "Expected method, value, identifier, or operator"
            ))
        return res.success(expr)

    def expr(self):
        res = ParseResult()

        if self.current_tok.list_matches(TT_KEYWORD, VAR_TYPES):
            var_type = self.current_tok.value
            res.register_advancement()
            self.advance()

            var_name = self.current_tok
            res.register_advancement()
            self.advance()

            tok_type = self.current_tok.type

            if tok_type in (TT_SET, TT_SET_RET):
                res.register_advancement()
                self.advance()
                expr = res.register(self.expr())
                if res.error: return res
                return res.success(VarAssignNode(var_type, var_name, expr, True if tok_type == TT_SET_RET else False))
            return res.success(VarAssignNode(var_type, var_name, None, True if tok_type == TT_SET_RET else False))

        elif self.current_tok.matches(TT_KEYWORD, 'silent'):
            res.register_advancement()
            self.advance()
            if self.current_tok.type != TT_LT:
                return res.failure(InvalidSyntaxError(self.current_tok.pos_start, self.current_tok.pos_end,
                                                      'Expected \'<\' after \'silent\' keyword'))
            res.register_advancement()
            self.advance()
            if not self.current_tok.list_matches(TT_KEYWORD, SILENCABLE_TYPES):
                expanded_list = ", ".join([f'\'{silencable_type}\'' for silencable_type in SILENCABLE_TYPES])
                return res.failure(InvalidSyntaxError(self.current_tok.pos_start, self.current_tok.pos_end,
                                                      f'Expected one of {expanded_list}'))
            silenced_type = self.current_tok.value
            res.register_advancement()
            self.advance()

            if self.current_tok.type != TT_GT:
                return res.failure(InvalidSyntaxError(self.current_tok.pos_start, self.current_tok.pos_end,
                                                      f'Expected \'>\' after \'silent<{silenced_type}\''))
            res.register_advancement()
            self.advance()

            if self.current_tok.type != TT_IDENTIFIER:
                return res.failure(InvalidSyntaxError(self.current_tok.pos_start, self.current_tok.pos_end,
                                                      f'Expected identifier after \'silent<{silenced_type}>\''))
            identifier = self.current_tok
            res.register_advancement()
            self.advance()

            if self.current_tok.type != TT_SET:
                return res.failure(InvalidSyntaxError(self.current_tok.pos_start, self.current_tok.pos_end,
                                                      f'Expected \'::\' after '
                                                      f'\'silent<{silenced_type}> {identifier.value}\''))
            res.register_advancement()
            self.advance()

            pos_start = self.current_tok.pos_start
            suite = res.register(self.statement())
            if res.error: return res

            return res.success(VarAssignNode('silent', identifier, SilentNode(suite, pos_start), False))

        elif self.current_tok.type == TT_IDENTIFIER:
            var_name = self.current_tok
            res.register_advancement()
            self.advance()

            tok_type = self.current_tok.type

            if tok_type in VAR_SET:
                tok = VAR_EQUIV.get(tok_type, tok_type)
                res.register_advancement()
                self.advance()
                expr = res.register(self.expr())
                if res.error: return res
                return res.success(VarReassignNode(var_name, expr, True if tok_type in VAR_SET_RET else False, tok))
            elif tok_type == TT_LPAREN_SQUARE:
                res.register_advancement()
                self.advance()

                def get_list_range():
                    lower = 0
                    higher = 0
                    pos_start = self.current_tok.pos_start
                    if self.current_tok.type == TT_SEMICOLON:
                        res.register_advancement()
                        self.advance()
                        if self.current_tok.type == TT_RPAREN_SQUARE:
                            return VarAccessNode(var_name)
                        higher = res.register(self.expr())
                        if res.error: return res
                    else:
                        if self.current_tok.type == TT_RPAREN_SQUARE:
                            return VarAccessNode(var_name)
                        lower = res.register(self.expr())
                        if res.error: return res
                        if self.current_tok.type == TT_SEMICOLON:
                            res.register_advancement()
                            self.advance()
                            if self.current_tok.type != TT_RPAREN_SQUARE:
                                higher = res.register(self.expr())
                                if res.error: return res
                        elif self.current_tok.type in (TT_RPAREN_SQUARE, TT_COMMA):
                            return VarGetItemNode(lower, 0, False, pos_start, self.current_tok.pos_start)

                    return VarGetItemNode(lower, higher, True, pos_start, self.current_tok.pos_start)

                ref_items = [get_list_range()]
                if isinstance(ref_items[0], VarAccessNode):
                    res.register_advancement()
                    self.advance()
                    return res.success(ref_items[0])
                while self.current_tok.type == TT_COMMA:
                    res.register_advancement()
                    self.advance()
                    ref_items.append(get_list_range())
                if self.current_tok.type != TT_RPAREN_SQUARE:
                    return res.failure(InvalidSyntaxError(var_name.pos_start,
                                                          self.current_tok.pos_end, 'Expected \']\''))
                res.register_advancement()
                self.advance()

                return res.success(VarGetSetNode(var_name, ref_items))

            self.reverse()

        node = res.register(self.bin_op(self.comp_expr, ((TT_KEYWORD, "and"), (TT_KEYWORD, "or"))))

        if self.current_tok.type == TT_QUESTION_MARK:
            res.register_advancement()
            self.advance()
            if_true = res.register(self.expr())
            if res.error: return res
            if self.current_tok.type != TT_SEMICOLON:
                return res.failure(InvalidSyntaxError(self.current_tok.pos_start, self.current_tok.pos_end,
                                                      'Expected \':\''))
            res.register_advancement()
            self.advance()
            if_false = res.register(self.expr())
            if res.error: return res
            return res.success(TrueFalseNode(node, if_true, if_false))

        if res.error:
            return res.failure(InvalidSyntaxError(self.current_tok.pos_start, self.current_tok.pos_end,
                                                  "Expected variable assignment keyword, int, float, identifier, '+',"
                                                  "'-', or '('"))

        return res.success(node)

    def comp_expr(self):
        res = ParseResult()

        if self.current_tok.matches(TT_KEYWORD, 'not'):
            op_tok = self.current_tok
            res.register_advancement()
            self.advance()

            node = res.register(self.comp_expr())
            if res.error: return res
            return res.success(UnaryOpNode(op_tok, node))

        node = res.register(self.bin_op(self.arith_expr, (TT_EQ, TT_NE, TT_LT, TT_GT, TT_LTE, TT_GTE)))

        if res.error:
            return res.failure(InvalidSyntaxError(
                self.current_tok.pos_start, self.current_tok.pos_end,
                "Expected int, float, identifier, '+', '-', '(', '[', 'IF', 'FOR', 'WHILE', 'FUN' or 'NOT'"
            ))

        return res.success(node)

    def arith_expr(self):
        return self.bin_op(self.term, (TT_PLUS, TT_MINUS))

    def term(self):
        return self.bin_op(self.factor, (TT_MULT, TT_DIV, TT_FDIV, TT_MOD))

    def factor(self):
        res = ParseResult()
        tok = self.current_tok
        if tok.type in (TT_PLUS, TT_MINUS, TT_NOT):
            res.register_advancement()
            self.advance()
            factor = res.register(self.factor())
            if res.error: return res
            return res.success(UnaryOpNode(tok, factor))

        return self.power()

    def power(self):
        return self.bin_op(self.call, (TT_POW,), self.factor)

    def call(self):
        res = ParseResult()
        atom = res.register(self.atom())
        if res.error: return res

        if self.current_tok.type == TT_LPAREN:
            res.register_advancement()
            self.advance()
            arg_nodes = []

            if self.current_tok.type == TT_RPAREN:
                res.register_advancement()
                self.advance()
            else:
                arg_nodes.append(res.register(self.expr()))
                if res.error:
                    return res.failure(InvalidSyntaxError(
                        self.current_tok.pos_start, self.current_tok.pos_end,
                        "Expected ')', 'VAR', 'IF', 'FOR', 'WHILE', 'FUN', int, float, identifier, '+', '-', '(', '[' or 'NOT'"
                    ))

                while self.current_tok.type == TT_COMMA:
                    res.register_advancement()
                    self.advance()

                    arg_nodes.append(res.register(self.expr()))
                    if res.error: return res

                if self.current_tok.type != TT_RPAREN:
                    return res.failure(InvalidSyntaxError(
                        self.current_tok.pos_start, self.current_tok.pos_end,
                        f"Expected ',' or ')'"
                    ))

                res.register_advancement()
                self.advance()
            return res.success(CallNode(atom, arg_nodes))
        return res.success(atom)

    def atom(self):
        res = ParseResult()
        tok = self.current_tok

        if tok.type in (TT_INT, TT_FLOAT):
            res.register_advancement()
            self.advance()
            return res.success(NumberNode(tok))

        elif tok.type == TT_STRING:
            res.register_advancement()
            self.advance()
            return res.success(StringNode(tok))

        elif tok.type == TT_IDENTIFIER:
            pos_start = self.current_tok.pos_start
            res.register_advancement()
            self.advance()
            if self.current_tok.type == TT_DOT:
                trace = [VarAccessNode(tok)]
                while self.current_tok.type == TT_DOT:
                    res.register_advancement()
                    self.advance()
                    if not self.current_tok.type == TT_IDENTIFIER:
                        return res.failure(InvalidSyntaxError(self.current_tok.pos_start, self.current_tok.pos_end,
                                                              'Expected a method identifier'))
                    tok = self.current_tok
                    res.register_advancement()
                    self.advance()
                    if self.current_tok.type == TT_LPAREN:
                        res.register_advancement()
                        self.advance()
                        args = []
                        if self.current_tok.type == TT_RPAREN:
                            res.register_advancement()
                            self.advance()
                            trace.append(MethodCallNode(tok, args))
                        else:
                            while True:
                                arg = res.register(self.expr())
                                if res.error: return res
                                args.append(arg)
                                if self.current_tok.type == TT_COMMA:
                                    res.register_advancement()
                                    self.advance()
                                elif self.current_tok.type == TT_RPAREN:
                                    res.register_advancement()
                                    self.advance()
                                    break
                                else:
                                    return res.failure(InvalidSyntaxError(self.current_tok.pos_start,
                                                                          self.current_tok.pos_end,
                                                                          'Expected \',\' or \')\''))
                            trace.append(MethodCallNode(tok, args))
                    else:
                        trace.append(VarAccessNode(tok))
                return res.success(VarSubFuncNode(trace, pos_start, self.current_tok.pos_end))
            return res.success(VarAccessNode(tok))

        elif tok.type == TT_LPAREN:
            res.register_advancement()
            self.advance()
            expr = res.register(self.expr())
            if res.error: return res
            if self.current_tok.type == TT_RPAREN:
                res.register_advancement()
                self.advance()
                return res.success(expr)
            else:
                return res.failure(InvalidSyntaxError(
                    self.current_tok.pos_start, self.current_tok.pos_end,
                    "Expected ')'"
                ))

        elif tok.type == TT_LPAREN_SQUARE:
            list_expr = res.register(self.list_expr())
            if res.error: return res
            return res.success(list_expr)

        elif tok.matches(TT_KEYWORD, 'if'):
            if_expr = res.register(self.if_expr())
            if res.error: return res
            return res.success(if_expr)

        elif tok.type == TT_AT:
            pos_start = tok.pos_start
            res.register_advancement()
            self.advance()
            if not self.current_tok.type == TT_IDENTIFIER:
                return res.failure(InvalidSyntaxError(pos_start, self.current_tok.pos_end,
                                                      'Expected identifier after \'@\''))
            loop_name = self.current_tok.value
            res.register_advancement()
            self.advance()
            if not self.current_tok.matches(TT_KEYWORD, 'iterate'):
                return res.failure(InvalidSyntaxError(pos_start, self.current_tok.pos_end,
                                                      'Expected \'iterate\' loop after \'@\' named identifier'))
            for_expr = res.register(self.iterate_expr())
            if res.error: return res
            for_expr.pos_start = pos_start
            for_expr.reference_name = loop_name
            return res.success(for_expr)

        elif tok.matches(TT_KEYWORD, 'iterate'):
            for_expr = res.register(self.iterate_expr())
            if res.error: return res
            return res.success(for_expr)

        elif tok.matches(TT_KEYWORD, 'while'):
            while_expr = res.register(self.while_expr())
            if res.error: return res
            return res.success(while_expr)

        elif tok.matches(TT_KEYWORD, 'def'):
            func_def = res.register(self.func_def())
            if res.error: return res
            return res.success(func_def)

        elif tok.matches(TT_KEYWORD, 'import'):
            file_name = res.register(self.import_expr())
            if res.error: return res
            return res.success(file_name)

        elif tok.matches(TT_KEYWORD, 'case'):
            cases_expr = res.register(self.cases_expr())
            if res.error: return res
            return res.success(cases_expr)

        return res.failure(InvalidSyntaxError(
            tok.pos_start, tok.pos_end,
            "Expected int, float, identifier, '+', '-', '(', '[', IF', 'FOR', 'WHILE', 'FUN'"
        ))

    def list_expr(self):
        res = ParseResult()
        element_nodes = []
        pos_start = self.current_tok.pos_start.copy()

        if self.current_tok.type != TT_LPAREN_SQUARE:
            return res.failure(InvalidSyntaxError(
                self.current_tok.pos_start, self.current_tok.pos_end,
                f"Expected '['"
            ))

        res.register_advancement()
        self.advance()

        if self.current_tok.type == TT_RPAREN_SQUARE:
            res.register_advancement()
            self.advance()
        else:
            element_nodes.append(res.register(self.expr()))
            if res.error:
                return res.failure(InvalidSyntaxError(
                    self.current_tok.pos_start, self.current_tok.pos_end,
                    "Expected ']', 'VAR', 'IF', 'FOR', 'WHILE', 'FUN', int, float, identifier, '+', '-', '(', '[' or 'NOT'"
                ))

            while self.current_tok.type == TT_COMMA:
                res.register_advancement()
                self.advance()

                element_nodes.append(res.register(self.expr()))
                if res.error: return res

            if self.current_tok.type != TT_RPAREN_SQUARE:
                return res.failure(InvalidSyntaxError(
                    self.current_tok.pos_start, self.current_tok.pos_end,
                    f"Expected ',' or ']'"
                ))

            res.register_advancement()
            self.advance()

        return res.success(ListNode(
            element_nodes,
            pos_start,
            self.current_tok.pos_end.copy()
        ))

    def import_expr(self):
        res = ParseResult()
        res.register_advancement()
        self.advance()
        if self.current_tok.type != TT_LPAREN:
            return res.failure(InvalidSyntaxError(self.current_tok.pos_start, self.current_tok.pos_end,
                                                  "Expected '('"))
        res.register_advancement()
        self.advance()
        name = res.register(self.expr())
        if res.error: return res
        if self.current_tok.type != TT_RPAREN:
            return res.failure(InvalidSyntaxError(self.current_tok.pos_start, self.current_tok.pos_end,
                                                  "Expected ')'"))
        res.register_advancement()
        self.advance()
        return res.success(ImportNode(name))

    def cases_expr(self):
        res = ParseResult()
        start_pos = self.current_tok.pos_start
        res.register_advancement()
        self.advance()
        condition = res.register(self.expr())
        if self.current_tok.type != TT_LPAREN_CURLY:
            return res.failure(InvalidSyntaxError(self.current_tok.pos_start, self.current_tok.pos_end,
                                                  'Expected \'{\''))
        res.register_advancement()
        self.advance()
        cases = []
        default = None
        while True:
            while self.current_tok.type == TT_NEWLINE:
                res.register_advancement()
                self.advance()
            pos_start = self.current_tok.pos_start
            if self.current_tok.matches(TT_KEYWORD, 'option'):
                res.register_advancement()
                self.advance()
                if self.current_tok.type != TT_LPAREN:
                    return res.failure(InvalidSyntaxError(self.current_tok.pos_start, self.current_tok.pos_end,
                                                          'Expected \'(\''))
                res.register_advancement()
                self.advance()
                option = res.register(self.expr())
                if res.error: return res
                if self.current_tok.type != TT_RPAREN:
                    return res.failure(InvalidSyntaxError(self.current_tok.pos_start, self.current_tok.pos_end,
                                                          'Expected \')\''))
                res.register_advancement()
                self.advance()
                if self.current_tok.type != TT_LPAREN_CURLY:
                    return res.failure(InvalidSyntaxError(self.current_tok.pos_start, self.current_tok.pos_end,
                                                          'Expected \'{\''))
                res.register_advancement()
                self.advance()
                expr = res.register(self.statements())
                if res.error: return res
                if self.current_tok.type != TT_RPAREN_CURLY:
                    return res.failure(InvalidSyntaxError(self.current_tok.pos_start, self.current_tok.pos_end,
                                                          'Expected \'}\''))
                cases.append(OptionNode(option, expr, pos_start, self.current_tok.pos_end))
                res.register_advancement()
                self.advance()
                continue
            elif self.current_tok.matches(TT_KEYWORD, 'default'):
                res.register_advancement()
                self.advance()
                if self.current_tok.type != TT_LPAREN_CURLY:
                    return res.failure(InvalidSyntaxError(self.current_tok.pos_start, self.current_tok.pos_end,
                                                          'Expected \'{\''))
                res.register_advancement()
                self.advance()
                expr = res.register(self.statements())
                if res.error: return res
                if self.current_tok.type != TT_RPAREN_CURLY:
                    return res.failure(InvalidSyntaxError(self.current_tok.pos_start, self.current_tok.pos_end,
                                                          'Expected \'}\''))
                default = OptionNode(None, expr, pos_start, self.current_tok.pos_end)
                res.register_advancement()
                self.advance()
                continue
            elif self.current_tok.type == TT_RPAREN_CURLY:
                end_pos = self.current_tok.pos_end
                res.register_advancement()
                self.advance()
                break
            else:
                return res.failure(InvalidSyntaxError(self.current_tok.pos_start, self.current_tok.pos_end,
                                                      'Expected \'option\', \'default\', \'}\', or option identifier'))

        return res.success(CaseNode(condition, cases, start_pos, end_pos, default))

    def if_expr(self):
        res = ParseResult()
        all_cases = res.register(self.if_expr_cases("if"))
        if res.error: return res
        cases, else_case = all_cases
        return res.success(IfNode(cases, else_case))

    def if_expr_elif(self):
        return self.if_expr_cases("elif")

    def if_expr_else(self):
        res = ParseResult()
        else_case = None

        if self.current_tok.matches(TT_KEYWORD, "else"):
            res.register_advancement()
            self.advance()

            if self.current_tok.type != TT_LPAREN_CURLY:
                return res.failure(InvalidSyntaxError(self.current_tok.pos_start, self.current_tok.pos_end,
                                                      "Expected '{'"))

            res.register_advancement()
            self.advance()

            statements = res.register(self.statement())
            if res.error: return res
            else_case = (statements, True)

            if self.current_tok.type != TT_RPAREN_CURLY:
                return res.failure(InvalidSyntaxError(self.current_tok.pos_start, self.current_tok.pos_end,
                                                      "Expected '}'"))

            res.register_advancement()
            self.advance()

        return res.success(else_case)

    def if_expr_elif_or_else(self):
        res = ParseResult()
        cases, else_cases = [], None

        if self.current_tok.matches(TT_KEYWORD, "elif"):
            all_cases = res.register(self.if_expr_elif())
            if res.error: return res
            cases, else_case = all_cases
        else:
            else_case = res.register(self.if_expr_else())
            if res.error: return res

        return res.success((cases, else_case))

    def if_expr_cases(self, case_keyword):
        res = ParseResult()
        cases = []

        res.register_advancement()
        self.advance()

        condition = res.register(self.expr())
        if res.error: return res

        if self.current_tok.type != TT_LPAREN_CURLY:
            return res.failure(InvalidSyntaxError(self.current_tok.pos_start, self.current_tok.pos_end,
                                                  "Expected '{'"))

        res.register_advancement()
        self.advance()

        statements = res.register(self.statements())
        if res.error: return res
        cases.append((condition, statements, True))

        if self.current_tok.type != TT_RPAREN_CURLY:
            return res.failure(InvalidSyntaxError(self.current_tok.pos_start, self.current_tok.pos_end,
                                                  "Expected '}'"))
        res.register_advancement()
        self.advance()
        all_cases = res.register(self.if_expr_elif_or_else())
        if res.error: return res
        new_cases, else_case = all_cases
        cases.extend(new_cases)

        return res.success((cases, else_case))

    def iterate_expr(self):
        res = ParseResult()
        starting_value = None

        res.register_advancement()
        self.advance()

        ending_value = res.register(self.statement())
        if res.error: return res

        if self.current_tok.matches(TT_KEYWORD, "to"):
            starting_value = ending_value

            res.register_advancement()
            self.advance()

            ending_value = res.register(self.expr())
            if res.error: return res

        if self.current_tok.matches(TT_KEYWORD, "step"):
            res.register_advancement()
            self.advance()

            step = res.register(self.expr())
            if res.error: return res
        else:
            step = None

        if self.current_tok.type == TT_SET:
            res.register_advancement()
            self.advance()
            if not self.current_tok.type == TT_IDENTIFIER:
                return res.failure(InvalidSyntaxError(self.current_tok.pos_start, self.current_tok.pos_end,
                                                      "Expected identifier"))

            iterable_var = self.current_tok

            res.register_advancement()
            self.advance()
        else:
            iterable_var = None

        if not self.current_tok.type == TT_LPAREN_CURLY:
            return res.failure(InvalidSyntaxError(self.current_tok.pos_start, self.current_tok.pos_end,
                                                  "Expected '{'"))

        res.register_advancement()
        self.advance()

        suite = res.register(self.statements())
        if res.error: return res

        if self.current_tok.type != TT_RPAREN_CURLY:
            print(self.current_tok)
            return res.failure(InvalidSyntaxError(self.current_tok.pos_start, self.current_tok.pos_end,
                                                  "Expected '}'"))

        res.register_advancement()
        self.advance()

        return res.success(IterateNode(iterable_var, starting_value, ending_value, step, suite, True))

    def while_expr(self):
        res = ParseResult()

        res.register_advancement()
        self.advance()

        condition = res.register(self.statement())
        if res.error: return res

        if not self.current_tok.type == TT_LPAREN_CURLY:
            return res.failure(InvalidSyntaxError(self.current_tok.pos_start, self.current_tok.pos_end,
                                                  "Expected '{'"))

        res.register_advancement()
        self.advance()

        suite = res.register(self.statements())
        if res.error: return res

        if self.current_tok.type != TT_RPAREN_CURLY:
            return res.failure(InvalidSyntaxError(self.current_tok.pos_start, self.current_tok.pos_end,
                                                  "Expected '}'"))

        res.register_advancement()
        self.advance()

        return res.success(WhileNode(condition, suite, True))

    def func_def(self):
        res = ParseResult()
        res.register_advancement()
        self.advance()

        if self.current_tok.type == TT_IDENTIFIER:
            var_name_tok = self.current_tok
            res.register_advancement()
            self.advance()
            if self.current_tok.type != TT_LPAREN:
                return res.failure(InvalidSyntaxError(self.current_tok.pos_start, self.current_tok.pos_end,
                                                      "Expected '('"))
        else:
            var_name_tok = None
            if self.current_tok.type != TT_LPAREN:
                return res.failure(InvalidSyntaxError(self.current_tok.pos_start, self.current_tok.pos_end,
                                                      "Expected identifier or '('"))

        res.register_advancement()
        self.advance()

        arg_name_toks = []

        if self.current_tok.type == TT_IDENTIFIER:
            arg_name_toks.append(self.current_tok)
            res.register_advancement()
            self.advance()

            while self.current_tok.type == TT_COMMA:
                res.register_advancement()
                self.advance()

                if self.current_tok.type != TT_IDENTIFIER:
                    return res.failure(InvalidSyntaxError(self.current_tok.pos_start, self.current_tok.pos_end,
                                                          "Expected identifier"))

                arg_name_toks.append(self.current_tok)
                res.register_advancement()
                self.advance()

            if self.current_tok.type != TT_RPAREN:
                return res.failure(InvalidSyntaxError(self.current_tok.pos_start, self.current_tok.pos_end,
                                                      "Expected ',' or ')'"))

            res.register_advancement()
            self.advance()

            if self.current_tok.type != TT_LPAREN_CURLY:
                return res.failure(InvalidSyntaxError(self.current_tok.pos_start, self.current_tok.pos_end,
                                                      "Expected '{'"))

            res.register_advancement()
            self.advance()

            suite = res.register(self.statements())
            if res.error: return res

            if self.current_tok.type != TT_RPAREN_CURLY:
                return res.failure(InvalidSyntaxError(self.current_tok.pos_start, self.current_tok.pos_end,
                                                      "Expected '}'"))

            res.register_advancement()
            self.advance()

            return res.success(FuncDefNode(var_name_tok, arg_name_toks, suite))

    def bin_op(self, func_a, ops, func_b=None):
        if func_b is None:
            func_b = func_a

        res = ParseResult()
        left = res.register(func_a())
        if res.error: return res

        while self.current_tok.type in ops or (self.current_tok.type, self.current_tok.value) in ops:
            op_tok = self.current_tok
            res.register_advancement()
            self.advance()
            right = res.register(func_b())
            if res.error: return res
            left = BinOpNode(left, op_tok, right)

        return res.success(left)


#######################################
# INTERPRETER
#######################################

class Interpreter:
    def __init__(self):
        self.BinOpFuncNames = {TT_PLUS: 'added_to', TT_MINUS: 'subbed_by', TT_MULT: 'multed_by', TT_DIV: 'dived_by',
                               TT_POW: 'powed_by', TT_EQ: 'get_comparison_eq', TT_NE: 'get_comparison_ne',
                               TT_LT: 'get_comparison_lt', TT_GT: 'get_comparison_gt', TT_LTE: 'get_comparison_lte',
                               TT_GTE: 'get_comparison_gte', TT_MOD: 'modded_by', TT_FDIV: 'fdived_by'}
        self.KeywordFunctionNames = {'and': 'anded_by', 'or': 'ored_by'}

    def visit(self, node, context):
        method_name = f'visit_{type(node).__name__}'
        method = getattr(self, method_name, self.no_visit_method)
        return method(node, context)

    def no_visit_method(self, node, context):
        raise Exception(f'No visit_{type(node).__name__} method defined')

    ###################################

    def visit_NumberNode(self, node: NumberNode, context):
        ret_class = {'INT': Int, 'FLOAT': Float}.get(node.tok.type)
        return RTResult().success(
            ret_class(node.tok.value).set_context(context).set_pos(node.pos_start, node.pos_end)
        )

    def visit_StringNode(self, node: StringNode, context):
        return RTResult().success(
            String(node.tok.value).set_context(context).set_pos(node.pos_start, node.pos_end)
        )

    def visit_ImportNode(self, node: ImportNode, context):
        res = RTResult()
        name = res.register(self.visit(node.file_name, context))
        if res.should_return(): return res
        if not isinstance(name, String):
            return res.failure(InvalidSyntaxError(node.pos_start, node.pos_end, 'Expected string'))
        result = res.register(self.visit(CallNode(BuiltInFunction('run'), name), context))
        print(result)

    def visit_ListNode(self, node: ListNode, context):
        res = RTResult()
        elements = []

        for element_node in node.element_nodes:
            elements.append(res.register(self.visit(element_node, context)))
            if res.should_return(): return res

        return res.success(
            List(elements).set_context(context).set_pos(node.pos_start, node.pos_end)
        )

    def visit_TrueFalseNode(self, node: TrueFalseNode, context):
        res = RTResult()
        condition = res.register(self.visit(node.condition, context)).is_true()
        if res.should_return(): return res
        if condition:
            value = res.register(self.visit(node.if_true, context))
        else:
            value = res.register(self.visit(node.if_false, context))
        if res.should_return(): return res
        return res.success(value)

    def visit_VarAccessNode(self, node: VarAccessNode, context):
        res = RTResult()
        var_name = node.name_tok.value
        value = context.symbol_table.get(var_name)

        if not value:
            return res.failure(RTError(
                node.pos_start, node.pos_end,
                f"'{var_name}' is not defined",
                context
            ))

        value = value.copy().set_pos(node.pos_start, node.pos_end).set_context(context)
        return res.success(value)

    def visit_VarSubFuncNode(self, node: VarSubFuncNode, context):
        def attribute_check(parent_, method_):
            try:
                return False, getattr(parent_, method_)
            except AttributeError:
                return True, None

        res = RTResult()
        if isinstance(node.trace[0], VarAccessNode):
            parent = context.symbol_table.get(node.trace[0].name_tok.value)
        else:
            return res.failure(InvalidSyntaxError(node.trace[0].pos_start, node.trace[0].pos_end, 'Not yet implimentd'))
        traceback = node.trace[0].name_tok.value
        for method in node.trace[1:]:
            if isinstance(method, MethodCallNode):
                err, attr = attribute_check(parent.silenced_node, method.name_tok.value)
                if err:
                    return res.failure(UnknownAttributeError(node.pos_start, node.pos_end,
                                                             method.name_tok.value, traceback))
                args = attr.args
                if len(method.args) != len(args):
                    return res.failure(InvalidSyntaxError(node.pos_start, node.pos_end,
                                                          f'{abs(len(method.args) - len(args))} too '
                                                          f'{"few" if len(method.args) < len(args) else "many"}'
                                                          f' were passed into \'{method.name_tok.value}\''))
                if method.name_tok.value == 'execute' and isinstance(parent.silenced_node, CaseNode):
                    parent = res.register(self.visit_CaseNode(parent.silenced_node, context))
                else:
                    for arg in args:
                        args[arg] = method.args[0]
                        del method.args[0]
                    parent = attr(*args.values(), context=context)
            traceback += f'.{method.name_tok.value}'
        return res.success(parent if isinstance(parent, Value) else None)

    def visit_VarGetSetNode(self, node: VarGetSetNode, context):
        res = RTResult()
        var_name = node.var_name_tok.value
        value = context.symbol_table.get(var_name)
        if not value:
            return res.failure(RTError(
                node.pos_start, node.pos_end,
                f"'{var_name}' is not defined",
                context
            ))

        for range_get_node in node.rangeList:
            if not isinstance(value, List):
                return res.failure(RTError(
                    range_get_node.pos_start, range_get_node.pos_end,
                    f"'{value}' is not a list and so cannot use the [] operators to access members of itself",
                    context
                ))
            value_len = len(value.elements)
            value = res.register(self.visit_VarGetItemNode(range_get_node, context, value, value_len))
            if res.should_return(): return res

        return res.success(value)

    def visit_VarGetItemNode(self, node: VarGetItemNode, context, value, list_len):
        res = RTResult()
        range_get = node.range_get

        if node.lower == 0:
            lower = 0
        else:
            lower = res.register(self.visit(node.lower, context))
            if res.should_return(): return res
            if isinstance(lower, Number):
                if isinstance(lower.value, float):
                    NonBreakError(lower.pos_start, lower.pos_end, context, WT_ListIndexFloat).print_method()
                lower = int(lower.value)
            else:
                return res.failure(IllegalValueError(lower.pos_start,
                                                     lower.pos_end, "List index must be a single value"))
        if node.higher == 0:
            higher = list_len - 1
        else:
            higher = res.register(self.visit(node.higher, context))
            if res.should_return(): return res
            if isinstance(higher, Number):
                if isinstance(higher.value, float):
                    NonBreakError(higher.pos_start, higher.pos_end, context, WT_ListIndexFloat).print_method()
                higher = int(higher.value)
            else:
                return res.failure(
                    IllegalValueError(higher.pos_start, higher.pos_end, "List index must be a single value"))

        lower = list_len + lower if lower < 0 else lower
        higher = list_len + higher if higher < 0 else higher

        if lower > list_len:
            NonBreakError(node.pos_start, node.pos_end, context, WT_ListIndexOutOfRange).print_method()
            lower = list_len - 1
        if higher > list_len:
            NonBreakError(node.pos_start, node.pos_end, context, WT_ListIndexOutOfRange).print_method()
            higher = list_len - 1

        reverse = False
        if higher < lower and node.range_get:
            temp = lower
            lower = higher
            higher = temp
            reverse = True

        if range_get:
            value = List(value.elements[lower:higher + 1]).set_pos(node.pos_start, node.pos_end).set_context(context)
            if reverse:
                value.elements.reverse()
        else:
            value = value.elements[lower].set_pos(node.pos_start, node.pos_end).set_context(context)
        return res.success(value)

    def assignChecks(self, var_name, value, previous, node, context):
        res = RTResult()
        if isinstance(previous, Infinity):
            previous = previous.previous_type
        if previous != List:
            if isinstance(value, List):
                return res.failure(RTError(node.pos_start, node.pos_end, f'Cannot assign a {node.var_type} variable to '
                                                                         f'a list', context))
            if not isinstance(value, Infinity):
                value = previous(value.value)
        elif isinstance(value, Number):
            NonBreakError(node.pos_start, node.pos_end, context, WT_ListFromValue).print_method()
            value = List([value.value])

        if res.should_return(): return res

        context.symbol_table.set(var_name, value)
        return res.success(value if node.ret else None)

    def visit_VarAssignNode(self, node: VarAssignNode, context):
        res = RTResult()
        if node.var_type == 'silent':
            context.symbol_table.set(node.var_name_tok.value, node.value_node)
            return res.success(None)
        else:
            class_type = {'int': Int, 'float': Float, 'bool': Bool, 'list': List}.get(node.var_type)
            var_name = node.var_name_tok.value
            if node.value_node:
                value = res.register(self.visit(node.value_node, context))
            else:
                value = {'int': Int(0), 'float': Float(0), 'bool': Bool(False), 'list': List([])}.get(node.var_type)
            return self.assignChecks(var_name, value, class_type, node, context)

    def visit_VarReassignNode(self, node: VarReassignNode, context):
        res = RTResult()
        var_name = node.var_name_tok
        previous = context.symbol_table.get(var_name.value).set_pos(node.pos_start, node.pos_end)
        if previous is None:
            return res.failure(
                RTError(node.pos_start, node.pos_end, f"Variable {var_name.value} is undefined", context))
        if res.should_return(): return res
        token = node.token
        value = res.register(self.visit(node.value_node, context))

        if token == TT_SET:
            previous = type(previous)
            return self.assignChecks(var_name.value, value, previous, node, context)

        methods = {TT_SET_PLUS: previous.added_to, TT_SET_MINUS: previous.subbed_by, TT_SET_MULT: previous.multed_by,
                   TT_SET_DIV: previous.dived_by, TT_SET_FDIV: previous.fdived_by, TT_SET_MOD: previous.modded_by,
                   TT_SET_POW: previous.powed_by}

        value, error = methods.get(token)(value)
        if error: return None, error
        return self.assignChecks(var_name.value, value, type(previous), node, context)

    def visit_BinOpNode(self, node: BinOpNode, context):
        res = RTResult()
        left = res.register(self.visit(node.left_node, context))
        if res.should_return(): return res
        right = res.register(self.visit(node.right_node, context))
        if res.should_return(): return res

        found = False
        for i, n in self.BinOpFuncNames.items():
            if node.op_tok.type == i:
                result, error = getattr(left, n)(right)
                found = True
                break
        if not found:
            for i, n in self.KeywordFunctionNames.items():
                if node.op_tok.matches(TT_KEYWORD, i):
                    result, error = getattr(left, n)(right)
                    found = True
                    break
        if not found:
            return res.failure(InvalidSyntaxError(node.pos_start, node.pos_end, f'No method defined for {node.op_tok}'))

        if error:
            return res.failure(error)
        return res.success(result.set_pos(node.pos_start, node.pos_end))

    def visit_UnaryOpNode(self, node: UnaryOpNode, context):
        res = RTResult()
        number = res.register(self.visit(node.node, context))
        if res.should_return(): return res

        error = None

        if node.op_tok.type == TT_MINUS:
            number, error = number.multed_by(Number(-1))
        elif node.op_tok.type == TT_NOT:
            number, error = number.notted()

        if error:
            return res.failure(error)
        else:
            return res.success(number.set_pos(node.pos_start, node.pos_end))

    def visit_IfNode(self, node: IfNode, context):
        res = RTResult()

        for condition, expr, should_return_null in node.cases:
            condition_value = res.register(self.visit(condition, context))
            if res.should_return(): return res

            if condition_value.is_true():
                expr_value = res.register(self.visit(expr, context))
                if res.should_return(): return res
                return res.success(None if should_return_null else expr_value)

        if node.else_case:
            expr, should_return_null = node.else_case
            expr_value = res.register(self.visit(expr, context))
            if res.should_return(): return res
            return res.success(None if should_return_null else expr_value)

        return res.success(None)

    def visit_CaseNode(self, node: CaseNode, context):
        res = RTResult()
        value = res.register(self.visit(node.condition, context))
        if isinstance(value, Number) or isinstance(value, String):
            value = value.value
        elif isinstance(value, List):
            value = value.elements
        if res.should_return(): return res
        for i in node.cases:
            value_to_check = res.register(self.visit(i.option, context))
            if res.should_return(): return res
            if isinstance(value_to_check, Number) or isinstance(value_to_check, String):
                value_to_check = value_to_check.value
            elif isinstance(value_to_check, List):
                value_to_check = value_to_check.elements
            if value_to_check == value:
                return_value = res.register(self.visit(i.expr, context))
                if res.should_return(): return res
                return res.success(return_value)
        if node.default:
            return_value = res.register(self.visit(node.default.expr, context))
            if res.should_return(): return res
            return res.success(return_value)
        return res.success(None)

    def visit_IterateNode(self, node: IterateNode, context):
        res = RTResult()
        elements = []
        should_return_null = node.should_return_null
        if node.reference_name:
            context.named_loops.append(node.reference_name)

        if node.start_value_node:
            start_value = res.register(self.visit(node.start_value_node, context))
            if res.should_return(): return res
        else:
            start_value = Number(0)

        end_value = res.register(self.visit(node.end_value_node, context))
        if res.should_return(): return res

        end_value = end_value.value
        start_value = start_value.value
        i = start_value

        if node.step_value_node:
            step_value = res.register(self.visit(node.step_value_node, context))
            if res.should_return(): return res
            step_value = step_value.value
        else:
            step_value = 1 if start_value < end_value else -1

        if step_value > 0:
            if start_value > end_value:
                step_value *= -1
                NonBreakError((node.start_value_node if node.step_value_node else node.end_value_node).pos_start
                              , node.step_value_node.pos_end, context, WT_IterateStepLoop).print_method()
            condition = lambda: i < end_value
        elif step_value < 0:
            if start_value < end_value:
                step_value *= -1
                NonBreakError((node.start_value_node if node.step_value_node else node.end_value_node).pos_start
                              , node.step_value_node.pos_end, context, WT_IterateStepLoop).print_method()
            condition = lambda: i > end_value
        else:
            NonBreakError(node.step_value_node.pos_start, node.step_value_node.pos_end,
                          context, WT_IterateStepZero).print_method()
            step_value = 1 if start_value < end_value else -1
            if start_value < end_value:
                condition = lambda: i < end_value
            else:
                condition = lambda: i > end_value

        # iterable = [start_value + offset * step_value for offset in
        #             list(range(int(end_value - step_value) // step_value))]

        while condition():
            if node.var_name_tok:
                context.symbol_table.set(node.var_name_tok.value, Number(i))

            value = res.register(self.visit(node.suite_node, context))
            if res.loop_should_break:
                if res.loop_name != node.reference_name:
                    return res
            if res.should_return() and (not res.loop_should_continue) and (not res.loop_should_break): return res

            if node.var_name_tok:
                i = context.symbol_table.get(node.var_name_tok.value).value
            i += step_value

            if res.loop_should_continue:
                continue

            if res.loop_should_break:
                break

            elements.append(value)

        if node.var_name_tok:
            context.symbol_table.remove(node.var_name_tok.value)

        if node.reference_name:
            del context.named_loops[-1]

        return res.success(None if should_return_null else
                           List(elements).set_context(context).set_pos(node.pos_start, node.pos_end))

    def visit_WhileNode(self, node: WhileNode, context):
        res = RTResult()
        elements = []

        while True:
            condition = res.register(self.visit(node.condition_node, context))
            if res.should_return(): return res

            if not condition.is_true():
                break

            value = res.register(self.visit(node.body_node, context))
            if res.should_return() and res.loop_should_continue is False and res.loop_should_break is False: return res

            if res.loop_should_continue:
                continue

            if res.loop_should_break:
                break

            elements.append(value)

        return res.success(
            None if node.should_return_null else
            List(elements).set_context(context).set_pos(node.pos_start, node.pos_end)
        )

    def visit_FuncDefNode(self, node: FuncDefNode, context):
        res = RTResult()

        func_name = node.var_name_tok.value if node.var_name_tok else None
        body_node = node.body_node
        arg_names = [arg_name.value for arg_name in node.arg_name_toks]
        func_value = Function(func_name, body_node, arg_names).set_context(context).set_pos(
            node.pos_start, node.pos_end)

        if node.var_name_tok:
            context.symbol_table.set(func_name, func_value)

        return res.success(func_value)

    def visit_CallNode(self, node: CallNode, context):
        res = RTResult()
        args = []

        value_to_call = res.register(self.visit(node.node_to_call, context))
        if res.should_return(): return res
        value_to_call = value_to_call.copy().set_pos(node.pos_start, node.pos_end)

        for arg_node in node.arg_nodes:
            args.append(res.register(self.visit(arg_node, context)))
            if res.should_return(): return res

        return_value = res.register(value_to_call.execute(args))
        if res.should_return(): return res
        if not isinstance(return_value, Null) and return_value:
            return_value = return_value.copy().set_pos(node.pos_start, node.pos_end).set_context(context)
        return res.success(return_value)

    def visit_ReturnNode(self, node: ReturnNode, context):
        res = RTResult()

        if node.node_to_return:
            value = res.register(self.visit(node.node_to_return, context))
            if res.should_return(): return res
        else:
            value = Null()

        return res.success_return(value)

    def visit_ContinueNode(self, node: ContinueNode, context):
        return RTResult().success_continue()

    def visit_BreakNode(self, node: BreakNode, context):
        if node.break_to:
            if node.break_to not in context.named_loops:
                return RTResult().failure(InvalidSyntaxError(node.pos_start, node.pos_end,
                                                             f'Loop \'@{node.break_to}\' is undefined in'
                                                             f' the current scope'))
        return RTResult().success_break(node.break_to)


#######################################
# GLOBAL VARIABLES
#######################################

global_symbol_table = SymbolTable()
global_symbol_table.set("true", Bool(1))
global_symbol_table.set("false", Bool(0))
global_symbol_table.set("null", Null())
built_in_funcs = ['log', 'print', 'type', 'input', 'clear', 'run']
for i in built_in_funcs:
    global_symbol_table.set(i, BuiltInFunction(i))


#######################################
# RUN
#######################################

class RunRes:
    def __init__(self, result, error, previous: list, newLineNeeded: bool):
        self.result = result
        self.error = error
        self.previous = previous
        self.newLineNeeded = newLineNeeded


def run(fn, text, previous=None) -> RunRes:
    out = RunRes(None, None, [], False)

    # Generate tokens
    lexer = Lexer(fn, text)
    tokens, error = lexer.make_tokens()
    if error:
        out.error = error
        return out

    if previous:
        tokens = previous.previous + tokens

    paren = sum([1 if i.type == TT_LPAREN else 0 for i in tokens]) - sum(
        [1 if i.type == TT_RPAREN else 0 for i in tokens])
    if paren != 0:
        out.previous = tokens[:-1]  # [:-1] needed to remove the 'EOF' token at the end of the generated tokens
        out.newLineNeeded = True
        return out
    paren = sum([1 if i.type == TT_LPAREN_SQUARE else 0 for i in tokens]) - sum(
        [1 if i.type == TT_RPAREN_SQUARE else 0 for i in tokens])
    if paren != 0:
        out.previous = tokens[:-1]
        out.newLineNeeded = True
        return out
    paren = sum([1 if i.type == TT_LPAREN_CURLY else 0 for i in tokens]) - sum(
        [1 if i.type == TT_RPAREN_CURLY else 0 for i in tokens])
    if paren != 0:
        out.previous = tokens[:-1]
        out.newLineNeeded = True
        return out

    del paren

    # Generate AST
    parser = Parser(tokens)
    ast = parser.parse()
    if ast.error:
        out.error = ast.error
        return out

    # Run program
    interpreter = Interpreter()
    context = Context('<program>')
    context.symbol_table = global_symbol_table
    result = interpreter.visit(ast.node, context)
    out.result, out.error = result.value, result.error

    return out


#######################################
# CLASS REDEFINITIONS
#######################################

def execute(self, args):
    res = RTResult()
    interpreter = Interpreter()
    exec_ctx = self.generate_new_context()

    res.register(self.check_and_populate_args(self.arg_names, args, exec_ctx))
    if res.should_return(): return res

    value = res.register(interpreter.visit(self.body_node, exec_ctx))
    if res.should_return() and res.func_return_value is None: return res

    ret_value = value or res.func_return_value or None
    return res.success(ret_value)


Function.execute = execute


def execute_run(self, exec_ctx):
    fn = exec_ctx.symbol_table.get("fn")

    if not isinstance(fn, String):
        return RTResult().failure(RTError(self.pos_start, self.pos_end, "Filename must be a string", exec_ctx))

    fn = fn.value

    try:
        with open(fn, 'r') as f:
            script = split('[\n;]', f.read())
    except Exception as e:
        return RTResult().failure(RTError(self.pos_start, self.pos_end, f'Failed to load script \"{fn}\"\n'
                                          + str(e), exec_ctx))

    res = None
    for line in script:
        res = run(fn, line, res)

        if res.error:
            return RTResult().failure(RTError(self.pos_start, self.pos_end, f"An error was returned while running"
                                                                            f" \"{fn}\"\n{res.error.as_string()}",
                                              exec_ctx))

        if res.newLineNeeded:
            continue
        res = None

    return RTResult().success(None)


execute_run.arg_names = ["fn"]

BuiltInFunction.execute_run = execute_run
