class NumberNode:
    def __init__(self, tok):
        self.tok = tok

        self.pos_start = self.tok.pos_start
        self.pos_end = self.tok.pos_end

    def __repr__(self):
        return f'{self.tok}'


class IntNode:
    def __init__(self, tok):
        self.tok = tok

        self.pos_start = self.tok.pos_start
        self.pos_end = self.tok.pos_end

    def __repr__(self):
        return f'{self.tok}'


class FloatNode:
    def __init__(self, tok):
        self.tok = tok

        self.pos_start = self.tok.pos_start
        self.pos_end = self.tok.pos_end

    def __repr__(self):
        return f'{self.tok}'


class BoolNode:
    def __init__(self, tok):
        self.tok = tok

        self.pos_start = self.tok.pos_start
        self.pos_end = self.tok.pos_end

    def __repr__(self):
        return f'{self.tok}'


class StringNode:
    def __init__(self, tok):
        self.tok = tok

        self.pos_start = self.tok.pos_start
        self.pos_end = self.tok.pos_end

    def __repr__(self):
        return f'{self.tok}'


class ListNode:
    def __init__(self, element_nodes, pos_start, pos_end):
        self.element_nodes = element_nodes

        self.pos_start = pos_start
        self.pos_end = pos_end


class VarAccessNode:
    def __init__(self, var_name_tok):
        self.var_name_tok = var_name_tok

        self.pos_start = self.var_name_tok.pos_start
        self.pos_end = self.var_name_tok.pos_end


class VarGetSetNode:
    def __init__(self, var_name_tok, rangeList):
        self.var_name_tok = var_name_tok
        self.rangeList = rangeList

        self.pos_start = self.var_name_tok.pos_start
        self.pos_end = self.rangeList[-1].pos_end


class VarGetItemNode:
    def __init__(self, lower, higher, range_get, pos_start, pos_end):
        self.lower = lower
        self.higher = higher
        self.range_get = range_get

        self.pos_start = pos_start
        self.pos_end = pos_end


class VarAssignNode:
    def __init__(self, var_type, var_name_tok, value_node, ret):
        self.var_type = var_type
        self.var_name_tok = var_name_tok
        self.value_node = value_node
        self.ret = ret
        self.pos_start = self.var_name_tok.pos_start
        self.pos_end = self.value_node.pos_end if self.value_node else self.var_name_tok.pos_end


class VarReassignNode:
    def __init__(self, var_name_tok, value_node, ret, token):
        self.var_name_tok = var_name_tok
        self.value_node = value_node
        self.ret = ret
        self.token = token
        self.pos_start = self.var_name_tok.pos_start
        self.pos_end = self.value_node.pos_end


class TrueFalseNode:
    def __init__(self, condition, if_true, if_false):
        self.condition = condition
        self.if_true = if_true
        self.if_false = if_false

        self.pos_start = self.condition.pos_start
        self.pos_end = self.if_false.pos_end


class BinOpNode:
    def __init__(self, left_node, op_tok, right_node):
        self.left_node = left_node
        self.op_tok = op_tok
        self.right_node = right_node

        self.pos_start = self.left_node.pos_start
        self.pos_end = self.right_node.pos_end

    def __repr__(self):
        return f'({self.left_node}, {self.op_tok}, {self.right_node})'


class UnaryOpNode:
    def __init__(self, op_tok, node):
        self.op_tok = op_tok
        self.node = node

        self.pos_start = self.op_tok.pos_start
        self.pos_end = node.pos_end

    def __repr__(self):
        return f'({self.op_tok}, {self.node})'


class IfNode:
    def __init__(self, cases, else_case):
        self.cases = cases
        self.else_case = else_case

        self.pos_start = self.cases[0][0].pos_start
        self.pos_end = (self.else_case or self.cases[len(self.cases) - 1])[0].pos_end


class CasesNode:
    def __init__(self, condition, cases, pos_start, pos_end, default=None):
        self.condition = condition
        self.cases = cases
        self.default = default

        self.pos_start = pos_start
        self.pos_end = pos_end


class OptionNode:
    def __init__(self, option, expr, pos_start, pos_end):
        self.option = option
        self.expr = expr

        self.pos_start = pos_start
        self.pos_end = pos_end


class IterateNode:
    def __init__(self, var_name_tok, start_value_node, end_value_node, step_value_node, suite_node, should_return_null):
        self.var_name_tok = var_name_tok
        self.start_value_node = start_value_node
        self.end_value_node = end_value_node
        self.step_value_node = step_value_node
        self.suite_node = suite_node
        self.should_return_null = should_return_null

        self.pos_start = (self.start_value_node or self.end_value_node).pos_start
        self.pos_end = self.suite_node.pos_end


class WhileNode:
    def __init__(self, condition_node, body_node, should_return_null):
        self.condition_node = condition_node
        self.body_node = body_node
        self.should_return_null = should_return_null

        self.pos_start = self.condition_node.pos_start
        self.pos_end = self.body_node.pos_end


class FuncDefNode:
    def __init__(self, var_name_tok, arg_name_toks, body_node):
        self.var_name_tok = var_name_tok
        self.arg_name_toks = arg_name_toks
        self.body_node = body_node

        if self.var_name_tok:
            self.pos_start = self.var_name_tok.pos_start
        elif len(self.arg_name_toks) > 0:
            self.pos_start = self.arg_name_toks[0].pos_start
        else:
            self.pos_start = self.body_node.pos_start

        self.pos_end = self.body_node.pos_end


class CallNode:
    def __init__(self, node_to_call, arg_nodes):
        self.node_to_call = node_to_call
        self.arg_nodes = arg_nodes

        self.pos_start = self.node_to_call.pos_start

        if len(self.arg_nodes) > 0:
            self.pos_end = self.arg_nodes[len(self.arg_nodes) - 1].pos_end
        else:
            self.pos_end = self.node_to_call.pos_end


class ReturnNode:
    def __init__(self, node_to_return, pos_start, pos_end):
        self.node_to_return = node_to_return

        self.pos_start = pos_start
        self.pos_end = pos_end


class ContinueNode:
    def __init__(self, pos_start, pos_end):
        self.pos_start = pos_start
        self.pos_end = pos_end


class BreakNode:
    def __init__(self, pos_start, pos_end):
        self.pos_start = pos_start
        self.pos_end = pos_end
