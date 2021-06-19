from Errors import *
from ErrorsNew import *
from Bases import wrap_length, Token, TT_AT, TokenPosition


class NumberNode:
    def __init__(self, tok):
        self.tok = tok

        self.pos_start = self.tok.pos_start
        self.pos_end = self.tok.pos_end

    def __repr__(self):
        return f'{self.tok}'


class autoNode:
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
        self.name_tok = var_name_tok

        self.pos_start = self.name_tok.pos_start
        self.pos_end = self.name_tok.pos_end


class VarGetRangeNode:
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
    def __init__(self, default_value, var_name_tok, value_node, ret, pos_start, pos_end):
        self.default_value = default_value
        self.var_name_tok = var_name_tok
        self.value_node = value_node
        self.ret = ret
        self.pos_start = pos_start
        self.pos_end = pos_end


class AutoVarAssignNode(VarAssignNode):
    def __init__(self, default_value, var_name_tok, value_node, ret, pos_start, pos_end):
        super().__init__(default_value, var_name_tok, value_node, ret, pos_start, pos_end)


class VarReassignNode:
    def __init__(self, var_name_tok, value_node, ret, token):
        self.var_name_tok = var_name_tok
        self.value_node = value_node
        self.ret = ret
        self.token = token
        self.pos_start = self.var_name_tok.pos_start
        self.pos_end = self.value_node.pos_end


class VarSubFuncNode:
    def __init__(self, trace, pos_start, pos_end):
        self.trace = trace

        self.pos_start = pos_start
        self.pos_end = pos_end


class MethodCallNode:
    def __init__(self, method_name, args):
        self.name_tok = method_name
        self.args = args


class TrueFalseNode:
    def __init__(self, condition, if_true, if_false, pos_start, pos_end):
        self.condition = condition
        self.if_true = if_true
        self.if_false = if_false

        self.pos_start = pos_start
        self.pos_end = pos_end


class AsErrorCatchNode:
    def __init__(self, as_node, error_catch_node, pos_start, pos_end):
        self.as_node = as_node
        self.error_catch_node = error_catch_node

        self.pos_start = pos_start
        self.pos_end = pos_end


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


class SilentNode:
    def __init__(self, silenced_node, pos_start):
        self.silenced_node = silenced_node

        self.pos_start = pos_start
        self.pos_end = self.silenced_node.pos_end


class AsNode:
    def __init__(self, expr, as_type, pos_start):
        self.expr = expr
        self.as_type = as_type

        self.pos_start = pos_start
        self.pos_end = self.as_type.pos_end
        self.as_type = self.as_type.value


class IfNode:
    def __init__(self, cases, else_case):
        self.cases = cases
        self.else_case = else_case

        self.pos_start = self.cases[0][0].pos_start
        self.pos_end = (self.else_case or self.cases[len(self.cases) - 1])[0].pos_end


class AtNameNode:
    def __init__(self, token):
        self.at = token.value

        self.pos_start = token.pos_start
        self.pos_end = token.pos_end


class CaseNode:
    def __init__(self, condition, cases, pos_start, pos_end, default=None):
        self.condition = condition
        self.cases = cases
        self.default = default

        self.pos_start = pos_start
        self.pos_end = pos_end

    def new_option(self, option_expr, option_method, option_name: AtNameNode = None, context=None):
        if option_name:
            if option_name.at == 'default':
                ErrorNew(AET_IllegalArgumentValue, 'Cannot give a case statement option the identifier \'@default\'',
                         option_name.pos_start, option_name.pos_start, context)
        self.cases.append(OptionNode(option_expr, option_method, option_expr.pos_start,
                                     option_name.pos_end if option_name else option_method.pos_end, option_name))
        return self

    new_option.args = {'option_expr': None, 'option_method': None}
    new_option.optional_args = {'option_name': None}
    new_option.args_str = {'option_expr<any>': 'Expression to evaluate and check against the expression '
                                               'of the case statement',
                           'option_method<any>': 'Method to run if the option_expr matches the case '
                                                 'statement expression'}
    new_option.optional_args_str = {'option_name<name>': 'Name of the option. Can be used to remove or '
                                                         'reference this option'}

    def new_default(self, default_method, context=None):
        if self.default is not None:
            NonBreakError(default_method.pos_start, default_method.pos_end, context,
                          WT_SilentCaseResetDefault).print_method()
        self.default = OptionNode(None, default_method, default_method.pos_start, default_method.pos_end,
                                  AtNameNode(Token(TT_AT, 'default',
                                                   TokenPosition(0, 0), TokenPosition(0, 0))))

    new_default.args = {'default_method': None}
    new_default.optional_args = {}
    new_default.args_str = {'default_method': 'Method to run if no other options are evaluated in the case statement'}
    new_default.optional_args_str = {}

    def remove(self, identifier, context=None):
        for option in self.cases:
            assert isinstance(option, OptionNode)
            if option.name == identifier:
                del option

    def execute(self):
        return

    execute.args = {}
    execute.optional_args = {}


