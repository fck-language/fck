from Bases import *
from Errors import *
from Results import RTResult
import os


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


class Null(Value):
    def __init__(self):
        super().__init__()

    def added_to(self, other):
        if isinstance(other, String):
            return String(self.__repr__() + other.value).set_context(self.context), None
        return other

    def __repr__(self):
        return "<Null value>"


class Infinity(Value):
    def __init__(self, value):
        super().__init__()
        self.mult_zero = value

    def added_to(self, other):
        return self.illegal_operation()

    def subbed_by(self, other):
        return self.illegal_operation()

    def multed_by(self, other):
        if isinstance(other, Number):
            if other.value == 0:
                return Number(self.mult_zero).set_context(self.context), None
            return self
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

    def notted(self):
        return self.illegal_operation()

    def is_true(self):
        return self.illegal_operation()

    def __repr__(self):
        return "<Infinity>"


class Number(Value):
    def __init__(self, value):
        super().__init__()
        self.value = value

    def added_to(self, other):
        if isinstance(other, Number):
            return Number(self.value + other.value).set_context(self.context), None
        return self.illegal_operation(other)

    def subbed_by(self, other):
        if isinstance(other, Number):
            return Number(self.value - other.value).set_context(self.context), None
        return self.illegal_operation(other)

    def multed_by(self, other):
        if isinstance(other, Number):
            return Number(self.value * other.value).set_context(self.context), None
        return self.illegal_operation()

    def dived_by(self, other):
        if isinstance(other, Number):
            if other.value == 0:
                NonBreakError(self.pos_start, other.pos_end, self.context, ET_DivideByZero).print_method()
                return Infinity(self.value).set_context(self.context), None
            return Number(self.value / other.value).set_context(self.context), None
        return self.illegal_operation()

    def fdived_by(self, other):
        if isinstance(other, Number):
            if other.value == 0:
                repr(NonBreakError(self.pos_start, other.pos_end, self.context, ET_DivideByZero))
                return Infinity(self.value).set_context(self.context), None
            return Number(self.value // other.value).set_context(self.context), None
        return self.illegal_operation()

    def modded_by(self, other):
        if isinstance(other, Number):
            if other.value == 0:
                repr(NonBreakError(self.pos_start, other.pos_end, self.context, ET_ModByZero))
                return Number(0).set_context(self.context), None
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
        elif isinstance(other, Null):
            return String(self.value + other.__repr__()).set_context(self.context), None
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
        self.name = name or "<anonymous>"

    def generate_new_context(self):
        new_context = Context(self.name, self.context, self.pos_start)
        new_context.symbol_table = SymbolTable(new_context.parent.symbol_table)
        return new_context

    def check_args(self, arg_names, args):
        res = RTResult()

        if len(args) > len(arg_names):
            return res.failure(RTError(
                self.pos_start, self.pos_end,
                f"{len(args) - len(arg_names)} too many args passed into {self}",
                self.context
            ))

        if len(args) < len(arg_names):
            return res.failure(RTError(
                self.pos_start, self.pos_end,
                f"{len(arg_names) - len(args)} too few args passed into {self}",
                self.context
            ))

        return res.success(None)

    def populate_args(self, arg_names, args, exec_ctx):
        for i in range(len(args)):
            arg_name = arg_names[i]
            arg_value = args[i]
            arg_value.set_context(exec_ctx)
            exec_ctx.symbol_table.set(arg_name, arg_value)

    def check_and_populate_args(self, arg_names, args, exec_ctx):
        res = RTResult()
        res.register(self.check_args(arg_names, args))
        if res.should_return(): return res
        self.populate_args(arg_names, args, exec_ctx)
        return res.success(None)


class Function(BaseFunction):
    def __init__(self, name, body_node, arg_names):
        super().__init__(name)
        self.body_node = body_node
        self.arg_names = arg_names

    def execute(self, args):
        pass

    def copy(self):
        copy = Function(self.name, self.body_node, self.arg_names)
        copy.set_context(self.context)
        copy.set_pos(self.pos_start, self.pos_end)
        return copy

    def __repr__(self):
        return f"<function {self.name}>"


class BuiltInFunction(BaseFunction):
    def __init__(self, name):
        super().__init__(name)

    def execute(self, args):
        res = RTResult()
        exec_ctx = self.generate_new_context()

        method = getattr(self, f'execute_{self.name}', self.no_visit_method)
        res.register(self.check_and_populate_args(method.arg_names, args, exec_ctx))
        if res.should_return(): return res

        return_value = res.register(method(exec_ctx))
        if res.should_return(): return res
        return res.success(return_value)

    def no_visit_method(self, context):
        raise Exception(f'No \'execute_{self.name}\' method defined silly x')

    def execute_log(self, exec_ctx):
        print(repr(exec_ctx.symbol_table.get("value")))
        return RTResult().success(None)

    execute_log.arg_names = ['value']

    def execute_print(self, exec_ctx):
        value = exec_ctx.symbol_table.get("value")
        print(str(value)) if not isinstance(value, Null) else None
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

    def execute_run(self, exec_ctx):
        pass

    def copy(self):
        copy = BuiltInFunction(self.name)
        copy.set_context(self.context).set_pos(self.pos_start, self.pos_end)
        return copy

    def __repr__(self):
        return f'<built-in function {self.name}>'