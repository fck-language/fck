from Nodes import *
from Errors import *
import os


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
        raise Exception(f'No visit method defined for {type(node).__name__}')

    def visit_NumberNode(self, node, context):
        return RTResult().success(Number(node.tok.value).set_context(context).set_pos(node.pos_start, node.pos_end))

    def visit_StringNode(self, node, context):
        return RTResult().success(String(node.tok.value).set_context(context).set_pos(node.pos_start, node.pos_end))

    def visit_ListNode(self, node, context):
        res = RTResult()
        elements = []

        for element_node in node.element_nodes:
            elements.append(res.register(self.visit(element_node, context)))
            if res.error: return res

        return res.success(List(elements).set_context(context).set_pos(node.pos_start, node.pos_end))

    def visit_VarAccessNode(self, node, context):
        res = RTResult()
        var_name = node.var_name_tok.value
        value = context.symbol_table.get(var_name)

        if not value:
            return res.failure(RTError(node.pos_start, node.pos_end, f'\'{var_name}\' is not defined. Duuuuh', context))

        value = value.copy().set_pos(node.pos_start, node.pos_end).set_context(context)
        return res.success(value)

    def visit_VarAssignNode(self, node, context):
        res = RTResult()
        var_name = node.var_name_tok.value
        value = res.register(self.visit(node.value_node, context))
        if res.error: return res

        context.symbol_table.set(var_name, value)
        if node.ret:
            return res.success(value)
        return res.success(None)

    def visit_BinOpNode(self, node, context):
        res = RTResult()
        left = res.register(self.visit(node.left_node, context))
        if res.error: return res
        right = res.register(self.visit(node.right_node, context))
        if res.error: return res

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
                    break

        if error:
            return res.failure(error)
        return res.success(result.set_pos(node.pos_start, node.pos_end))

    def visit_UnaryOpNode(self, node, context):
        res = RTResult()
        number = res.register(self.visit(node.node, context))
        if res.error: return res

        if node.op_tok.type == TT_MINUS:
            number, error = number.multed_by(Number(-1))
        elif node.op_tok.matches(TT_KEYWORD, 'not'):
            number, error = number.notted()

        if error:
            return res.failure(error)
        return res.success(number.set_pos(node.pos_start, node.pos_end))

    def visit_IfNode(self, node, context):
        res = RTResult()

        for condition, expr, should_return_null in node.cases:
            condition_value = res.register(self.visit(condition, context))
            if res.error: return res

            if condition_value.is_true():
                expr_value = res.register(self.visit(expr, context))
                if res.error: return res
                return res.success(None if should_return_null else expr_value)

        if node.else_case:
            expr, should_return_null = node.else_case
            else_value = res.register(self.visit(expr, context))
            if res.error: return res
            return res.success(None if should_return_null else else_value)

        return res.success(None)

    def visit_ForNode(self, node, context):
        res = RTResult()
        elements = []
        should_return_null = node.should_return_null

        if node.start_value_node:
            start_value = res.register(self.visit(node.start_value_node, context))
            if res.error: return res
        else:
            start_value = Number(0)

        end_value = res.register(self.visit(node.end_value_node, context))
        if res.error: return res

        if node.step_value_node:
            step_value = res.register(self.visit(node.step_value_node, context))
            if res.error: return res
        else:
            step_value = Number(1)

        i = start_value.value

        if step_value.value > 0:
            if start_value.value > end_value.value:
                return res.failure(RTError(node.pos_start, node.pos_end, f'Step value for iteration would never end. '
                                                                         f'Just do a while true', context))
            condition = lambda: i < end_value.value
        elif step_value.value < 0:
            if start_value.value < end_value.value:
                return res.failure(RTError(node.pos_start, node.pos_end, f'Step value for iteration would never end. '
                                                                         f'Just do a while true', context))
            condition = lambda: i > end_value.value
        else:
            return res.failure(RTError(node.pos_start, node.pos_end, f'Step value for iterate method cannot be 0... '
                                                                     f'come on really?', context))

        while condition():
            if node.var_name_tok:
                context.symbol_table.set(node.var_name_tok.value, Number(i))

            elements.append(res.register(self.visit(node.suite_node, context)))
            if res.error: return res

            if node.var_name_tok:
                i = context.symbol_table.get(node.var_name_tok.value).value
            i += step_value.value

        if node.var_name_tok:
            context.symbol_table.remove(node.var_name_tok.value)

        return res.success(None if should_return_null else
                           List(elements).set_context(context).set_pos(node.pos_start, node.pos_end))

    def visit_WhileNode(self, node: WhileNode, context):
        res = RTResult()

        while True:
            condition = res.register(self.visit(node.condition_node, context))
            if res.error: return res

            if not condition.is_true(): break

            res.register(self.visit(node.suite_node, context))
            if res.error: return res

        return res.success(None)

    def visit_FuncDefNode(self, node, context):
        res = RTResult()

        func_name = node.var_name_tok.value if node.var_name_tok else None
        suite_node = node.suite_node
        arg_names = [arg_name.value for arg_name in node.arg_name_toks]
        func_value = Function(func_name, suite_node, arg_names, node.should_return_null).set_context(context) \
            .set_pos(node.pos_start, node.pos_end)

        if node.var_name_tok:
            context.symbol_table.set(func_name, func_value)

        return res.success(func_value)

    def visit_CallNode(self, node: CallNode, context):
        res = RTResult()
        args = []

        value_to_call = res.register(self.visit(node.node_to_call, context))
        if res.error: return res
        value_to_call = value_to_call.copy().set_pos(node.pos_start, node.pos_end)

        for arg_node in node.arg_nodes:
            args.append(res.register(self.visit(arg_node, context)))
            if res.error: return res

        return_value = res.register(value_to_call.execute(args))
        if res.error: return res
        if return_value:
            return_value = return_value.copy().set_pos(node.pos_start, node.pos_end).set_context(context)
        return res.success(return_value)