class OptionNode:
    def __init__(self, option, expr, pos_start, pos_end, name=None):
        self.option = option
        self.expr = expr
        self.name = name

        self.pos_start = pos_start
        self.pos_end = pos_end


class IterateNode:
    def __init__(self, var_name_tok, start_value_node, end_value_node, step_value_node,
                 suite_node, should_return_null, reference_name=None):
        self.var_name_tok = var_name_tok
        self.start_value_node = start_value_node
        self.end_value_node = end_value_node
        self.step_value_node = step_value_node
        self.suite_node = suite_node
        self.should_return_null = should_return_null
        self.reference_name = reference_name

        self.pos_start = (self.start_value_node or self.end_value_node).pos_start
        self.pos_end = self.suite_node.pos_end


class ImportNode:
    def __init__(self, file_name):
        self.file_name = file_name
        self.pos_start = self.file_name.pos_start
        self.pos_end = self.file_name.pos_end


class WhileNode:
    def __init__(self, condition_node, body_node, should_return_null):
        self.condition_node = condition_node
        self.body_node = body_node
        self.should_return_null = should_return_null

        self.pos_start = self.condition_node.pos_start
        self.pos_end = self.body_node.pos_end


class FuncDefNode:
    def __init__(self, var_name_tok, arg_name_toks, body_node, ret_type=None):
        self.var_name_tok = var_name_tok
        self.arg_name_toks = arg_name_toks
        self.body_node = body_node
        self.ret_type = ret_type

        if self.var_name_tok:
            self.pos_start = self.var_name_tok.pos_start
        elif len(self.arg_name_toks) > 0:
            self.pos_start = self.arg_name_toks[0].pos_start
        else:
            self.pos_start = self.body_node.pos_start

        self.pos_end = self.body_node.pos_end


class FuncArgNode:
    def __init__(self, type_, default_value_node=None, explain=None):
        self.type_ = type_
        self.default_value_node = default_value_node

        self.explain = explain


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
    def __init__(self, pos_start, pos_end, continue_to=None):
        self.continue_to = continue_to
        self.pos_start = pos_start
        self.pos_end = pos_end


class BreakNode:
    def __init__(self, pos_start, pos_end, break_to=None):
        self.break_to = break_to
        self.pos_start = pos_start
        self.pos_end = pos_end


def arg_explain(args: dict, method_name: str):
    optional = {key: item for key, item in args.items() if not item.default_value_node}
    required = {key: item for key, item in args.items() if item.default_value_node}

    max_arg_length = max([len(key) + len(item.type_) for key, item in args.items()])
    newline = '\n' + " " * (max_arg_length + 6)

    title = f'Parameters for {method_name} method'
    title = title.rjust((wrap_length - len(title)) // 2 + len(title), '~').ljust(wrap_length, '~')
    title = f'\033[34m{title}\033[0m'
    if len(args) == 0:
        title += '\nMethod does not take any arguments'
        return title
    if len(required) > 0:
        req = '# Required parameters\n'
        for key, item in required.items():
            req += f'{key} ({item.type_})'.ljust(max_arg_length)
            if item.explain:
                req += ' : ' + newline.join(wrap(item.explain, wrap_length - max_arg_length - 6))
            req += '\n'
        title += f'\n{req}'
        del req
    if len(optional) > 0:
        opt = '# Optional parameters\n'
        for key, item in optional.items():
            opt += f'{key} ({item.type_})'.ljust(max_arg_length)
            if item.explain:
                opt += ' : ' + newline.join(wrap(item.explain, wrap_length - max_arg_length - 6))
            opt += '\n'
        title += f'\n{opt}'
        del opt
    return title


SILENCABLE_TYPES = {'case': CaseNode}
