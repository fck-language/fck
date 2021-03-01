from Nodes import *
from Errors import *


class ParseRes:
    def __init__(self):
        self.error = None
        self.node = None
        self.advance_count = 0

    def register_advancement(self):
        self.advance_count += 1

    def register(self, res):
        self.advance_count += res.advance_count
        if res.error: self.error = res.error
        return res.node

    def success(self, node):
        self.node = node
        return self

    def failure(self, error):
        if not self.error or self.advance_count == 0:
            self.error = error
        return self


class Parser:
    def __init__(self, tokens: list):
        self.tokens = tokens
        self.tok_idx = -1
        self.advance()

    def advance(self):
        self.tok_idx += 1
        if self.tok_idx < len(self.tokens):
            self.current_tok = self.tokens[self.tok_idx]
        return self.current_tok

    def parse(self):
        res = self.expr()
        if not res.error and self.current_tok.type != TT_EOF:
            return res.failure(InvalidSyntaxError(self.current_tok.pos_start, self.current_tok.pos_end,
                                                  "Expected '+', '-', '*', '/', or '**'"))
        return res

    def iterate_expr(self):
        res = ParseRes()
        starting_value = None

        if not self.current_tok.matches(TT_KEYWORD, "iterate"):
            return res.failure(InvalidSyntaxError(self.current_tok.pos_start, self.current_tok.pos_end,
                                                  "Expected 'iterate'"))

        res.register_advancement()
        self.advance()

        ending_value = res.register(self.expr())
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

        suite = res.register(self.expr())
        if res.error: return res

        if not self.current_tok.type == TT_RPAREN_CURLY:
            return res.failure(InvalidSyntaxError(self.current_tok.pos_start, self.current_tok.pos_end,
                                                  "Expected '}'"))

        res.register_advancement()
        self.advance()

        return res.success(ForNode(iterable_var, starting_value, ending_value, step, suite))

    def while_expr(self):
        res = ParseRes()

        if not self.current_tok.matches(TT_KEYWORD, "while"):
            return res.failure(InvalidSyntaxError(self.current_tok.pos_start, self.current_tok.pos_end,
                                                  "Expected 'while'"))

        res.register_advancement()
        self.advance()

        condition = res.register(self.expr())
        if res.error: return res

        if not self.current_tok.type == TT_LPAREN_CURLY:
            return res.failure(InvalidSyntaxError(self.current_tok.pos_start, self.current_tok.pos_end,
                                                  "Expected '{'"))

        res.register_advancement()
        self.advance()

        suite = res.register(self.expr())
        if res.error: return res

        if not self.current_tok.type == TT_RPAREN_CURLY:
            return res.failure(InvalidSyntaxError(self.current_tok.pos_start, self.current_tok.pos_end,
                                                  "Expected '}'"))

        res.register_advancement()
        self.advance()

        return res.success(WhileNode(condition, suite))

    def if_expr(self):
        res = ParseRes()
        cases = []
        else_case = None

        if not self.current_tok.matches(TT_KEYWORD, 'if'):
            return res.failure(InvalidSyntaxError(self.current_tok.pos_start, self.current_tok.pos_end,
                                                  "Expected 'if'"))

        res.register_advancement()
        self.advance()

        condition = res.register(self.expr())
        if res.error: return res

        if not self.current_tok.type == TT_LPAREN_CURLY:
            return res.failure(InvalidSyntaxError(self.current_tok.pos_start, self.current_tok.pos_end,
                                                  "Expected '{'"))

        res.register_advancement()
        self.advance()

        expr = res.register(self.expr())
        if res.error: return res

        if not self.current_tok.type == TT_RPAREN_CURLY:
            return res.failure(InvalidSyntaxError(self.current_tok.pos_start, self.current_tok.pos_end,
                                                  "Expected '}'"))

        res.register_advancement()
        self.advance()

        cases.append((condition, expr))

        while self.current_tok.matches(TT_KEYWORD, 'elif'):
            res.register_advancement()
            self.advance()

            condition = res.register(self.expr())
            if res.error: return res

            if not self.current_tok.type == TT_LPAREN_CURLY:
                return res.failure(InvalidSyntaxError(self.current_tok.pos_start, self.current_tok.pos_end,
                                                      "Expected '{'"))

            res.register_advancement()
            self.advance()

            expr = res.register(self.expr())
            if res.error: return res

            if not self.current_tok.type == TT_RPAREN_CURLY:
                return res.failure(InvalidSyntaxError(self.current_tok.pos_start, self.current_tok.pos_end,
                                                      "Expected '}'"))

            res.register_advancement()
            self.advance()

            cases.append((condition, expr))

        if self.current_tok.matches(TT_KEYWORD, 'else'):
            res.register_advancement()
            self.advance()

            if not self.current_tok.type == TT_LPAREN_CURLY:
                return res.failure(InvalidSyntaxError(self.current_tok.pos_start, self.current_tok.pos_end,
                                                      "Expected '{'"))

            res.register_advancement()
            self.advance()

            expr = res.register(self.expr())
            if res.error: return res

            if not self.current_tok.type == TT_RPAREN_CURLY:
                return res.failure(InvalidSyntaxError(self.current_tok.pos_start, self.current_tok.pos_end,
                                                      "Expected '}'"))

            res.register_advancement()
            self.advance()

            else_case = expr

        return res.success(IfNode(cases, else_case))

    def call(self):
        res = ParseRes()
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
                    return res.failure(InvalidSyntaxError(self.current_tok.pos_start, self.current_tok.pos_end,
                                                          "Expected ')', method , int, float, identifier, "
                                                          "'+', or '-'"))
                while self.current_tok.type == TT_COMMA:
                    res.register_advancement()
                    self.advance()

                    arg_nodes.append(res.register(self.expr()))
                    if res.error: return res

                if self.current_tok.type != TT_RPAREN:
                    return res.failure(InvalidSyntaxError(self.current_tok.pos_start, self.current_tok.pos_end,
                                                          "Expected ',' or ')"))

                res.register_advancement()
                self.advance()

            return res.success(CallNode(atom, arg_nodes))
        return res.success(atom)

    def atom(self):
        res = ParseRes()
        tok = self.current_tok

        if tok.type in (TT_INT, TT_FLOAT):
            res.register_advancement()
            self.advance()
            return res.success(NumberNode(tok))

        if tok.type == TT_STRING:
            res.register_advancement()
            self.advance()
            return res.success(StringNode(tok))

        elif tok.type == TT_IDENTIFIER:
            res.register_advancement()
            self.advance()
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
                return res.failure(InvalidSyntaxError(self.current_tok.pos_start, self.current_tok.pos_end,
                                                      "Expected ')'"))

        elif tok.type == TT_LPAREN_SQUARE:
            list_expr = res.register(self.list_expr())
            if res.error: return res
            return res.success(list_expr)

        elif tok.matches(TT_KEYWORD, "if"):
            if_expr = res.register(self.if_expr())
            if res.error: return res
            return res.success(if_expr)

        elif tok.matches(TT_KEYWORD, "iterate"):
            iterate_expr = res.register(self.iterate_expr())
            if res.error: return res
            return res.success(iterate_expr)

        elif tok.matches(TT_KEYWORD, "while"):
            while_expr = res.register(self.while_expr())
            if res.error: return res
            return res.success(while_expr)

        elif tok.matches(TT_KEYWORD, "def"):
            func_def = res.register(self.func_def())
            if res.error: return res
            return res.success(func_def)

        return res.failure(InvalidSyntaxError(
            tok.pos_start, tok.pos_end, "Expected int, float, identifier, '+', '-', or '('"
        ))

    def power(self):
        return self.bin_op(self.call, (TT_POW,), self.factor)

    def factor(self):
        res = ParseRes()
        tok = self.current_tok

        if tok.type in (TT_PLUS, TT_MINUS):
            res.register_advancement()
            self.advance()
            factor = res.register(self.factor())
            if res.error: return res
            return res.success(UnaryOpNode(tok, factor))

        return self.power()

    def term(self):
        return self.bin_op(self.factor, (TT_MULT, TT_DIV, TT_FDIV, TT_MOD))

    def comp_expr(self):
        res = ParseRes()

        if self.current_tok.matches(TT_KEYWORD, "not"):
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
                "Expected int, float, identifier, '+', '-', '(', or 'not'"
            ))

        return res.success(node)

    def list_expr(self):
        res = ParseRes()
        element_nodes = []
        pos_start = self.current_tok.pos_start.copy()

        if not self.current_tok.type == TT_LPAREN_SQUARE:
            return res.failure(InvalidSyntaxError(self.current_tok.pos_start, self.current_tok.pos_end,
                                                  "Expected '['"))

        res.register_advancement()
        self.advance()

        if self.current_tok.type == TT_RPAREN_SQUARE:
            res.register_advancement()
            self.advance()
        else:
            element_nodes.append(res.register(self.expr()))
            if res.error:
                return res.failure(InvalidSyntaxError(self.current_tok.pos_start, self.current_tok.pos_end,
                                                      "Expected ']', method , int, float, identifier, "
                                                      "'+', or '-'"))
            while self.current_tok.type == TT_COMMA:
                res.register_advancement()
                self.advance()

                element_nodes.append(res.register(self.expr()))
                if res.error: return res

            if self.current_tok.type != TT_RPAREN_SQUARE:
                print(self.current_tok)
                return res.failure(InvalidSyntaxError(self.current_tok.pos_start, self.current_tok.pos_end,
                                                      "Expected ',' or ']"))

            res.register_advancement()
            self.advance()

        return res.success(ListNode(element_nodes, pos_start, self.current_tok.pos_end.copy()))

    def arith_expr(self):
        return self.bin_op(self.term, (TT_PLUS, TT_MINUS))

    def expr(self):
        res = ParseRes()

        if self.current_tok.matches(TT_KEYWORD, "var"):
            res.register_advancement()
            self.advance()
            if self.current_tok.type != TT_IDENTIFIER:
                return res.failure(InvalidSyntaxError(self.current_tok.pos_start, self.current_tok.pos_end,
                                                      "Expected identifier"))

            var_name = self.current_tok
            res.register_advancement()
            self.advance()

            tok_type = self.current_tok.type

            if tok_type not in (TT_SET, TT_SET_RET):
                return res.failure(InvalidSyntaxError(self.current_tok.pos_start, self.current_tok.pos_end,
                                                      "Expected \'::\' or \'->\'"))

            res.register_advancement()
            self.advance()
            expr = res.register(self.expr())
            if res.error: return res
            return res.success(VarAssignNode(var_name, expr, True if tok_type == TT_SET_RET else False))

        node = res.register(self.bin_op(self.comp_expr, ((TT_KEYWORD, "and"), (TT_KEYWORD, "or"))))

        if res.error:
            return res.failure(InvalidSyntaxError(self.current_tok.pos_start, self.current_tok.pos_end,
                                                  "Expected variable assignment keyword, int, float, identifier, '+',"
                                                  "'-', or '('"))

        return res.success(node)

    def func_def(self):
        res = ParseRes()

        if not self.current_tok.matches(TT_KEYWORD, "def"):
            if self.current_tok.type != TT_IDENTIFIER:
                return res.failure(InvalidSyntaxError(self.current_tok.pos_start, self.current_tok.pos_end,
                                                      "Expected 'def'"))

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

            node_to_return = res.register(self.expr())
            if res.error: return res

            if self.current_tok.type != TT_RPAREN_CURLY:
                return res.failure(InvalidSyntaxError(self.current_tok.pos_start, self.current_tok.pos_end,
                                                      "Expected '}'"))

            res.register_advancement()
            self.advance()

            return res.success(FuncDefNode(var_name_tok, arg_name_toks, node_to_return))

    def bin_op(self, func_a, ops, func_b=None):
        if func_b is None:
            func_b = func_a
        res = ParseRes()
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