class Value:
    def __init__(self):
        self.pos_start = None
        self.pos_end = None
        self.context = None

    def set_pos(self, pos_start=None, pos_end=None):
        self.pos_start = pos_start
        self.pos_end = pos_end
        return self

    def set_context(self, context=None):
        self.context = context
        return self

    def added_to(self, other):
        return self.illegal_operation()

    def subbed_by(self, other):
        return self.illegal_operation()

    def multed_by(self, other):
        return self.illegal_operation()

    def dived_by(self, other):
        return self.illegal_operation()

    def fdived_by(self, other):
        return self.illegal_operation()

    def modded_by(self, other):
        return self.illegal_operation()

    def powed_by(self, other):
        return self.illegal_operation()

    def get_comparison_lt(self, other):
        return self.illegal_operation()

    def get_comparison_gt(self, other):
        return self.illegal_operation()

    def get_comparison_lte(self, other):
        return self.illegal_operation()

    def get_comparison_gte(self, other):
        return self.illegal_operation()

    def get_comparison_eq(self, other):
        return self.illegal_operation()

    def get_comparison_ne(self, other):
        return self.illegal_operation()

    def anded_by(self, other):
        return self.illegal_operation()

    def ored_by(self, other):
        return self.illegal_operation()

    def execute(self, args):
        return self.illegal_operation()

    def notted(self):
        return self.illegal_operation()

    def is_true(self):
        return self.illegal_operation()

    def copy(self):
        copy = Number(self.value)
        copy.set_pos(self.pos_start, self.pos_end)
        copy.set_context(self.context)

        return copy

    def illegal_operation(self, other=None):
        other = self if not other else other
        return None, RTError(self.pos_start, other.pos_end, "Illegal operation. No can do sorry", self.context)

    def __repr__(self):
        return f'{self.type}: {self.value}'


class Number(Value):
    def __init__(self, value):
        super().__init__()
        self.value = value

    def added_to(self, other):
        if isinstance(other, Number):
            return Number(self.value + other.value).set_context(self.context), None
        return None, IllegalOperationError(self.pos_start, self.pos_end)

    def subbed_by(self, other):
        if isinstance(other, Number):
            return Number(self.value - other.value).set_context(self.context), None
        return self.illegal_operation()

    def multed_by(self, other):
        if isinstance(other, Number):
            return Number(self.value * other.value).set_context(self.context), None
        return self.illegal_operation()

    def dived_by(self, other):
        if isinstance(other, Number):
            if other.value == 0:
                return None, RTError(other.pos_start, other.pos_end, "Division by zero. No", self.context)
            return Number(self.value / other.value).set_context(self.context), None
        return self.illegal_operation()

    def fdived_by(self, other):
        if isinstance(other, Number):
            if other.value == 0:
                return None, RTError(other.pos_start, other.pos_end, "Division by zero. No", self.context)
            return Number(self.value // other.value).set_context(self.context), None
        return self.illegal_operation()

    def modded_by(self, other):
        if isinstance(other, Number):
            if other.value == 0:
                return None, RTError(other.pos_start, other.pos_end, "Division by zero. No", self.context)
            return Number(self.value % other.value).set_context(self.context), None
        return self.illegal_operation()

    def powed_by(self, other):
        if isinstance(other, Number):
            return Number(self.value ** other.value).set_context(self.context), None
        return self.illegal_operation()

    def get_comparison_lt(self, other):
        if isinstance(other, Number):
            return Number(int(self.value < other.value)).set_context(self.context), None
        return self.illegal_operation()

    def get_comparison_gt(self, other):
        if isinstance(other, Number):
            return Number(int(self.value > other.value)).set_context(self.context), None
        return self.illegal_operation()

    def get_comparison_lte(self, other):
        if isinstance(other, Number):
            return Number(int(self.value <= other.value)).set_context(self.context), None
        return self.illegal_operation()

    def get_comparison_gte(self, other):
        if isinstance(other, Number):
            return Number(int(self.value >= other.value)).set_context(self.context), None
        return self.illegal_operation()

    def get_comparison_eq(self, other):
        if isinstance(other, Number):
            return Number(int(self.value == other.value)).set_context(self.context), None
        return self.illegal_operation()

    def get_comparison_ne(self, other):
        if isinstance(other, Number):
            return Number(int(self.value != other.value)).set_context(self.context), None
        return self.illegal_operation()

    def anded_by(self, other):
        if isinstance(other, Number):
            return Number(int(self.value and other.value)).set_context(self.context), None
        return self.illegal_operation()

    def ored_by(self, other):
        if isinstance(other, Number):
            return Number(int(self.value or other.value)).set_context(self.context), None
        return self.illegal_operation()

    def notted(self):
        return Number(1 if self.value <= 0 else 0).set_context(self.context), None

    def is_true(self):
        return self.value > 0

    def __repr__(self):
        return str(self.value)


class String(Value):
    def __init__(self, value):
        super().__init__()
        self.value = value

    def added_to(self, other):
        if isinstance(other, String):
            return String(self.value + other.value).set_context(self.context), None
        elif isinstance(other, Number):
            return String(self.value + str(other.value)).set_context(self.context), None
        return self.illegal_operation()

    def multed_by(self, other):
        if isinstance(other, Number):
            return String(self.value * other.value).set_context(self.context), None
        return self.illegal_operation()

    def is_true(self):
        return len(self.value) > 0, None

    def copy(self):
        copy = String(self.value)
        copy.set_context(self.context).set_pos(self.pos_start, self.pos_end)
        return copy

    def __str__(self):
        return f'{self.value}'

    def __repr__(self):
        return f'\"{self.value}\"'


class List(Value):
    def __init__(self, elements):
        super().__init__()
        self.elements = elements

    def added_to(self, other):
        if isinstance(other, List):
            self.elements.extend(other.elements)
            return self, None
        elif isinstance(other, Number):
            self.elements = [i.value + other.value for i in self.elements]
            return self, None
        else:
            return self.illegal_operation()

    def multed_by(self, other):
        if isinstance(other, Number):
            self.elements = [i.value * other.value for i in self.elements]
            return self, None
        else:
            return self.illegal_operation()

    def copy(self):
        copy = List(self.elements)
        copy.set_context(self.context).set_pos(self.pos_start, self.pos_end)
        return copy

    def __str__(self):
        return ", ".join([str(i) for i in self.elements])

    def __repr__(self):
        return f'[{", ".join([str(i) for i in self.elements])}]'


class BaseFunction(Value):
    def __init__(self, name):
        super().__init__()
        self.name = name or "<no one knows(anonymous)>"

    def generate_new_context(self):
        new_context = Context(self.name, self.context, self.pos_start)
        new_context.symbol_table = SymbolTable(new_context.parent.symbol_table)
        return new_context

    def check_args(self, arg_names, args):
        res = RTResult()

        if len(args) > len(arg_names):
            return res.failure(RTError(self.pos_start, self.pos_end, f"{len(args) - len(arg_names)} too many args "
                                                                     f"passed into '{self.name}'", self.context))

        if len(args) < len(arg_names):
            return res.failure(RTError(self.pos_start, self.pos_end, f"{len(arg_names) - len(args)} too few args "
                                                                     f"passed into '{self.name}'", self.context))

        return res.success(None)

    def populate_arge(self, arg_names, args, exec_ctx):
        for i in range(len(args)):
            arg_name = arg_names[i]
            arg_value = args[i]
            arg_value.set_context(exec_ctx)
            exec_ctx.symbol_table.set(arg_name, arg_value)

    def check_and_populate_args(self, arg_names, args, exec_ctx):
        res = RTResult()
        res.register(self.check_args(arg_names, args))
        if res.error: return res
        self.populate_arge(arg_names, args, exec_ctx)
        return res.success(None)


class Function(BaseFunction):
    def __init__(self, name, suite_node, arg_names, should_return_null):
        super().__init__(name)
        self.suite_node = suite_node
        self.arg_names = arg_names
        self.should_return_null = should_return_null

    def execute(self, args):
        res = RTResult()
        interpreter = Interpreter()
        exec_ctx = self.generate_new_context()

        self.check_and_populate_args(self.arg_names, args, exec_ctx)

        value = res.register(interpreter.visit(self.suite_node, exec_ctx))
        if res.error: return res
        return res.success(None if self.should_return_null else value)

    def copy(self):
        copy = Function(self.name, self.suite_node, self.arg_names, self.should_return_null)
        copy.set_context(self.context)
        copy.set_pos(self.pos_start, self.pos_end)
        return copy

    def __repr__(self):
        return f'<function {self.name}>'


class BuiltInFunction(BaseFunction):
    def __init__(self, name):
        super().__init__(name)

    def execute(self, args):
        res = RTResult()
        exec_ctx = self.generate_new_context()

        method = getattr(self, f'execute_{self.name}', self.no_visit_method)
        res.register(self.check_and_populate_args(method.arg_names, args, exec_ctx))
        if res.error: return res

        return_value = res.register(method(exec_ctx))
        if res.error: return res
        return res.success(return_value)

    def no_visit_method(self, context):
        raise Exception(f'No \'execute_{self.name}\' method defined silly x')

    def execute_log(self, exec_ctx):
        print(repr(exec_ctx.symbol_table.get("value")))
        return RTResult().success(None)

    execute_log.arg_names = ['value']

    def execute_print(self, exec_ctx):
        print(str(exec_ctx.symbol_table.get("value")))
        return RTResult().success(None)

    execute_print.arg_names = ['value']

    def execute_input(self, exec_ctx):
        text = input()
        return RTResult().success((String(text)))

    execute_input.arg_names = []

    def execute_clear(self, exec_ctx):
        os.system('cls' if os.name == 'nt' else 'clear')
        return RTResult().success(None)

    execute_clear.arg_names = []

    def copy(self):
        copy = BuiltInFunction(self.name)
        copy.set_context(self.context).set_pos(self.pos_start, self.pos_end)
        return copy

    def __repr__(self):
        return f'<built-in function {self.name}>'


class RTResult:
    def __init__(self):
        self.value = None
        self.error = None

    def register(self, res):
        if res.error: self.error = res.error
        return res.value

    def success(self, value):
        self.value = value
        return self

    def failure(self, error):
        self.error = error
        return self
